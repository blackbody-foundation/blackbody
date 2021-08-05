/*
    .. + get_reader.rs + ..

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

/// header into bytes and send to the next pipechan with **Kind::Phase0Header**<br>
/// if header is None then header will be ***Vec::new()*** (= empty vector)
/// ## Return
/// header's `last pos` vector. (if header is None then returning is `emptry vector`)
pub fn send_header(chan: &Chan<Message>, header: Option<Box<CCCSHeader>>) -> ResultSend<Vec<uPS>> {
    let mut last_pos = Vec::new();
    let header = match header {
        Some(v) => {
            last_pos = v.last_pos.clone();
            v.into_bytes_send()?
        }
        None => Vec::new(),
    };
    send_message(chan, Kind::Phase0Header, header)?;
    Ok(last_pos)
}
