/*
    .. + read.rs + ..

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

//! read cccs or any file

use super::cmn::*;

derive_substruct! {
    super: Requirement;
    pub struct TRead {
        infile: String,
    }
}

impl TSubGroup<msg::Message> for TRead {
    type R = Requirement;
    type O = (); // join handler's output type
    fn new(
        requirement: &Self::R,
        channel: Chan<msg::Message>,
    ) -> std::thread::JoinHandle<ResultSend<Self::O>> {
        // tx ->
        let info = Self::copy_from_super(requirement);

        std::thread::spawn(move || -> ResultSend<()> {
            let _reader = get_reader(&info.infile)?;

            loop {
                channel.send(msg::Message::new(msg::Kind::Through, vec![23, 12]))?;
            }
        })
    }
}

fn get_reader(infile: &str) -> ResultSend<impl io::Read> {
    let reader: Box<dyn io::Read> = if infile.is_empty() {
        Box::new(io::BufReader::new(io::stdin()))
    } else {
        Box::new(io::BufReader::new(std::fs::File::open(infile)?))
    };
    Ok(reader)
}
