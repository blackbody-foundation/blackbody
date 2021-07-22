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
    mut target: target::OtooDB,
    write_tx: channel::Sender<Vec<u8>>,
) -> io::Result<()> {
    let mut temporary = Vec::<u8>::new();
    let mut len;
    let mut rest;
    let (src_size, dst_size) = target.get_info();

    let mut found_count: uPS;
    'outer: loop {
        found_count = 0;
        while let Ok(r_vec) = read_rx.recv() {
            // received vector into the temporary vector
            temporary.extend(r_vec.into_iter());

            len = temporary.len();

            if len < src_size {
                continue; // collect more
            }

            // get the chunks of source's bytes
            for src_bytes in temporary.chunks(src_size) {
                //
                // transform source bytes to target bytes
                if let Some(dst_bytes) = target.transform(src_bytes) {
                    //
                    // send the target bytes and then break out
                    if write_tx.send(dst_bytes).is_err() {
                        break 'outer;
                    }
                    found_count += 1;
                }
            }

            // calculates rest of source bytes
            rest = (len % src_size) / 8;
            // replace whole vector of temporary to the rest of source bytes
            temporary = (temporary[len - rest..len]).to_vec();
        }
        // completed transforming
        // flush header,
        // temporary.len() <- stopped index
        if found_count == 0 {
            // if any of transforming process doesn't, just break out
            break;
        }
        // or repeat more
    }

    Ok(())
}
