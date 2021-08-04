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
                        temporary.extend(m.payload.into_iter());

                        if temporary.len() < db_src_size {
                            continue;
                        }

                        // get the chunks of source's bytes
                        for src_bytes in temporary.chunks(db_src_size) {
                            //
                            // transform source bytes to target bytes
                            if let Ok(dst_bytes) = db.get(src_bytes) {
                                if dst_bytes.is_some() {
                                    found_count += 1;
                                }
                                //
                                // send the target bytes and then break out
                                if send_message(
                                    &channel,
                                    Kind::Phase0Forward,
                                    dst_bytes.unwrap_or_default(),
                                )
                                .is_err()
                                {
                                    break 'outer;
                                }
                            }
                        }
                    }
                    _ => break,
                }
            }

            Ok(())
        })
    }
}
