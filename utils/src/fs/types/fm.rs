/*
    .. + fm.rs + ..

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

use super::*;

#[derive(Debug)]
pub struct FM<T> {
    ptr: Ptr,
    pub path: PathBuf,
    pub header: Box<T>,
    pub header_size: uPS,
    pub file_size: uPS,
    pub content_lim: Lim<uPS>,
}

impl<T: HeaderTrait> FM<T> {
    pub fn new(path: &str, mut header: Box<T>) -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        let mut ptr = Ptr::new(file);

        let path = PathBuf::from(path);

        let header_size = header.read(&mut ptr)? as uPS;
        let file_size = ptr.seek(SeekFrom::End(0))?;

        Ok(Self {
            ptr,
            path,
            header,
            header_size,
            file_size,
            content_lim: Lim::new(header_size, file_size),
        })
    }
    pub fn is_eof(&mut self) -> Result<bool> {
        if 0 == self.ptr.read(&mut [0u8; 1])? {
            Ok(true)
        } else {
            self.set_cursor_relative(-1)?;
            Ok(false)
        }
    }
    pub fn set_cursor_general(&mut self, pos: uPS) -> Result<uPS> {
        Self::err_tunnel(self.ptr.seek(SeekFrom::Start(pos)))
    }
    pub fn set_cursor_relative(&mut self, pos: iPS) -> Result<uPS> {
        Ok(Self::err_tunnel(self.ptr.seek(SeekFrom::Current(pos)))? - self.header_size as uPS)
    }
    /// whole proccess exclusives header size
    pub fn set_cursor(&mut self, pos: uPS) -> Result<uPS> {
        Ok(
            Self::err_tunnel(self.ptr.seek(SeekFrom::Start(pos + self.header_size)))?
                - self.header_size as uPS,
        )
    }
    pub fn read_general(&mut self, buf: &mut [u8]) -> Result<LS> {
        Self::err_tunnel(self.ptr.read(buf))
    }
    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        Self::err_tunnel(self.ptr.read_exact(buf))
    }
    pub fn read_cursoring(&mut self, buf: &mut [u8], pos: uPS) -> Result<()> {
        self.set_cursor(pos)?;
        Self::err_tunnel(self.ptr.read_exact(buf))
    }
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        Self::err_tunnel(self.ptr.write_all(buf))
    }
    pub fn write_cursoring(&mut self, buf: &[u8], pos: uPS) -> Result<()> {
        self.set_cursor(pos)?;
        Self::err_tunnel(self.ptr.write_all(buf))
    }
    pub fn debug(&mut self) -> Result<()> {
        let mut buf = [0u8; 1024];
        let mut num_read;
        let prev_pos = SeekFrom::Current(0);
        self.set_cursor(0)?;
        eprintln!("start reading...");
        let mut total_num = 0;
        loop {
            num_read = self.read_general(&mut buf[..])?;
            if num_read < 1 {
                break;
            }
            eprint!("{:?}", &buf[..num_read]);
            total_num += num_read;
        }
        eprintln!("\nred {} bytes.", total_num);
        self.ptr.seek(prev_pos)?;
        Ok(())
    }
    pub fn content_end_pos(&mut self, flush: bool) -> Result<uPS> {
        if flush {
            self.flush_file_size()?;
        }
        Ok(self.content_lim.end - self.header_size)
    }
    pub fn flush_header(&mut self) -> Result<()> {
        let ptr = &mut self.ptr;
        let header_size = self.header.write(ptr)?;
        self.header_size = header_size as uPS;
        self.flush_file_size()
    }
    fn flush_file_size(&mut self) -> Result<()> {
        self.file_size = Self::err_tunnel(self.ptr.seek(SeekFrom::End(0)))?;
        self.content_lim = Lim::new(self.header_size, self.file_size);
        Ok(())
    }
    fn err_tunnel<E>(io_e: std::io::Result<E>) -> Result<E> {
        handle_io_error(io_e)
    }
}
