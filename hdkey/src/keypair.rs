use super::version::{self, Version};
use ed25519_dalek::{
    Keypair as dalekKeypair, PublicKey, SecretKey, KEYPAIR_LENGTH, SECRET_KEY_LENGTH,
};
use std::{error::Error, fmt};

pub struct Keypair(dalekKeypair);

impl Keypair {
    pub fn new(seed: &[u8]) -> Result<Self, Box<dyn Error>> {
        if seed.len() != KEYPAIR_LENGTH {
            /* = 32 * 2,  [8 * (32 * 2) = 512 bits] */
            return Err(format!(
                "seed size must be {}, you are {}",
                KEYPAIR_LENGTH,
                seed.len()
            )
            .into());
        }
        let secret = SecretKey::from_bytes(&seed[..SECRET_KEY_LENGTH])?; // L 256 bits
        let public = PublicKey::from(&secret);
        Ok(Self(dalekKeypair { secret, public }))
    }
    pub fn public(&self) -> &PublicKey {
        &self.0.public
    }
}

impl Keys for PublicKey {}
impl Keys for SecretKey {}

trait Keys: AsRef<[u8]> {
    fn as_hex(&self) -> String {
        hex::encode(self.as_ref())
    }
    fn as_base58(&self, version: Version) -> String {
        bs58::encode(version::encode(self.as_ref(), version).as_slice()).into_string()
    }
}
impl fmt::Debug for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let sk_hex = self.0.secret.as_hex();
        let pk_hex = self.0.public.as_hex();
        let sk_bs58 = self.0.secret.as_base58(Version::Zero);
        let pk_bs58 = self.0.public.as_base58(Version::Zero);
        write!(
            f,
            "hex (\nsecret: {}\npublic: {}\n)\nbase58[V.{}] (\nsecret: {}\npublic: {}\n)\n",
            sk_hex,
            pk_hex,
            Version::Zero.to_string(),
            sk_bs58,
            pk_bs58
        )
    }
}

impl PartialEq for Keypair {
    fn eq(&self, other: &Self) -> bool {
        (self.0.public == other.0.public) && (self.0.secret.as_bytes() == other.0.secret.as_bytes())
    }
}
impl Eq for Keypair {}
