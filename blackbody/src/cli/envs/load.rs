/*
    .. + load.rs + ..

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

use crate::{err, errbang, errcast, style, Result};
use std::io::{Read, Write};
use std::{
    env,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

use hmac::{Hmac, Mac, NewMac};
use sha3::Sha3_256;

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};

const DEFAULT_PATH: &str = "./envs.locked";

use super::config::Config;

pub struct Envs {
    path: PathBuf,
}
impl Envs {
    pub fn new_config() -> Config {
        Config::default()
    }
    pub fn new() -> Self {
        let mut path =
            PathBuf::from(env::var("ENV_PATH").unwrap_or_else(|_| DEFAULT_PATH.to_string()));
        if !path.is_file() && path.is_dir() {
            path.push("envs.locked");
        }
        Self { path }
    }
    pub fn exists(&self) -> bool {
        self.path.is_file()
    }
    pub fn load(&self, password: &str) -> Result<Config> {
        if !self.exists() {
            return errbang!(err::FileNotFound);
        }
        let path = self.path.as_path();

        // read envs.locked file
        let mut file = OpenOptions::new().read(true).open(path).unwrap_or_else(|e| panic!("{}", style(format!("{}: please check the path's permission, or set the '$ENV_PATH' environment variable to change default envs path. now: {:?}", e, path)).red().bold()));
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        // decrypt
        buf = decrypt(password, buf.as_slice())?;

        // decode
        self.decode(buf)
    }
    fn decode(&self, original_src: Vec<u8>) -> Result<Config> {
        // bytes to toml
        let config: Config = toml::from_slice(original_src.as_slice())?;
        // insert env variables
        for pair in config.env.iter() {
            env::set_var(&pair.key, &pair.value);
        }
        // and return a whole config data
        Ok(config)
    }
    pub fn save(&self, password: &str, config: Config) -> Result<()> {
        // encode
        let original_src = self.encode(config)?;
        // encrypt
        let buf = encrypt(password, original_src.as_bytes())?;
        // write envs.locked file
        let path = self.path.as_path();

        set_permission(path, false);
        self.delete()?;
        let mut file = OpenOptions::new().write(true).create(true).open(path).unwrap_or_else(|e| panic!("{}", style(format!("{}: please check the path's permission, or set the '$ENV_PATH' environment variable to change default envs path. now: {:?}", e, path)).red().bold()));
        file.write_all(&buf)?;
        set_permission(path, true); // read only
        Ok(())
    }
    /// *** warning ***
    pub fn delete(&self) -> Result<()> {
        let path = self.path.as_path();
        if path.is_file() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
    fn encode(&self, config: Config) -> Result<String> {
        Ok(toml::to_string(&config)?)
    }
}
fn encrypt(password: &str, buf: &[u8]) -> Result<Vec<u8>> {
    let hash = password_to_hash(password)?;

    let secret_key = Key::from_slice(&hash);
    let cipher = Aes256Gcm::new(secret_key);

    let nonce = Nonce::from_slice(&secret_key[..12]); // 96 bits;

    Ok(errcast!(cipher.encrypt(nonce, buf), err::UnwrapingError))
}
fn decrypt(password: &str, buf: &[u8]) -> Result<Vec<u8>> {
    let hash = password_to_hash(password)?;

    let secret_key = Key::from_slice(&hash);
    let cipher = Aes256Gcm::new(secret_key);

    let nonce = Nonce::from_slice(&secret_key[..12]); // 96 bits;

    Ok(errcast!(cipher.decrypt(nonce, buf), err::UnwrapingError))
}
fn password_to_hash(password: &str) -> Result<Vec<u8>> {
    let password_len = password.len();
    if password_len < 8 {
        return errbang!(
            err::InvalidLenSize,
            "password must be more than 8 length bytes. you are {}",
            password_len
        );
    }

    let bytes = password.as_bytes();
    let mut mac =
        Hmac::<Sha3_256>::new_from_slice(blake3::hash(&bytes[..4].repeat(password_len)).as_bytes())
            .unwrap();

    mac.update(bytes);

    Ok(mac.finalize().into_bytes().to_vec())
}

#[inline]
fn set_permission(path: &Path, readonly: bool) {
    if let Ok(v) = path.metadata() {
        let mut perm = v.permissions();
        perm.set_readonly(readonly);
        std::fs::set_permissions(path, perm).unwrap_or_else(|e| eprintln!("{}", e));
    }
}
