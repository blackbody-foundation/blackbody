/*
    .. + console.rs + ..

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

use crossbeam::channel::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Console {
    pub handle: JoinHandle<()>,
    sender: Sender<String>,
}

impl Console {
    pub fn new() -> Self {
        Self::default()
    }
    fn looping(receiver: Receiver<String>) {
        loop {
            match receiver.recv() {
                Ok(r) => eprint!("{}", r),
                Err(_) => {
                    eprint!("\n\n* console terminated\n");
                    break;
                }
            }
        }
    }
    pub fn log(&self, context: String) {
        if let Err(e) = self.sender.send(context) {
            eprintln!("* console sending error: {:?}", e);
        }
    }
}

impl Default for Console {
    fn default() -> Self {
        let (sender, receiver) = channel::unbounded();
        let handle = thread::spawn(move || Self::looping(receiver));
        eprint!("* console connected\n\n");
        Self { handle, sender }
    }
}
