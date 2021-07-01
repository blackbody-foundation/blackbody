/*
    .. + lib.rs + ..

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

// Infinite Thread Queue

// #![allow(unused, dead_code)]

pub use tokio;

mod tq;
use tq::{TQFn, ThreadQueue};

pub struct ITQ {
    thread_queue: ThreadQueue,
}
impl Default for ITQ {
    fn default() -> Self {
        Self {
            thread_queue: ThreadQueue::new(128 * 1024),
        }
    }
}

impl ITQ {
    pub fn new() -> Self {
        ITQ::default()
    }
    pub fn push(&self, f: TQFn) {
        if let Err(e) = self.thread_queue.push(f) {
            let func = match *e {
                tq::TQError::Full(f) => {
                    dbg!(self.thread_queue.await_some()); // problem solved
                    f
                }
            };

            self.push(func); // recursive
        }
    }
    pub fn drop(self) {
        self.thread_queue.await_all();
    }
}
impl Drop for ITQ {
    fn drop(&mut self) {
        self.thread_queue.await_all();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
