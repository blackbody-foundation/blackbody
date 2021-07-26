/*
    .. + stdptr.rs + ..

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
pub struct StdPtr {
    instream: Stdin,
    outstream: Stdout,
}

impl Ptr for StdPtr {}

impl Read for StdPtr {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.instream.read(buf)
    }
}

impl Seek for StdPtr {
    /// this will not work
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        Ok(0)
    }
}

impl Write for StdPtr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.outstream.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
