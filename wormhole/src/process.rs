/*
    .. + process.rs + ..

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

use super::cmn::*;

pub fn process_loop(
    read_rx: channel::Receiver<Vec<u8>>,
    target: Target,
    write_tx: channel::Sender<Vec<u8>>,
) -> io::Result<()> {
    let mut temporary = Vec::<u8>::new();
    let mut len;
    let mut src_set = target.src_bytes_size;
    let mut rest;

    while let Ok(received_vec) = read_rx.recv() {
        temporary.extend(received_vec.into_iter());
        len = temporary.len();
        if len < src_set {
            continue;
        }

        // processed
        rest = (len % src_set) / 8;
        temporary = (temporary[len - rest..len]).to_vec();
    }
    Ok(())
}
