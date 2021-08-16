/*
    .. + insert.rs + ..

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

use crate::fs::types::{uPS, HeaderTrait, Packet, FM, LS};
use crate::{
    system::*,
    types::{MBuf, CHUNK_SIZE},
};

pub mod cross_insert {

    use super::*;

    pub fn insert<T: HeaderTrait>(
        fm: &mut FM<T>,
        mut packet: Packet,
        remove_mode: bool,
    ) -> Result<()> {
        packet.sort_by_key(|k| k.1); // sort by position in the file

        let mut read_buf = MBuf::default();
        let mut write_buf = MBuf::default();

        let mut eof;
        let mut end_pos;
        let mut added_pos = 0;

        for (bytes, pos) in packet.iter() {
            end_pos = fm.content_end_pos(true)?;

            let mut pos = if remove_mode {
                *pos - added_pos
            } else {
                *pos + added_pos
            };
            if pos > end_pos {
                pos = end_pos;
            }
            let bytes_len = bytes.len() as uPS;

            write_buf.reset(pos);
            read_buf.reset(if remove_mode { pos + bytes_len } else { pos });

            write_buf.set_buf_from(if remove_mode { &[] } else { bytes.as_slice() })?;

            if bytes.len() >= CHUNK_SIZE {
                return errbang!(
                    err::OutOfBounds,
                    "insert bytes length must be less than CHUNK_SIZE: {}",
                    CHUNK_SIZE
                );
            }

            eof = false;
            while !eof {
                read_buf.set_len((end_pos - read_buf.pos()) as LS);

                eof = read_checking(fm, &mut read_buf)?;

                write_checking(fm, &mut write_buf)?;

                std::mem::swap(&mut read_buf.buf, &mut write_buf.buf);
                write_buf.set_len(read_buf.len());
            }

            added_pos += bytes_len;
        }
        Ok(())
    }

    #[inline]
    fn read_checking<T: HeaderTrait>(fm: &mut FM<T>, mbuf: &mut MBuf<CHUNK_SIZE>) -> Result<bool> {
        fm.set_cursor(mbuf.pos())?;

        let num_read = fm.read_general(mbuf.get_mut_slice())?;
        mbuf.add_num_process(num_read);

        Ok(num_read == 0)
    }

    #[inline]
    fn write_checking<T: HeaderTrait>(
        fm: &mut FM<T>,
        mbuf: &mut MBuf<CHUNK_SIZE>,
    ) -> Result<usize> {
        let len = mbuf.len();

        fm.write_cursoring(mbuf.get_slice(), mbuf.pos())?;
        mbuf.add_num_process(len);

        Ok(len)
    }
}
