use std::borrow::Cow;
use std::cmp;
use std::cell::RefCell;
use std::rc::Rc;
use bufchr::Bufchr3;
use bytecount;
use std::fmt;
use unicode_bom::Bom;
use log::{info, trace, warn};


fn get_col_sep(readed_byte:&[u8]) -> u8 {
	let comma = bytecount::count(readed_byte, b',');
	let tab = bytecount::count(readed_byte, b'\t');
	let pipe = bytecount::count(readed_byte, b'|');
	if comma > tab && comma > pipe {
		return b',';
	}
	if tab > pipe {
		return b'\t';
	}
	b'|'
}

fn get_row_sep(readed_byte:&[u8]) -> u8 {
	// let cr_lf = bytecount::count(readed_byte, 0x0D0A);
	// \r
	let cr = bytecount::count(readed_byte, 0x0D);
	// \n
	let lf = bytecount::count(readed_byte, 0x0A);
	if cr >= lf {
		return 0x0D;
	}
	0x0A
}

// fn get_col<'a>(v: &'a [u8]) -> Cow<'a, str> {
// 	let col = unsafe {
// 		std::str::from_utf8_unchecked(v)
// 	};
// 	Cow::Borrowed(col)
// }

#[derive(Debug, Copy, Clone)]
pub enum FieldResult {
    Field,
	FieldEnd,
    End,

}
impl fmt::Display for FieldResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
		FieldResult::Field => write!(f, "FieldResult::Field"),
		FieldResult::FieldEnd => write!(f, "FieldResult::FieldEnd"),
		FieldResult::End => write!(f, "FieldResult::End"),
       }
    }
}


// row_sep => row seperator start character
// if \r\n => just \r
// Box<std::io::BufReader<R>>
pub struct Core<'bufchr, 'csv: 'bufchr> {
	buffer: Rc<&'csv [u8]>,
    col_sep: u8,
	row_sep: u8,
	last_field_result: FieldResult,
	pos: usize,
	ref_sep_iter: RefCell<Option<Bufchr3<'bufchr>>>,
	start_pos: usize, start_quote: usize, end_quote: usize,
	// double double quote
	in_dd_quote: bool,
}

#[derive(Debug, Default)]
pub struct CoreBuilder{
    col_sep: u8,
	row_sep: u8,
}

impl CoreBuilder{
    pub fn new() -> Self {
        Self {
            col_sep: b'\0',
			row_sep: b'\0',
        }
    }

    pub fn col_sep(mut self, col_sep: u8) -> Self{
        self.col_sep = col_sep;
        self
    }

	pub fn row_sep(mut self, row_sep: u8) -> Self{
        self.row_sep = row_sep;
        self
    }

	pub fn from_buffer<'bufchr, 'csv: 'bufchr>(&self, buffer:&'csv [u8]) -> Core<'bufchr, 'csv> {
        Core::new(self, &buffer)
    }
}

impl<'bufchr, 'csv: 'bufchr> Core<'bufchr, 'csv>{

	fn get_col(&mut self) -> Cow<'csv, str> {
		let v = &self.buffer[self.start_pos + self.start_quote..self.pos - self.end_quote];
		let col = unsafe {
			std::str::from_utf8_unchecked(v)
		};
		Cow::Borrowed(col)
	}

	fn new(builder: &CoreBuilder, buffer:&'csv [u8]) -> Core<'bufchr, 'csv> {
		// BOM Check
		let bom: Bom = Bom::from(&buffer[0..4]);
		if Bom::Utf8 == bom {
			println!("Utf8");
		}

		let det_buf_len = cmp::min(buffer.len(), 1024*4);

		let col_sep;
		if builder.col_sep == b'\0'{	col_sep = get_col_sep(&buffer[0..det_buf_len]);	}
		else{	col_sep = builder.col_sep;	}
		let row_sep;
		if builder.row_sep == b'\0'{	row_sep = get_row_sep(&buffer[0..det_buf_len]);	}
		else{	row_sep = builder.row_sep;	}
		Core{
			buffer: Rc::new(buffer), col_sep, row_sep,
			last_field_result: FieldResult::Field, pos:0, 
			ref_sep_iter: RefCell::new(None), 
			start_pos: 0, start_quote: 0, end_quote: 0, in_dd_quote: false,
		}

	}

	pub fn next(&mut self) -> (FieldResult, Cow<'csv, str>){
		let buffer= self.buffer.clone();
		
		let mut quote_on = false;
		self.start_quote = 0;
		self.end_quote = 0;
		self.start_pos = self.pos;
		self.in_dd_quote = false;
		
		loop{
			let next_sep_pos_wrap = {
				if self.ref_sep_iter.borrow().is_none(){
					self.ref_sep_iter = RefCell::new(Some(Bufchr3::new(
					&buffer, self.col_sep, self.row_sep, b'\"')));
				}
			 	self.ref_sep_iter.borrow_mut().as_mut().unwrap().next()
			};

			if let Some(next_sep_pos) = next_sep_pos_wrap {
				self.pos = next_sep_pos;
			} else if matches!(self.last_field_result, FieldResult::FieldEnd){
				if buffer.len() == self.pos{
					return (FieldResult::End, Cow::Borrowed(""))
				}
				self.pos = buffer.len();
				let col = self.get_col();
				return (FieldResult::FieldEnd, col)
			} else {
				self.pos = buffer.len();
				let col = self.get_col();
				println!("{}", col);
				self.last_field_result = FieldResult::FieldEnd;
				return (FieldResult::FieldEnd, col)
			}
			let ch = buffer[self.pos];
			let mut next_ch = '\0' as u8;
			if self.pos + 1 < buffer.len(){
				next_ch = buffer[self.pos+1];
			}
			
			if ch == b'"'{
				// check start_quote
				if self.start_pos == self.pos{
					self.start_quote = 1;
					quote_on = true;
					continue
				}
				// check last_quote
				else if !quote_on && (next_ch == self.col_sep || next_ch == self.row_sep){
					self.end_quote = 1;
					continue
				}
				else if quote_on && (next_ch == self.col_sep || next_ch == self.row_sep){
					quote_on = !quote_on;
					self.end_quote = 1;
					continue
				}
				else if next_ch == b'"'{  // Double Double Quotes("") check
					// skip next Quote
					let next_sep_pos = self.ref_sep_iter.borrow_mut().as_mut().unwrap().next().unwrap();
					self.pos = next_sep_pos;
					self.in_dd_quote = true;
					continue
				}
				quote_on = !quote_on;
				continue
			}
			else if quote_on{
				continue
			}
			else if ch == self.col_sep {
				let col = self.get_col();
				self.pos += 1;
				self.last_field_result = FieldResult::Field;
				info!("col: {}", col);
				return (FieldResult::Field, col)
			}
			else if ch == (self.row_sep as u8) {
				let col = self.get_col();
				self.last_field_result = FieldResult::FieldEnd;

				// buffer overflow check
				if buffer.len() == self.pos + 1 {
					self.pos += 1;
					info!("col: {}", col);
					return (FieldResult::FieldEnd, col)
				}
				let next_ch = buffer[self.pos+1];
				if next_ch == 0x0A {  // check cr_lf
					self.pos += 2;
				}
				else{
					self.pos += 1;
				}
				info!("col: {}", col);
				return (FieldResult::FieldEnd, col)
			}
			else if ch == 0x00 {
				let col = self.get_col();
				info!("col: {}", col);
				self.last_field_result = FieldResult::End;
				return (FieldResult::End, col)
			}
			self.end_quote = 0;
		}
	}

	pub fn skip(&mut self, skip_len: usize){
		for _ in 0..skip_len{
			self.next();
		}
	}

	pub fn position_estimate(&self) -> usize{
		self.pos
	}

}


