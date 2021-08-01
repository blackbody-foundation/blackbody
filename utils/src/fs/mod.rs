/*
    .. + fs + ..

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

pub mod algorithms;
pub mod types;

use types::*;

use super::epool;

epool! {
    ///```no_run
    ///{
    ///     HasHeader(T),
    ///     HasNoHeader(T),
    ///}
    ///```
    pub enum FilePool<T>
    {
        HasHeader(T),
        HasNoHeader(T),
    }
}

/// File\<dyn HeaderTrait\>
#[derive(Debug, Clone)]
pub struct File<T>
where
    T: HeaderTrait,
{
    pub fm: FM<T>,
}

impl<T: HeaderTrait> File<T> {
    pub fn open<P: AsRef<Path>>(path: P, header: Box<T>) -> Result<Self> {
        let fm = FM::new(path.as_ref(), header)?;

        Ok(Self { fm })
    }
    pub fn close(self) {}
}

impl<T: HeaderTrait> Ptr for File<T> {}
impl<T: HeaderTrait> ReadPtr for File<T> {}
impl<T: HeaderTrait> WritePtr for File<T> {}

impl<T: HeaderTrait> Read for File<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fm.ptr.read(buf)
    }
}

impl<T: HeaderTrait> Write for File<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.fm.ptr.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.fm.ptr.flush()
    }
}

impl<T: HeaderTrait> Seek for File<T> {
    /// only content seek (this means pos(0) = pos(header_size))
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(v) => err_to_io!(self.fm.set_cursor(v)),
            SeekFrom::Current(v) => err_to_io!(self.fm.set_cursor_relative(v)),
            SeekFrom::End(v) => {
                let pos = err_to_io!(self.fm.content_end_pos(false))?;
                err_to_io!(self.fm.set_cursor(pos + v as u64))
            }
        }
    }
}
