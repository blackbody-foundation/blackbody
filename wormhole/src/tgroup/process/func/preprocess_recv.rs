/*
    .. + preprocess_recv.rs + ..

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

use super::*;

/// should be a first receive.
/// ## Return
/// (***Header***, ***db_version***, ***db_src_size***, ***db_dst_size***)
pub fn preprocess_recv(
    channel: &Chan<Message>,
    db: &DB,
) -> ResultSend<(Box<CCCSHeader>, usize, usize, usize)> {
    let mut header;
    let (db_version, mut db_src_size, mut db_dst_size) = db.get_info_as_usize();
    match channel.recv().unwrap() {
        //
        m if m.kind == Kind::Phase0Header => {
            //
            if m.payload.is_empty() {
                // if target file has no header
                header = CCCSHeader::default(); // create processing header
                header.version = db_version as HHSize;
            } else {
                // has a header
                let t: CCCSHeader = m.payload.into_something_send()?;
                header = Box::new(t);

                if header.version != db_version as HHSize {
                    // *** warning: matching our db version ***
                    return errbang!(err::UnexpectedVersion);
                }

                if header.cccs_flag {
                    // decode order
                    std::mem::swap(&mut db_src_size, &mut db_dst_size);
                }
            }
            //
            send_message(channel, Kind::Phase0Header, header.to_bytes_send()?)?;

            Ok((header, db_version, db_src_size, db_dst_size))
        }
        _ => {
            return errbang!(err::ThreadReceiving, "first sending should be a header.");
        }
    }
}
