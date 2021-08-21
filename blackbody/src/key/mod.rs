use crate::Result;
use hdkey::*;
use std::path::Path;

pub const VERSION: Version = Version::Zero(NetType::TestNet);

pub fn read_key<T>(
    words: &str,
    salt: usize,
    lang: Language,
    login_password: &str,
    target_directories: &[T],
) -> Result<Keypair>
where
    T: AsRef<Path>,
{
    gen::master_key_from_directories(
        VERSION,
        words,
        salt,
        lang,
        login_password,
        target_directories,
    )
}
