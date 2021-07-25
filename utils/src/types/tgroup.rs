/*
    .. + tgroup.rs + ..

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

/// Thread Group.<br>
/// R = Requirement<br>
/// O = Handle Output<br>
pub trait TGroup {
    type R;
    type O;
    fn new(requirement: Self::R) -> Self;
    fn join(self) -> Vec<Self::O>;
}

use super::chan::Chan;

/// Thread Sub Group.<br>
/// R = Requirement<br>
/// O = Handle Output<br>
/// M = Message Type<br>
pub trait TSubGroup<M = ()> {
    type R;
    type O;
    fn new(requirement: &Self::R, channel: Chan<M>) -> std::thread::JoinHandle<Self::O>;
}

pub use crate::tgroup;
