use blake3;
use ed25519_dalek::Digest;
use sha3::Sha3_256;

pub enum Version {
    Zero,
}

impl Version {
    pub fn into_kind(self) -> Kind {
        match self {
            Self::Zero => Kind(0, vec![boxed(blake3_256), boxed(sha3_256)], 4),
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Self::Zero => String::from("Zero"),
        }
    }
}

pub fn encode<T: AsRef<[u8]>>(data: T, version: Version) -> Vec<u8> {
    version.into_kind().attach_to(data.as_ref())
}
pub fn decode<T: AsRef<[u8]>>(
    data: T,
    version: Version,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    version.into_kind().detach_from(data.as_ref())
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
        let mut version = Vec::from(self.0.to_le_bytes());
        let mut checksum = src.to_owned();
        let hashing = self.1;
        for h in hashing.iter() {
            checksum = h(&checksum);
        }
        version.extend_from_slice(src);
        let checksum_len = self.2 as usize;
        for &byte in checksum.iter().take(checksum_len) {
            version.push(byte);
        }
        version
    }
    pub fn detach_from(self, src: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let target_version = self.0.to_le_bytes();
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
        let mut target_checksum0 = Vec::from(payload.as_slice());
        for h in hashing.iter() {
            target_checksum0 = h(&target_checksum0);
        }
        let mut target_checksum = Vec::new();
        for byte in target_checksum0.into_iter().take(checksum_len) {
            target_checksum.push(byte);
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
