/*
    .. + get_reader.rs + ..

    Copyright 2021 Hwakyeom Kim(=just-do-halee)

    BlackBody is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    BlackBody is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with BlackBody. If not, see <http://www.gnu.org/licenses/>.

*/

use std::io::BufReader;

use super::*;

pub type Reader = (Box<dyn ReadPtr>, Option<Box<CCCSHeader>>);

/// ### returns
///```no_run
/// (Box<dyn ReadPtr>, Box<CCCSHeader>)
///```
pub fn get_reader(file_path: &Path) -> ResultSend<Reader> {
    let header = CCCSHeader::default();

    match File::open(file_path, header) {
        Ok(v) => {
            let header = v.fm.header.clone();
            dbg!(&header);
            Ok((Box::new(v), Some(header)))
        }
        Err(_) => Ok((
            Box::new(BufReader::new(std::fs::File::open(file_path)?)),
            None,
        )),
    }
}
