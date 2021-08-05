/*
    .. + cmn.rs + ..

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

//! common

pub use std::io::SeekFrom;

pub use utils::{
    derive_substruct,
    fs::{
        types::{uPS, ReadPtr, WritePtr, LS},
        File,
    },
    system::*,
    types::{bytes::*, chan::*, tgroup::*},
};

pub use crate::CCCSHeader;
pub use otoodb::*;

pub use crossbeam::channel;
pub const BOUNDED_CAP: usize = 1024;

utils::message! {
    pub msg,
    M = Vec<u8>,
    K = enum {

        Phase0Header,
        Phase0Through,
        Phase0Forward,
        Phase0End,

        Phase1Header,
        Phase1Through,
        Phase1Forward,
        Phase1End,

    }
}

pub use msg::*;

pub fn send_message(chan: &Chan<Message>, kind: Kind, payload: TypePayload) -> ResultSend<()> {
    chan.send(Message::new(kind, payload))
}

utils::derive_new! {
    pub struct Requirement {
        pub file_path: PathBuf, // target
        pub db: otoodb::DB,
        pub version: HHSize
    }
}
