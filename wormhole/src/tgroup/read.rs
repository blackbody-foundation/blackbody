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

// use std::{
//     fs::File,
//     io::{BufReader, Read},
// };

// use crate::cmn::*;

// pub fn read_loop(input: String, read_tx: channel::Sender<msg::Message>) -> io::Result<()> {
//     let mut reader: Box<dyn Read> = if input.is_empty() {
//         Box::new(BufReader::new(io::stdin()))
//     } else {
//         Box::new(BufReader::new(File::open(input)?))
//     };

//     Ok(())
// }

use super::cmn::*;

pub struct TRead {
    file_path: String,
}
impl TSubGroup<msg::Message> for TRead {
    type R = Requirement;
    type O = ();
    fn new(requirement: &Self::R, channel: Chan<msg::Message>) -> std::thread::JoinHandle<Self::O> {
        std::thread::spawn(move || {})
    }
}
