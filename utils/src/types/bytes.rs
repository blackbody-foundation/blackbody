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

use crate::system::*;

pub use crate::macros::bytes::*;
pub use primitive_types::U256;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}
impl Default for ByteOrder {
    fn default() -> Self {
        Self::LittleEndian
    }
}

pub trait BytesSer<T>
where
    T: serde::Serialize,
{
    fn into_bytes(self) -> Result<T>;
    fn into_bytes_send(self) -> ResultSend<T>;
    fn to_bytes(&self) -> Result<T>;
    fn to_bytes_send(&self) -> ResultSend<T>;
}
pub trait BytesDe<T>
where
    T: serde::de::DeserializeOwned,
{
    fn into_something(self) -> Result<T>;
    fn into_something_send(self) -> ResultSend<T>;
    fn to_something(&self) -> Result<T>;
    fn to_something_send(&self) -> ResultSend<T>;
}

pub type OptionBytes = Option<Vec<u8>>;

impl<T: serde::Serialize> BytesSer<OptionBytes> for Option<T> {
    fn into_bytes(self) -> Result<OptionBytes> {
        serialize_option_t!(&self)
    }
    fn into_bytes_send(self) -> ResultSend<OptionBytes> {
        serialize_option_t!(&self)
    }
    fn to_bytes(&self) -> Result<OptionBytes> {
        serialize_option_t!(self)
    }
    fn to_bytes_send(&self) -> ResultSend<OptionBytes> {
        serialize_option_t!(self)
    }
}

impl<T: serde::de::DeserializeOwned> BytesDe<Option<T>> for OptionBytes {
    fn into_something(self) -> Result<Option<T>> {
        deserialize_option_t!(&self)
    }
    fn into_something_send(self) -> ResultSend<Option<T>> {
        deserialize_option_t!(&self)
    }
    fn to_something(&self) -> Result<Option<T>> {
        deserialize_option_t!(self)
    }
    fn to_something_send(&self) -> ResultSend<Option<T>> {
        deserialize_option_t!(self)
    }
}

impl<T: serde::Serialize> BytesSer<Vec<u8>> for T {
    fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(serialize_t!(&self)?)
    }
    fn into_bytes_send(self) -> ResultSend<Vec<u8>> {
        Ok(serialize_t!(&self)?)
    }
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(serialize_t!(self)?)
    }
    fn to_bytes_send(&self) -> ResultSend<Vec<u8>> {
        Ok(serialize_t!(self)?)
    }
}
impl<T: serde::de::DeserializeOwned> BytesDe<T> for Vec<u8> {
    fn into_something(self) -> Result<T> {
        Ok(deserialize_t!(&self)?)
    }
    fn into_something_send(self) -> ResultSend<T> {
        Ok(deserialize_t!(&self)?)
    }
    fn to_something(&self) -> Result<T> {
        Ok(deserialize_t!(self)?)
    }
    fn to_something_send(&self) -> ResultSend<T> {
        Ok(deserialize_t!(self)?)
    }
}
