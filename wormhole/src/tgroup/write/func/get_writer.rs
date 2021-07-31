/*
    .. + get_writer.rs + ..

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

use std::io::BufWriter;

use super::*;

pub type Writer = Box<dyn WritePtr>;

/// ### returns
///```no_run
/// Box<dyn WritePtr>
///```
pub fn get_writer(file_path: &Path) -> ResultSend<Writer> {
    Ok(Box::new(BufWriter::new(std::fs::File::create(file_path)?)))
}
