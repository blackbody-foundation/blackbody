/*
    .. + header.rs + ..

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

use super::types::*;
use crate::system::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait TContent<'de>
where
    Self: Deserialize<'de> + Serialize + Debug + PartialEq,
{
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }
    fn decode(&self, bytes: &'de [u8]) -> Result<Self> {
        Ok(bincode::deserialize(bytes)?)
    }
}

pub trait THeader: Debug {
    fn read(&mut self, fm: &mut FM) -> Result<()>;
    fn write(&mut self, fm: &mut FM) -> Result<()>;
}
