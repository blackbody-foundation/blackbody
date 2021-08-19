use ed25519_dalek::{Keypair as dalekKeypair, PublicKey, SecretKey, SECRET_KEY_LENGTH};
use std::error::Error;

#[derive(Debug)]
pub struct Keypair(dalekKeypair);

impl Keypair {
    pub fn new(seed: &[u8]) -> Result<Self, Box<dyn Error>> {
        if seed.len() < SECRET_KEY_LENGTH {
            /* = 32,  [8 * 32 = 256 bits] */
            return Err(format!(
                "seed size must be {}, you are {}",
                SECRET_KEY_LENGTH,
                seed.len()
            )
            .into());
        }
        let secret = SecretKey::from_bytes(seed)?;
        let public = PublicKey::from(&secret);
        Ok(Self(dalekKeypair { secret, public }))
    }
}
