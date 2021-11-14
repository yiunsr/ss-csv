use std::error::Error;
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
pub struct CSV<'bufchr3, R: io::Read> {
	rdr: io::BufReader<R>,
    col_sep: u8,
	row_sep: u8,
	capacity: usize,
	last_field_result: FieldResult,
	pos: usize,
	encoding: &'bufchr3 encoding_rs::Encoding,
	ref_sep_iter: Option<Bufchr3<'bufchr3>>,
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

	pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<CSV< File>, Box<dyn Error>> {
		let f = File::open(path)?;
        Ok(CSV::new(self, io::BufReader::with_capacity(self.capacity, f)))
    }

	pub fn from_read<'bufchr3, 'R:'bufchr3, R: 'R + io::Read>(&self, rdr: R) -> CSV<'bufchr3, R> {
		CSV::new(self, io::BufReader::with_capacity(self.capacity, rdr))
    }

	pub fn from_bufread<'bufchr3, 'R:'bufchr3, R: 'R + io::Read>(&self, rdr: io::BufReader<R>) -> CSV<'bufchr3, R> {
        CSV::new(self, rdr)
    }
}


// impl<'bufchr3, 'R:'bufchr3, R: io::Read> CSV<'bufchr3, 'R, R>{
	impl<'bufchr3, 'R: 'bufchr3, R: 'R+io::Read> CSV<'bufchr3, R>{

	fn new(builder: &CSVBuilder, mut rdr: io::BufReader<R>) -> CSV<'bufchr3, R> {
		let buffer= rdr.fill_buf().unwrap();

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
			rdr, col_sep, row_sep,
			last_field_result: FieldResult::Field, pos:0, 
			encoding, ref_sep_iter: None,
		}
	}

	fn buffer_next(mut self) -> Option<usize> {
		let pos: Option<usize>;
		if self.ref_sep_iter.is_none(){
			let buffer = self.rdr.fill_buf().unwrap();
			self.ref_sep_iter = Some(Bufchr3::new(
				&buffer, self.col_sep, self.row_sep, b'\"'));
			//pos = self.ref_sep_iter.as_deref().unwrap_or(&0);
		}
		pos = self.ref_sep_iter.as_mut().unwrap().next();
		pos
	}

	pub fn next(&'bufchr3 mut self) -> (FieldResult, &'bufchr3 str){
		let buffer = self.rdr.fill_buf().unwrap();
		
		let mut quete_on = false;
		let start_pos = self.pos;
		loop{
			let next_sep_pos_wrap = self.buffer_next(); 
			if let Some(next_sep_pos) = next_sep_pos_wrap {
				self.pos = next_sep_pos;
			} else if matches!(self.last_field_result, FieldResult::FieldEnd){
				return (FieldResult::End, "")
			} else {
				let col = unsafe {
					std::str::from_utf8_unchecked(&buffer[start_pos..])
				};
				println!("{}", col);
				self.last_field_result = FieldResult::FieldEnd;
				return (FieldResult::FieldEnd, col)
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
				return (FieldResult::Field, col)
			}
			else if ch == (self.row_sep as u8) {
				self.last_field_result = FieldResult::FieldEnd;

				// buffer overflow check
				if buffer.len() == self.pos + 1 {
					self.pos += 1;
					return (FieldResult::FieldEnd, col)
				}
				let next_ch = buffer[self.pos+1];
				if next_ch == 0x0A {  // check cr_lf
					self.pos += 2;
				}
				else{
					self.pos += 1;
				}
				return (FieldResult::FieldEnd, col)
			}
			else if ch == 0x00 {
				self.last_field_result = FieldResult::End;
				return (FieldResult::End, col)
			}
		}
	}

}
