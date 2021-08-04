/*
    .. + read.rs + ..

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

//! read cccs or any file

use utils::types::CHUNK_SIZE;

use super::cmn::*;

mod func;

derive_substruct! {
    super: Requirement;
    pub struct TRead {
        file_path: PathBuf,
    }
}

impl TSubGroup<Message> for TRead {
    type R = Requirement;
    type O = (); // join handler's output type
    fn new(
        requirement: &Self::R,
        channel: Chan<Message>, // tx ->
    ) -> std::thread::JoinHandle<ResultSend<Self::O>> {
        let info = Self::copy_from_super(requirement);

        std::thread::spawn(move || -> ResultSend<Self::O> {
            let (mut reader, header) = func::get_reader(&info.file_path)?;

            func::send_header(&channel, header)?;

            reader.seek(SeekFrom::Start(0))?;
 
            let mut buf = [0_u8; CHUNK_SIZE];
            let mut num_read;
            // looping
            loop {
                num_read = match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(v) => v,
                };
                send_message(&channel, Kind::Phase0Forward, Vec::from(&buf[..num_read]))?;
            }
            Ok(())
        })
    }
}
