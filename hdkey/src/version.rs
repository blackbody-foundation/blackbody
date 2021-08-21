/*
    .. + version.rs + ..

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

use std::str::FromStr;

use ed25519_dalek::Digest;
use sha3::Sha3_256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    Secret,
    Public,
}
impl KeyType {
    #[inline]
    pub fn as_version(&self) -> u16 {
        match self {
            Self::Secret => 127,
            Self::Public => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetType {
    MainNet = 0,
    TestNet = 1,
}

impl NetType {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TestNet => "TestNet",
            Self::MainNet => "MainNet",
        }
    }
    #[inline]
    pub fn as_version(&self) -> u16 {
        *self as u16
    }
}

// *********************** CONFIGURABLE ***********************

const SERVER_COUNT: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    Zero(NetType),
}
impl Version {
    pub fn into_kind(self, key_type: KeyType) -> Kind {
        match self {
            Self::Zero(net) => Kind(
                net.as_version() + key_type.as_version(),
                vec![boxed(blake3_256), boxed(sha3_256)],
                4,
            ),
        }
    }
    #[inline]
    pub fn as_list() -> [&'static str; SERVER_COUNT] {
        ["Zero"]
    }
}
impl FromStr for Version {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero:TestNet" => Ok(Self::Zero(NetType::TestNet)),
            "Zero:MainNet" => Ok(Self::Zero(NetType::MainNet)),
            _ => Err("parse error".into()),
        }
    }
}
impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Self::Zero(net) => format!("Zero:{}", net.as_str()),
        }
    }
}

// ************************************************************

#[inline]
pub fn encode<T: AsRef<[u8]>>(data: T, version: Version, key_type: KeyType) -> Vec<u8> {
    version.into_kind(key_type).attach_to(data.as_ref())
}

#[inline]
pub fn decode<T: AsRef<[u8]>>(
    data: T,
    version: Version,
    key_type: KeyType,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    version.into_kind(key_type).detach_from(data.as_ref())
}

#[inline(always)]
fn boxed<H>(func: H) -> Box<H>
where
    H: Fn(&[u8]) -> Vec<u8>,
{
    Box::new(func)
}

// ==========================

fn blake3_256(src: &[u8]) -> Vec<u8> {
    blake3::hash(src).as_bytes().to_vec()
}

fn sha3_256(src: &[u8]) -> Vec<u8> {
    Sha3_256::digest(src).to_vec()
}

// ==========================

type Hashing = Box<dyn Fn(&[u8]) -> Vec<u8>>;

/// Kind( `version prefix`, `hash functions`, `checksum length` )
pub struct Kind(u16, Vec<Hashing>, u16);

impl Kind {
    pub fn attach_to(self, src: &[u8]) -> Vec<u8> {
        let mut version = Vec::from(self.0.to_be_bytes());
        version.extend_from_slice(src); // version + src
        let mut checksum = version.to_owned();
        let hashing = self.1;
        for h in hashing.iter() {
            checksum = h(checksum.as_slice());
        }
        let checksum_len = self.2 as usize;
        for &byte in checksum.iter().take(checksum_len) {
            version.push(byte); // (version + src) + checksum[..len]
        }
        version
    }
    pub fn detach_from(self, src: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let target_version = self.0.to_be_bytes();
        let src_len = src.len();
        let version_len = target_version.len();
        let checksum_len = self.2 as usize;
        let mut version = Vec::new();
        let mut payload = Vec::new();
        let mut checksum = Vec::new();
        for (i, &byte) in src.iter().enumerate() {
            if i < version_len {
                version.push(byte);
            } else if i < src_len - checksum_len {
                payload.push(byte);
            } else {
                checksum.push(byte);
            }
        }
        let hashing = self.1;
        let mut target_checksum0 = Vec::from(version.as_slice());
        target_checksum0.extend_from_slice(payload.as_slice()); // version + src
        for h in hashing.iter() {
            target_checksum0 = h(target_checksum0.as_slice());
        }
        let mut target_checksum = Vec::new();
        for byte in target_checksum0.into_iter().take(checksum_len) {
            target_checksum.push(byte); // H(version + src)[..len] = checksum
        }
        if version != target_version.as_ref() {
            return Err(format!(
                "version is not match: {} != your {}",
                hex::encode(target_version),
                hex::encode(version)
            )
            .into());
        }
        if checksum != target_checksum {
            return Err(format!(
                "checksum is not match: {} != your {}",
                hex::encode(target_checksum),
                hex::encode(checksum)
            )
            .into());
        }
        Ok(payload)
    }
}
