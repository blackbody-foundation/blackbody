/*
    .. + etc.rs + ..

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

use std::collections::VecDeque;

pub struct CommandStack {
    pub max_len: usize,
    stack: VecDeque<String>,
    ptr: usize,
}
impl CommandStack {
    pub fn new(max_len: usize) -> Self {
        Self {
            max_len,
            stack: VecDeque::new(),
            ptr: 0,
        }
    }
    pub fn push(&mut self, command: &str) {
        self.stack.push_back(command.to_owned());
        if self.stack.len() > self.max_len {
            self.stack.pop_front();
        }
    }
    pub fn reset_ptr(&mut self) {
        self.ptr = self.stack.len();
    }
    pub fn traverse_up(&mut self) -> Option<String> {
        if self.ptr > 0 {
            self.ptr -= 1;
            self.traverse_current()
        } else {
            None
        }
    }
    pub fn traverse_down(&mut self) -> Option<String> {
        if self.ptr < self.stack.len() {
            self.ptr += 1;
            self.traverse_current()
        } else {
            None
        }
    }
    pub fn traverse_current(&mut self) -> Option<String> {
        self.stack.get(self.ptr).map(|v| v.to_owned())
    }
}

#[macro_export]
macro_rules! cat {
    ($($s:expr),*) => {
        &format!($($s),*)
    };
}

pub use cat;

///```no_run
/// if let Ok(_) = $rx.try_recv() {
///     return Ok(());
/// }
///```
#[macro_export]
macro_rules! check_rx_return {
    ($rx:expr) => {
        if let Ok(_) = $rx.try_recv() {
            return Ok(());
        }
    };
}
pub use check_rx_return;
