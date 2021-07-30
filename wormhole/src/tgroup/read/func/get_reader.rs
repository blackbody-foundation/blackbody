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

use io::BufReader;

use super::*;

pub fn get_reader(file_path: &str) -> ResultSend<Box<dyn ReadPtr>> {
    let header = CCCSHeader::default();
    let reader: Box<dyn ReadPtr> = match File::open(file_path, header) {
        Ok(v) => Box::new(v),
        Err(_) => Box::new(BufReader::new(std::fs::File::open(file_path)?)),
    };
    Ok(reader)
}
