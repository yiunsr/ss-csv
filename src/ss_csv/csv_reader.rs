use std::io::BufRead;
use std::io;
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
pub struct CSV<'a, R: io::Read> {
	rdr: io::BufReader<R>,
    col_sep: u8,
	row_sep: u8,
	last_field_result: FieldResult,
	pos: usize,
	encoding: &'a encoding_rs::Encoding,
	sep_iter: Option<Bufchr3<'a>>,
}

#[derive(Debug, Default)]
pub struct CSVBuilder<R: io::Read>{
    col_sep: u8,
	row_sep: u8,
	rdr: Option<io::BufReader<R>>,
}

impl<R: io::Read> CSVBuilder<R>{
    pub fn new() -> Self {
        Self {
            col_sep: b'\0',
			row_sep: b'\0',
			rdr: None,
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

	pub fn rdr(mut self, rdr:R) -> Self{
		self.rdr = Some(io::BufReader::with_capacity(1024*1024, rdr));
		self
	}

    pub fn build<'a>(self) -> CSV<'a, R> {
		let mut rdr = self.rdr.unwrap();
		let buffer = rdr.fill_buf().unwrap();

		// BOM Check
		let bom: Bom = Bom::from(&buffer[0..4]);
		if Bom::Utf8 == bom {
			println!("Utf8");
		}

		let mut det = EncodingDetector::new();
		det.feed(&buffer, true);
		let encoding = det.guess(None, false);

		let col_sep;
		if self.col_sep == b'\0'{	col_sep = get_col_sep(buffer);	}
		else{	col_sep = self.col_sep;	}
		let row_sep;
		if self.row_sep == b'\0'{	row_sep = get_row_sep(buffer);	}
		else{	row_sep = self.row_sep;	}
        CSV{
			rdr : rdr, col_sep : col_sep, row_sep : row_sep,
			last_field_result: FieldResult::Field, pos:0, 
			encoding: encoding, sep_iter: None,
		}
    }
}

impl<'a, R: io::Read> CSV<'a, R> {
	// fn next_byte(&mut self) -> u8{
	// 	self.pos += 1;
	// 	self.buffer[self.pos]
	// }

	pub fn next(&mut self) -> (FieldResult, &str){
		let mut quete_on = false;
		let start_pos = self.pos;
		loop{
			let next_sep_pos_wrap = self.sep_iter.next(); 
			if let Some(next_sep_pos) = next_sep_pos_wrap {
				self.pos = next_sep_pos;
			} else if matches!(self.last_field_result, FieldResult::FieldEnd){
				return (FieldResult::End, "")
			} else {
				let col = unsafe {
					std::str::from_utf8_unchecked(&self.buffer[start_pos..])
				};
				println!("{}", col);
				self.last_field_result = FieldResult::FieldEnd;
				return (FieldResult::FieldEnd, col)
			}
			let ch = self.buffer[self.pos];
			if quete_on{
				quete_on = !quete_on;
				continue
			}
			else if ch == b'"' {
				quete_on = !quete_on;
				continue
			}

			let col = unsafe {
				std::str::from_utf8_unchecked(&self.buffer[start_pos..self.pos])
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
				if self.buffer.len() == self.pos + 1 {
					self.pos += 1;
					return (FieldResult::FieldEnd, col)
				}
				let next_ch = self.buffer[self.pos+1];
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
