/*
    .. + master.rs + ..

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

use crate::{Result, VERSION};
use hdkey::*;
use std::path::Path;

use hdkey::WrappedKeypair;

pub fn read_original_key<T>(
    words: String,
    salt: usize,
    lang: Language,
    login_password: String,
    target_directories: &[T],
) -> Result<Keypair>
where
    T: AsRef<Path>,
{
    gen::master_key_from_directories(
        VERSION,
        words.as_str(),
        salt,
        lang,
        login_password.as_str(),
        target_directories,
    )
}

pub fn safe_key(keypair: Keypair) -> WrappedKeypair {
    WrappedKeypair::new(keypair)
}
