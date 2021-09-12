use std::borrow::Cow;
use bufchr::Bufchr3;
use bytecount;


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
	let cr = bytecount::count(readed_byte, 0x0D);
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

// row_sep => row seperator start character
// if \r\n => just \r
pub struct CSV<'a> {
	buffer: &'a std::vec::Vec<u8>,
    col_sep: u8,
	row_sep: u8, 
	pos: usize,
	sep_iter: Bufchr3<'a>,
} 

impl<'a> CSV<'a> {
    pub fn new(buffer: &'a mut std::vec::Vec<u8>) -> CSV {
		let slice_size = std::cmp::min(buffer.len(), 2000 as usize);
		let col_sep = get_col_sep(buffer);
		let row_sep = get_row_sep(buffer);
		let sep_iter = Bufchr3::new(&buffer[..], col_sep, row_sep, b'\'');
        CSV {
			buffer: buffer,
			col_sep: col_sep,
			row_sep: row_sep,
			pos: 0 as usize,
			sep_iter: sep_iter
		}
    }

	pub fn next(&mut self) -> (FieldResult, &str){
		let mut quete_on = false;
		let start_pos = self.pos;
		loop{
			let next_sep_pos_wrap = self.sep_iter.next(); 
			if let Some(next_sep_pos) = next_sep_pos_wrap {
				self.pos = next_sep_pos;
			} else {
				let col = unsafe {
					std::str::from_utf8_unchecked(&self.buffer[start_pos..self.pos])
				};
				return (FieldResult::End, col)
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
				return (FieldResult::Field, col)
			}
			else if ch == (self.row_sep as u8) {
				// The end of the file is always EOF. There is no buffer overflow.
				let next_ch = self.buffer[self.pos];
				if next_ch == 0x0A {  // check cr_lf
					self.pos += 1;
				}
				return (FieldResult::FieldEnd, col)
			}
			else if ch == 0x00 {
				return (FieldResult::End, col)
			}
		}
	}

}