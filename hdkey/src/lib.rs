/*
    .. + lib.rs + ..

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

pub mod gen;
pub use gen::Language;

pub mod shield;

mod keypair;
pub use keypair::{Keypair, WrappedKey};

mod version;
pub use version::{KeyType, NetType, Version};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    const TARGET_DIR: &str = "/Users/hwakyeom/programs/blackchain/hdkey";
    const NUM_DIRS: usize = 10;
    #[test]
    fn it_works() {
        // crate paths
        let mut dirs = Vec::new();
        for i in 0..NUM_DIRS {
            dirs.push(PathBuf::from(format!("/{}/{}", TARGET_DIR, i)));
        }
        dirs.push(PathBuf::from("/Volumes/programs/codes/hdkey/0"));
        println!();
        // remove paths
        for dir in dirs.iter() {
            if dir.exists() {
                std::fs::remove_dir_all(dir).unwrap();
            }
        }

        // gen phrase & seed
        let (phrase1, seed1) = gen::new_seed("test5678", Language::Korean).unwrap();
        println!("* phrase1: {}\n* seed1: {:?}", &phrase1, &seed1);
        let keypair1 = Keypair::new(&seed1, Version::Zero(NetType::MainNet)).unwrap();
        println!("\n-- keypair1:\n{:?}", keypair1);
        // sign with keypair 1
        let msg = "nepi is handsome guy.";
        let sig1 = keypair1.sign(msg.as_bytes(), None).unwrap();
        println!("keypair1 signed '{}'.\n\n", msg);

        // distribute phrase into paths
        shield::thrust_mnemonic_phrase(&phrase1, &dirs, "test1234", 24213421).unwrap();
        println!("successed save.");

        // reload phrase
        let phrase_reload = shield::extract_mnemonic_phrase(&dirs, "test1234", 24213421).unwrap();
        println!("recovered: {}\n", phrase_reload);

        // gen second phrase & seed
        let seed2 = gen::seed_from_phrase("test5678", Language::Korean, &phrase_reload).unwrap();
        println!("* phrase2: {}\n* seed2: {:?}", &phrase_reload, &seed2);
        let keypair2 = Keypair::new(&seed2, Version::Zero(NetType::MainNet)).unwrap();
        println!("\n-- keypair2:\n{:?}\n", keypair2);

        // verify sig 1 with keypair 2
        let _ = keypair2
            .public()
            .verify(msg.as_bytes(), None, &sig1)
            .unwrap();
        println!("keypair2 is verified as keypair1\n\n");

        // eq!
        assert_eq!(keypair1, keypair2);
        assert_eq!(phrase1, phrase_reload);
        assert_eq!(format!("{:?}", seed1), format!("{:?}", seed2));

        // remove paths
        for dir in dirs.iter() {
            if dir.exists() {
                std::fs::remove_dir_all(dir).unwrap();
            }
        }
    }
}
