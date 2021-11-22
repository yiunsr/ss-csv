use std::borrow::Cow;
use std::error::Error;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::BufRead;
use std::io;
use std::fs::File;
use std::path::Path;
use bufchr::Bufchr3;
use bytecount;
use std::fmt;
use encoding_rs;
use chardetng::EncodingDetector;
use unicode_bom::Bom;

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
pub struct CSV<'bufchr, 'csv: 'bufchr> {
	buffer: Rc<&'csv [u8]>,
    col_sep: u8,
	row_sep: u8,
	capacity: usize,
	last_field_result: FieldResult,
	pos: usize,
	encoding: &'csv encoding_rs::Encoding,
	ref_sep_iter: RefCell<Option<Bufchr3<'bufchr>>>,
}

#[derive(Debug, Default)]
pub struct CSVBuilder{
    col_sep: u8,
	row_sep: u8,
	capacity: usize,
}

impl CSVBuilder{
    pub fn new() -> Self {
        Self {
            col_sep: b'\0',
			row_sep: b'\0',
			capacity: 1024,
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

	pub fn capacity(mut self, capacity: usize) -> Self{
        self.capacity = capacity;
        self
    }

	// pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<CSV< File>, Box<dyn Error>> {
	// 	let f = File::open(path)?;
    //     Ok(CSV::new(self, io::BufReader::with_capacity(self.capacity, f)))
    // }

	// pub fn from_read<'bufchr, 'csv: 'bufchr , R: 'static + io::Read>(&self, rdr: R) -> CSV<'bufchr, 'csv, R> {
	// 	CSV::new(self, io::BufReader::with_capacity(self.capacity, rdr))
    // }

	// pub fn from_bufread<'bufchr, 'csv: 'bufchr,  R: 'static + io::Read>(&self, rdr:io::BufReader<R>) -> CSV<'bufchr, 'csv, R> {
    //     CSV::new(self, rdr)
    // }

	pub fn from_buffer<'bufchr, 'csv: 'bufchr>(&self, buffer:&'csv [u8]) -> CSV<'bufchr, 'csv> {
        CSV::new(self, &buffer)
    }
}



impl<'bufchr, 'csv: 'bufchr> CSV<'bufchr, 'csv>{

	fn new(builder: &CSVBuilder, buffer:&'csv [u8]) -> CSV<'bufchr, 'csv> {
		// BOM Check
		let bom: Bom = Bom::from(&buffer[0..4]);
		if Bom::Utf8 == bom {
			println!("Utf8");
		}

		let mut det = EncodingDetector::new();
		det.feed(&buffer, true);
		let encoding = det.guess(None, false);

		let col_sep;
		if builder.col_sep == b'\0'{	col_sep = get_col_sep(buffer);	}
		else{	col_sep = builder.col_sep;	}
		let row_sep;
		if builder.row_sep == b'\0'{	row_sep = get_row_sep(buffer);	}
		else{	row_sep = builder.row_sep;	}
		CSV{
			buffer: Rc::new(buffer), col_sep, row_sep, capacity: builder.capacity,
			last_field_result: FieldResult::Field, pos:0, 
			encoding, ref_sep_iter: RefCell::new(None),
		}
	}

	// fn buffer_next(&'csv mut self) -> Option<usize> {
	// 	if self.ref_sep_iter.borrow().is_none(){
	// 		let buffer= self.rdr.fill_buf().unwrap();
	// 		self.ref_sep_iter = RefCell::new(Some(Bufchr3::new(
	// 			buffer, self.col_sep, self.row_sep, b'\"')));
	// 	}
	// 	 self.ref_sep_iter.borrow_mut().as_mut().unwrap().next()
	// }

	pub fn next(&mut self) -> (FieldResult, Cow<'csv, str>){
		let buffer= self.buffer.clone();
		
		let mut quete_on = false;
		let start_pos = self.pos;
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
				return (FieldResult::End, Cow::Borrowed(""))
			} else {
				let col = unsafe {
					std::str::from_utf8_unchecked(&buffer[start_pos..])
				};
				println!("{}", col);
				self.last_field_result = FieldResult::FieldEnd;
				return (FieldResult::FieldEnd, Cow::Borrowed(col))
			}
			let ch = buffer[self.pos];
			if quete_on{
				quete_on = !quete_on;
				continue
			}
			else if ch == b'"' {
				quete_on = !quete_on;
				continue
			}

			let col = unsafe {
				std::str::from_utf8_unchecked(&buffer[start_pos..self.pos])
			};
			if ch == self.col_sep {
				self.pos += 1;
				self.last_field_result = FieldResult::Field;
				println!("{}", col);
				return (FieldResult::Field, Cow::Borrowed(col))
			}
			else if ch == (self.row_sep as u8) {
				self.last_field_result = FieldResult::FieldEnd;

				// buffer overflow check
				if buffer.len() == self.pos + 1 {
					self.pos += 1;
					return (FieldResult::FieldEnd, Cow::Borrowed(col))
				}
				let next_ch = buffer[self.pos+1];
				if next_ch == 0x0A {  // check cr_lf
					self.pos += 2;
				}
				else{
					self.pos += 1;
				}
				return (FieldResult::FieldEnd, Cow::Borrowed(col))
			}
			else if ch == 0x00 {
				self.last_field_result = FieldResult::End;
				return (FieldResult::End, Cow::Borrowed(col))
			}
		}
	}

}
