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

impl fmt::Debug for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "secret: {}\npublic: {}\n",
            hex::encode(self.0.secret.as_bytes()),
            hex::encode(self.0.public.as_bytes())
        )
    }
}

impl PartialEq for Keypair {
    fn eq(&self, other: &Self) -> bool {
        (self.0.public == other.0.public) && (self.0.secret.as_bytes() == other.0.secret.as_bytes())
    }
}
impl Eq for Keypair {}
