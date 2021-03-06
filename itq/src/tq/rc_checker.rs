/*
    .. + rc_checker.rs + ..

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

use std::sync::{atomic::AtomicUsize, Arc, Mutex};

pub type Checker = Mutex<AtomicUsize>;
pub struct RcCheker {
    checker: Arc<Checker>,
}
impl RcCheker {
    pub fn new() -> Self {
        Self {
            checker: Arc::new(Checker::new(AtomicUsize::new(0))),
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            checker: Arc::clone(&self.checker),
        }
    }
    pub fn count(&self) -> usize {
        Arc::strong_count(&self.checker)
    }
}
