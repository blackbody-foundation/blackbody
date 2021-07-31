/*
    .. + bytes.rs + ..

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

pub use crate::macros::bytes::*;
pub use primitive_types::U512;

use crate::system::*;

pub trait BytesSer {
    fn into_bytes(self) -> Result<Option<Vec<u8>>>;
}
pub trait BytesDe {
    fn into_something<T>(self) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned;
}

impl<T: serde::Serialize> BytesSer for Option<T> {
    fn into_bytes(self) -> Result<Option<Vec<u8>>> {
        match self {
            Some(v) => Ok(Some(bincode::serialize(&v)?)),
            _ => Ok(None),
        }
    }
}
impl BytesDe for Option<Vec<u8>> {
    fn into_something<T>(self) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            Some(v) => Ok(Some(bincode::deserialize::<T>(&v)?)),
            _ => Ok(None),
        }
    }
}
