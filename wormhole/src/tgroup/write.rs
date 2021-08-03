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
        version: HHSize,
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
        let mut info = Self::copy_from_super(requirement);

        std::thread::spawn(move || -> ResultSend<Self::O> {
            let mut file_path = info.file_path;

            file_path.set_extension("cccs_tmp0");

            let mut writer = func::get_writer(&file_path)?;
            let mut header;

            // preprocess receive
            match channel.recv().unwrap() {
                m if m.kind == Kind::Phase0Header => {
                    let t: CCCSHeader = m.payload.to_something_send()?;
                    header = Box::new(t);

                    if !header.cccs_flag {
                        // binary -> .cccs
                        writer.write_all(&m.payload)?; // writing default header
                    }
                }
                _ => {
                    return errbangsend!(err::ThreadReceiving, "first sending should be a header.");
                }
            }

            // looping
            while let Ok(m) = channel.recv() {
                match m.kind {
                    Kind::Phase0Forward => {
                        writer.write_all(m.payload.as_slice())?;
                    }
                    _ => break,
                }
            }
            Ok(())
        })
    }
}
