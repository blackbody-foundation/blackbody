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

use crate::{cmn::*, style};
use std::io::{Read, Write};
use std::{
    env,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

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
    pub fn load(&self, password: Password) -> Result<Config> {
        if !self.exists() {
            return errbang!(err::FileNotFound);
        }
        let path = self.path.as_path();

        // read envs.locked file
        let mut file = errcast_panic!(
            OpenOptions::new().read(true).open(path),
            err::Permission,
            "{} {:?}",
            style(name!(NoEnvPath)).red().bold(),
            path
        );
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
    pub fn save(&self, password: Password, config: Config) -> Result<()> {
        // encode
        let original_src = self.encode(config)?;
        // encrypt
        let buf = encrypt(password, original_src.as_bytes())?;
        // write envs.locked file
        let path = self.path.as_path();

        set_permission(path, false);
        self.delete()?;
        let mut file = errcast_panic!(
            OpenOptions::new().write(true).create(true).open(path),
            err::Permission,
            "{} {:?}",
            style(name!(NoEnvPath)).red().bold(),
            path
        );
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
fn encrypt(password: Password, buf: &[u8]) -> Result<Vec<u8>> {
    let hash = password.as_ref();

    let secret_key = Key::from_slice(&hash[..32]);
    let cipher = Aes256Gcm::new(secret_key);

    let nonce = Nonce::from_slice(&secret_key[..12]); // 96 bits;

    Ok(errcast!(cipher.encrypt(nonce, buf), err::UnwrapingError))
}
fn decrypt(password: Password, buf: &[u8]) -> Result<Vec<u8>> {
    let hash = password.as_ref();

    let secret_key = Key::from_slice(&hash[..32]);
    let cipher = Aes256Gcm::new(secret_key);

    let nonce = Nonce::from_slice(&secret_key[..12]); // 96 bits;

    Ok(errcast!(cipher.decrypt(nonce, buf), err::UnwrapingError))
}

#[inline]
fn set_permission(path: &Path, readonly: bool) {
    if let Ok(v) = path.metadata() {
        let mut perm = v.permissions();
        perm.set_readonly(readonly);
        print_unwrap!(std::fs::set_permissions(path, perm));
    }
}
