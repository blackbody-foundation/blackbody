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
    pub fn open(file_path: &'static str, header: Header) -> io::Result<Self> {
        let reader: Reader = match IOFile::open(file_path) {
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
            Ok(file) => file.into_reader(),
        };
        let reader = header.compare(reader)?;
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
