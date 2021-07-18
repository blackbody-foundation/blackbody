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

use crate::fs::types::{HeaderTrait, Lim, Packet, FM, LS};
use crate::{
    system::*,
    types::{MBuf, CHUNK_SIZE},
};

pub mod cross_insert {

    use super::*;

    pub fn insert<T: HeaderTrait>(fm: &mut FM<T>, mut packet: Packet) -> Result<()> {
        packet.sort_by_key(|k| k.1); // sort by position in the file

        let mut read_buf = MBuf::new(packet[0].1);
        let mut write_buf = MBuf::new(packet[0].1);

        let lim = Lim::<LS>::new(0, CHUNK_SIZE);

        let (mut next_packet_pos, mut rest_pos);

        let max_i = packet.len() - 1;

        let mut eop;

        for (i, (bytes, pos)) in packet.iter().enumerate() {
            write_buf.set_buf_from(bytes.as_slice())?;

            next_packet_pos = if i < max_i {
                packet[i + 1].1
            } else {
                fm.content_end_pos(false)?
            };

            if bytes.len() < (next_packet_pos - pos) as LS {
                return errbang!(
                    err::OutOfBounds,
                    "insert bytes length must be less than previous read buf length."
                );
            }
            dbg!("before", &write_buf.pos(), &read_buf.pos());
            read_buf.reset(*pos);

            eop = false;
            while !eop {
                rest_pos = lim.lim((next_packet_pos - read_buf.pos()) as LS);

                read_buf.set_len(rest_pos); // limited

                eop = read_checking(fm, &mut read_buf)?;

                write_checking(fm, &mut write_buf)?;

                std::mem::swap(&mut read_buf.buf, &mut write_buf.buf);
                write_buf.set_len(read_buf.len());
            }
        }
        dbg!("done.", fm.debug()?);
        Ok(())
    }

    fn read_checking<T: HeaderTrait>(fm: &mut FM<T>, mbuf: &mut MBuf) -> Result<bool> {
        fm.set_cursor(mbuf.pos())?;

        let num_read = fm.read_general(mbuf.get_mut_slice())?;
        mbuf.add_num_process(num_read);

        Ok(num_read == 0)
    }
    fn write_checking<T: HeaderTrait>(fm: &mut FM<T>, mbuf: &mut MBuf) -> Result<usize> {
        let len = mbuf.len();

        fm.write_cursoring(mbuf.get_slice(), mbuf.pos())?;
        mbuf.add_num_process(len);

        Ok(len)
    }
}
