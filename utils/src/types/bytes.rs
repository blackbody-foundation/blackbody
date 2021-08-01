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

pub trait BytesSer<T>
where
    T: serde::Serialize,
{
    fn into_bytes(self) -> Result<T>;
}
pub trait BytesDe<T>
where
    T: serde::de::DeserializeOwned,
{
    fn into_something(self) -> Result<T>;
}

impl<T: serde::Serialize> BytesSer<Option<Vec<u8>>> for Option<T> {
    fn into_bytes(self) -> Result<Option<Vec<u8>>> {
        match self {
            Some(v) => Ok(Some(bincode::serialize(&v)?)),
            _ => Ok(None),
        }
    }
}
impl<T: serde::de::DeserializeOwned> BytesDe<Option<T>> for Option<Vec<u8>> {
    fn into_something(self) -> Result<Option<T>> {
        match self {
            Some(v) => Ok(Some(bincode::deserialize::<T>(&v)?)),
            _ => Ok(None),
        }
    }
}

impl<T: serde::Serialize> BytesSer<Vec<u8>> for T {
    fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(&self)?)
    }
}
impl<T: serde::de::DeserializeOwned> BytesDe<T> for Vec<u8> {
    fn into_something(self) -> Result<T> {
        Ok(bincode::deserialize::<T>(&self)?)
    }
}
