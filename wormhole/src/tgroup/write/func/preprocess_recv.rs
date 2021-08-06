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
pub fn preprocess_recv(
    channel: &Chan<Message>,
    writer: &mut Writer,
) -> ResultSend<Box<CCCSHeader>> {
    let header;
    match channel.recv().unwrap() {
        m if m.kind == Kind::Phase0Header => {
            let t: CCCSHeader = m.payload.to_something_send()?;
            header = Box::new(t);

            if !header.cccs_flag {
                // binary -> .cccs
                writer.seek(SeekFrom::Start(0))?;
                writer.write_all(&m.payload)?; // writing default header
            }
            Ok(header)
        }
        _ => {
            return errbang!(err::ThreadReceiving, "first sending should be a header.");
        }
    }
}
