/*
    .. + insert.rs + ..

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

use crate::fs::types::{uPS, HeaderTrait, Lim, Packet, FM, LS};
use crate::{system::*, types::CHUNK_SIZE};

pub mod cross_insert {
    use crate::types::mbuf::MBuf;

    use super::*;

    pub fn insert<T: HeaderTrait>(fm: &mut FM<T>, mut packet: Packet) -> Result<()> {
        packet.sort_by_key(|k| k.1); // sort by position in the file

        let reader = fm.try_to_create_reader()?;
        let writer = fm.try_to_create_writer()?;

        let buf1 = MBuf::default();
        let buf2 = MBuf::default();

        let (mut next_target_pos, mut limit) = (0, 0);

        let lim = Lim::<LS>::new(0, CHUNK_SIZE);

        let max_i = packet.len() - 1;

        let mut eof = false;

        for (i, (bytes, pos)) in packet.iter().enumerate() {
            buf1.reset(*pos);
            buf2.reset(*pos);

            next_target_pos = if i < max_i {
                packet[i + 1].1
            } else {
                fm.content_lim.end
            };

            while !eof {
                limit = lim.lim((next_target_pos - read_pos) as LS);

                eof = read_checking(fm, &mut buf1[..limit], &mut read_pos)?;
                write_checking(fm, &mut buf2, &mut write_pos)?;

                std::mem::swap(&mut buf1, &mut buf2);
            }
        }

        Ok(())
    }

    fn read_checking<T: HeaderTrait>(
        fm: &mut FM<T>,
        buf: &mut [u8],
        pos: &mut uPS,
    ) -> Result<bool> {
        fm.set_cursor(*pos)?;
        let num_read = fm.read_general(buf)? as uPS;
        *pos += num_read;
        Ok(num_read == 0)
    }
    fn write_checking<T: HeaderTrait>(fm: &mut FM<T>, buf: &mut [u8], pos: &mut uPS) -> Result<()> {
        fm.write_cursoring(buf, *pos)?;
        *pos += buf.len() as uPS;
        Ok(())
    }
}
