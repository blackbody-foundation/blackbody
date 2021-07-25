/*
    .. + chan.rs + ..

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

use crate::system::*;

use crossbeam::channel::{Receiver, Sender};

#[derive(Debug, Clone)]
/// M = Message Type
pub struct Chan<M> {
    sender: Option<Sender<M>>,
    receiver: Option<Receiver<M>>,
}

impl<M> Chan<M> {
    pub fn none() -> Chan<()> {
        Chan::default()
    }
    pub fn new(sender: Option<Sender<M>>, receiver: Option<Receiver<M>>) -> Self {
        Self { sender, receiver }
    }
    pub fn is_none(&self) -> bool {
        self.sender.is_none() && self.receiver.is_none()
    }
    pub fn sender(&self) -> ResultSend<&Sender<M>> {
        match &self.sender {
            Some(v) => Ok(v),
            None => errbangsend!(err::UnwrapingError, "this channel has no sender."),
        }
    }
    pub fn receiver(&self) -> ResultSend<&Receiver<M>> {
        match &self.receiver {
            Some(v) => Ok(v),
            None => errbangsend!(err::UnwrapingError, "this channel has no receiver."),
        }
    }
    pub fn send(&self, msg: M) -> ResultSend<()> {
        match self.sender().unwrap().send(msg) {
            Ok(_) => Ok(()),
            Err(_) => errbangsend!(err::UnexpectedEof, "sender error."),
        }
    }
    pub fn recv(&self) -> ResultSend<M> {
        match self.receiver().unwrap().recv() {
            Ok(m) => Ok(m),
            Err(_) => errbangsend!(err::UnexpectedEof, "receive error."),
        }
    }
}
impl<M> Default for Chan<M> {
    fn default() -> Self {
        Self {
            sender: None,
            receiver: None,
        }
    }
}

impl<M> Channels for Chan<M> {}
pub trait Channels {}
