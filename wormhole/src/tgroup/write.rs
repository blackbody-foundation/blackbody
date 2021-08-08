/*
    .. + write.rs + ..

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

//! write cccs or any file

use super::cmn::*;

mod func;

derive_substruct! {
    super: Requirement;
    pub struct TWrite {
        version: Version,
        file_path: PathBuf,
    }
}
impl TSubGroup<Message> for TWrite {
    type R = Requirement;
    type O = (); // join handler's output type
    fn new(
        requirement: &Self::R,
        channel: Chan<Message>,
    ) -> std::thread::JoinHandle<ResultSend<Self::O>> {
        // -> rx
        let info = Self::copy_from_super(requirement);

        std::thread::spawn(move || -> ResultSend<Self::O> {
            let mut file_path = info.file_path;

            file_path.set_extension("cccs_tmp0");

            let mut writer = func::get_writer(&file_path)?;

            let mut header = func::preprocess_recv(&channel, &mut writer)?;

            // looping
            while let Ok(m) = channel.recv() {
                match m.kind {
                    Kind::Phase0Forward => {
                        writer.write_all(m.payload.as_slice())?;
                    }
                    Kind::Phase0End => {
                        header.last_pos.push(writer.stream_position()?); // save the last position

                        // check last remains
                        if !m.payload.is_empty() {
                            writer.write_all(m.payload.as_slice())?;
                        }

                        let header_bytes = errcast!(header.to_bytes());
                        writer.write_all(&header_bytes)?; // re-write header
                        break;
                    }
                    _ => break,
                }
            }
            Ok(())
        })
    }
}
