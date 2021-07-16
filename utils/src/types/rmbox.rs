/*
    .. + rmbox.rs + ..

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

//! rebounded memory box

use std::ops::{Deref, DerefMut};
pub struct RMBox<'a, T>
where
    T: Copy,
{
    var: &'a mut T,
    m_var: T,
}
impl<'a, T> RMBox<'a, T>
where
    T: Copy,
{
    pub fn new(var: &'a mut T) -> Self {
        let m_var = var.to_owned();
        Self { var, m_var }
    }
}
impl<'a, T: Copy> Drop for RMBox<'a, T> {
    fn drop(&mut self) {
        let m_var = self.m_var;
        *self.var = m_var;
    }
}

impl<'a, T> Deref for RMBox<'a, T>
where
    T: Copy,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.var
    }
}

impl<'a, T> DerefMut for RMBox<'a, T>
where
    T: Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.var
    }
}
