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
pub use keypair::Keypair;

// a b c e / f g h k / n p s t / u x y z = 16
// HCS(How to Count Stars HCS) notation
/*
const WORDS: [char;16] = [
 'a', 'b', 'c', 'e',
 'f', 'g', 'h', 'k',
 'n', 'p', 's', 't',
 'u', 'x', 'y', 'z',
];

fn main() {
    let a: [u128; 12] = [1,112421423423423426,3,13,223,243,211,41,3,64,12,0];
    for &x in a.iter() {
        let x = x as usize;
        let n: usize = x / 16;
        let r: usize = x % 16;
        print!("{}{}", WORDS[r], n);
    }
    println!();

}

*/

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

        // gen phrase & seed
        let (phrase1, seed1) = gen::new_seed("test1234", Language::English).unwrap();
        println!("* phrase1: {}\n* seed1: {:?}", &phrase1, &seed1);
        let keypair1 = Keypair::new(&seed1).unwrap();
        println!("-- keypair1:\n{:?}\n", keypair1);

        // distribute phrase into paths
        shield::thrust_mnemonic_phrase(&phrase1, &dirs, "testtest", 24213421321).unwrap();
        println!("save successed.");

        // reload phrase
        let phrase_reload =
            shield::extract_mnemonic_phrase(&dirs, "testtest", 24213421321).unwrap();
        println!("recovered: {}\n", phrase_reload);

        // gen second phrase & seed
        let seed2 = gen::seed_from_phrase("test1234", Language::English, &phrase_reload).unwrap();
        println!("* phrase2: {}\n* seed2: {:?}", &phrase_reload, &seed2);
        let keypair2 = Keypair::new(&seed2).unwrap();
        println!("-- keypair2:\n{:?}\n", keypair2);

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
