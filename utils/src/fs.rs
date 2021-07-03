/*
    .. + fs.rs + ..

    Copyright (C) 2021 Hwakyeom Kim(=just-do-halee)

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

pub mod header;
pub mod types;

use types::{Convert, Header, IOFile, Reader, Writer};

use std::io::{self, ErrorKind};

pub struct File {
    pub file_path: &'static str,
    pub header: Header,
    reader: Reader,
    writer: Writer,
}

impl File {
    pub fn open(file_path: &'static str, mut header: Header) -> io::Result<Self> {
        let reader: Reader = match IOFile::open(file_path) {
            Ok(file) => file.into_reader(),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    // Create File.
                    let writer = IOFile::create(file_path)?.into_writer();
                    header.write(writer)?; // writing header
                    IOFile::open(file_path)?.into_reader()
                } else {
                    return Err(e);
                }
            }
        };
        let reader = header.read(reader)?;
        let writer = IOFile::create(file_path)?.into_writer();

        Ok(Self {
            file_path,
            header,
            reader,
            writer,
        })
    }
    pub fn close(self) {}
}
