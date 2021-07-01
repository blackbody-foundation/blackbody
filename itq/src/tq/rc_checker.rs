/*
    .. + rc_checker.rs + ..

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

use std::rc::Rc;

pub struct Checker;
pub struct RcCheker {
    pub checker: Rc<Checker>,
}
impl RcCheker {
    pub fn new() -> Self {
        Self {
            checker: Rc::new(Checker {}),
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            checker: Rc::clone(&self.checker),
        }
    }
    pub fn count(&self) -> usize {
        Rc::strong_count(&self.checker)
    }
}
unsafe impl Send for RcCheker {}
