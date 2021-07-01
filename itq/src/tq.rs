/*
    .. + tq.rs + ..

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

use tokio::task;

mod rc_checker;
use rc_checker::RcCheker;

mod tq_result;
pub use tq_result::{TQError, TQFn, TQResult};

pub struct ThreadQueue {
    pub limit: usize,
    checker: RcCheker,
}

impl ThreadQueue {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            checker: RcCheker::new(),
        }
    }
    pub fn push(&self, f: TQFn) -> TQResult {
        if self.checker.count() > self.limit {
            return TQError::err(TQError::Full(f));
        }
        self.print();
        let checker = self.checker.clone(); // Counting
        task::spawn(async move {
            let _ = checker;
            f();
        });
        Ok(())
    }
    pub fn await_all(&self) {
        while self.checker.count() != 1 {}
    }
    pub fn await_some(&self) {
        while self.checker.count() >= self.limit {}
    }
    pub fn print(&self) {
        println!("RC: {}", self.checker.count());
    }
}
