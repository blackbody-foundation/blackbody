/*
    .. + process.rs + ..

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

//! transfrom

use super::cmn::*;

mod func;

derive_substruct! {
    super: Requirement;
    pub struct TProcess {
        db: DB,
        file_path: PathBuf,
    }
}
impl TSubGroup<Message> for TProcess {
    type R = Requirement;
    type O = (); // join handler's output type
    fn new(
        requirement: &Self::R,
        channel: Chan<Message>,
    ) -> std::thread::JoinHandle<ResultSend<Self::O>> {
        // -> rx -> tx
        let info = Self::copy_from_super(requirement);

        std::thread::spawn(move || -> ResultSend<Self::O> {
            let mut db = info.db;

            let (mut header, db_version, db_src_size, db_dst_size) =
                func::preprocess_recv(&channel, &db)?;

            let mut temporary = Vec::<u8>::new();
            let mut found_count: uPS = 0;

            // looping
            'outer: while let Ok(m) = channel.recv() {
                match m.kind {
                    Kind::Phase0Forward => {
                        temporary.extend(m.payload.into_iter()); // extend receiving temp buf

                        if temporary.len() < db_src_size {
                            // temp buf must be more than otoo src len
                            continue;
                        }

                        let mut chunks = temporary.chunks(db_src_size);
                        let count = chunks.len() - 1;

                        let mut src_bytes;
                        // iterate chunks of otoo src len
                        for _ in 0..count {
                            src_bytes = chunks.next().unwrap();
                            // transform src to dst (otoodb pair)
                            if let Ok(dst_bytes) = db.get(src_bytes) {
                                let dst_bytes = match dst_bytes {
                                    Some(v) => {
                                        found_count += 1;
                                        eprint!("\rtransform: {}", found_count);
                                        v
                                    }
                                    None => src_bytes.to_vec(),
                                };
                                // to writing channel
                                if send_message(&channel, Kind::Phase0Forward, dst_bytes).is_err() {
                                    break 'outer;
                                }
                            }
                        }
                        let last = chunks.next().unwrap().to_vec();
                        // check remains
                        temporary = if last.len() == db_src_size {
                            if send_message(&channel, Kind::Phase0Forward, last).is_err() {
                                break 'outer;
                            }
                            Vec::<u8>::new()
                        } else {
                            last
                        };
                    }
                    _ => break,
                }
            }

            send_message(&channel, Kind::Phase0End, temporary)
        })
    }
}
