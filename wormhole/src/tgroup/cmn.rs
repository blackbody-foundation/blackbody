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

pub use std::io;

pub use utils::{
    derive_substruct,
    fs::{
        types::{uPS, ReadPtr, LS},
        File,
    },
    system::*,
    types::{bytes::*, chan::*, tgroup::*},
};

pub use crate::CCCSHeader;
pub use otoodb::DB;

pub use crossbeam::channel;
pub const BOUNDED_CAP: usize = 1024;

utils::message! {
    pub msg,
    M = Vec<u8>,
    K = enum {
        Header,
        Through,
        End,
    }
}

pub use msg::*;

pub fn send_message(
    chan: &Chan<Message>,
    kind: Kind,
    payload: Option<TypePayload>,
) -> ResultSend<()> {
    chan.send(Message::new(kind, payload))
}

utils::derive_new! {
    pub struct Requirement {
        pub file_path: String, // target
        pub db: otoodb::DB
    }
}
