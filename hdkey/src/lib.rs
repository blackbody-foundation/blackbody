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

mod gen;
pub use gen::*;

pub mod shield;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    const TARGET_DIR: &str = "/Users/hwakyeom/programs/blackchain/hdkey";
    const NUM_DIRS: usize = 2;
    #[test]
    fn it_works() {
        let mut dirs = Vec::new();
        for i in 0..NUM_DIRS {
            dirs.push(PathBuf::from(format!("/{}/{}", TARGET_DIR, i)));
        }
        dirs.push(PathBuf::from("/Volumes/programs/codes/hdkey/0"));

        let ident1 = Ident::new("test1234", Language::Korean).unwrap();
        let seed1 = ident1.into_seed().unwrap();
        let phrase: &str = seed1.0.phrase();
        println!("m:{} seed:{:?}", &seed1.0, &seed1.1);
        println!();

        shield::thrust_mnemonic_phrase(phrase, &dirs, "testtest", 2).unwrap();
        println!("successed save.");
        println!();
        let phrase_out = shield::extract_mnemonic_phrase(&dirs, "testtest", 2).unwrap();
        println!("recovered: {}", phrase_out);
        println!();

        let ident2 = Ident::from("test1234", Language::Korean, &phrase_out).unwrap();
        let seed2 = ident2.into_seed().unwrap();
        println!("m2:{} seed2:{:?}", &seed2.0, &seed2.1);
        println!();

        assert_eq!(phrase, phrase_out);
        assert_eq!(format!("{:?}", seed1), format!("{:?}", seed2));

        // let ident2 = Ident::from("test", Language::English, phrase).unwrap();
        // let seed2 = ident2.into_seed().unwrap();
        // println!("m:{} seed:{:?}", &seed2.0, &seed2.1);
        // assert_eq!(seed.0.phrase(), seed2.0.phrase());
        // assert_eq!(seed.1.as_bytes(), seed2.1.as_bytes());

        for dir in dirs.iter() {
            if dir.exists() {
                std::fs::remove_dir_all(dir).unwrap();
            }
        }
    }
}
