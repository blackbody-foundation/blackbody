/*
    .. + shield.rs + ..

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

use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use sha3::{Digest, Sha3_256 as sha256}; // Or `Aes128Gcm`

pub fn thrust_mnemonic_phrase<T: AsRef<Path>>(
    phrase: &str,
    target_directories: &[T],
    password: &str,
    salt: usize,
) -> Result<(), Box<dyn Error>> {
    let password = password.as_bytes();
    let num_dirs = target_directories.len();
    let h_pw = password_to_hash(password, salt); // H(pw)
    let phrase_chunk_size = (phrase.len() as f32 / num_dirs as f32).ceil() as usize;
    if phrase_chunk_size < 2 {
        return Err(format!(
            "too many target directories. must be less than {}",
            phrase.len() / 2
        )
        .into());
    }
    let mut piece_of_phrase = phrase.as_bytes().chunks(phrase_chunk_size);

    let chunk_size = (h_pw.len() as f32 / num_dirs as f32).ceil() as usize;
    let piece_of_file_name = h_pw.chunks(chunk_size);

    if piece_of_phrase.len() != piece_of_file_name.len() {
        return Err(format!(
            "please adjust chunks size of environments. phrase chunks: {} != file chunks: {}",
            piece_of_phrase.len(),
            piece_of_file_name.len()
        )
        .into());
    }

    mkdir(target_directories)?;

    let file_path: Vec<PathBuf> = target_directories
        .iter()
        .zip(piece_of_file_name)
        .map(|(dir, fi)| PathBuf::from(format!(r"{}/{}", dir.as_ref().display(), hex::encode(fi))))
        .collect();
    for path in file_path.iter() {
        if path.exists() {
            return Err(format!("file already exists. {:?}", path).into());
        }
    }

    // Chunks[H(pw), num_target_dirs] = [C1, C2, C3 ..]
    for (n, path) in file_path.into_iter().enumerate() {
        // open <../target_dir/'Cn'> file
        let mut file = OpenOptions::new().create(true).write(true).open(path)?;

        // write header H(RotL( H(pw) , n ))
        let mut h_rot_l_h_pw = h_rot_l(&h_pw, n);
        file.write_all(&h_rot_l_h_pw)?;

        {
            // H(RotR( H(pw) , salt ))
            let h_rot_r_h_pw = h_rot_r(&h_pw, salt);

            // H(RotL( H(pw) , n ))  XOR  H(RotR( H(pw) , salt )) = H xor H
            h_rot_l_h_pw
                .iter_mut()
                .zip(h_rot_r_h_pw.iter())
                .for_each(|(x1, x2)| *x1 ^= *x2);
        }

        let h_h_xor_h = normal_hash256(&h_rot_l_h_pw);
        drop(h_rot_l_h_pw);

        // Secret Key = H( (H xor H) )
        let secret_key = Key::from_slice(&h_h_xor_h);
        let cipher = Aes256Gcm::new(secret_key);

        // Nonce = h(pw)
        let nonce = Nonce::from_slice(&h_pw[..12]); // 96-bits;

        if let Ok(v) = cipher.encrypt(nonce, piece_of_phrase.next().unwrap_or_default()) {
            // write piece of mnemonic
            file.write_all(&v)?;
        } else {
            return Err("encrpyt failure.".into());
        }
    }
    Ok(())
}

pub fn extract_mnemonic_phrase<T: AsRef<Path>>(
    target_directories: &[T],
    password: &str,
    salt: usize,
) -> Result<String, Box<dyn Error>> {
    mkdir(target_directories)?;
    let password = password.as_bytes();
    let num_dirs = target_directories.len();
    let h_pw = password_to_hash(password, salt); // H(pw)

    let chunk_size = (h_pw.len() as f32 / num_dirs as f32).ceil() as usize;
    let piece_of_file_name = h_pw.chunks(chunk_size);

    let mut mnemonic = Vec::new();

    let file_path: Vec<PathBuf> = target_directories
        .iter()
        .zip(piece_of_file_name)
        .map(|(dir, fi)| PathBuf::from(format!(r"{}/{}", dir.as_ref().display(), hex::encode(fi))))
        .collect();
    for path in file_path.iter() {
        if !path.exists() {
            return Err(format!("file doesn't exists. {:?}", path).into());
        }
    }

    // Chunks[H(pw), num_target_dirs] = [C1, C2, C3 ..]
    for (n, path) in file_path.iter().enumerate() {
        // open <../target_dir/'Cn'> file
        let mut file = OpenOptions::new().read(true).open(path)?;

        // read exact sized target hash
        let mut target_hash = vec![0u8; h_pw.len()];
        file.read_exact(&mut target_hash)?;

        let mut h_rot_l_h_pw = h_rot_l(&h_pw, n);
        // if H(RotL( H(pw) , n )) == loaded target hash
        if h_rot_l_h_pw == target_hash {
            {
                // H(RotR( H(pw) , salt ))
                let h_rot_r_h_pw = h_rot_r(&h_pw, salt);

                // H(RotL( H(pw) , n ))  XOR  H(RotR( H(pw) , salt )) = H xor H
                h_rot_l_h_pw
                    .iter_mut()
                    .zip(h_rot_r_h_pw.iter())
                    .for_each(|(x1, x2)| *x1 ^= *x2);
            }

            let h_h_xor_h = normal_hash256(&h_rot_l_h_pw);
            drop(h_rot_l_h_pw);

            // Secret Key = H( (H xor H) )
            let secret_key = Key::from_slice(&h_h_xor_h);
            let cipher = Aes256Gcm::new(secret_key);

            // Nonce = h(pw)
            let nonce = Nonce::from_slice(&h_pw[..12]); // 96 bits;

            // read piece of mnemonic
            let mut mnemonic_buf = Vec::new();
            file.read_to_end(&mut mnemonic_buf)?;
            if let Ok(v) = cipher.decrypt(nonce, mnemonic_buf.as_ref()) {
                mnemonic.push(v);
            } else {
                return Err("decrpyt failure.".into());
            }
        }
    }
    Ok(String::from_utf8(mnemonic.concat())?)
}

fn password_to_hash(password: &[u8], salt: usize) -> Vec<u8> {
    let h_pw = normal_hash256(&password.repeat(password[1].into()));
    let h_rot_l_h_pw = h_rot_l(&h_pw, salt.overflowing_add(1).0);
    h_rot_r(&h_rot_l_h_pw, salt.overflowing_add(2).0)
}

fn normal_hash256(input: &[u8]) -> Vec<u8> {
    let mut hasher = sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

fn h_rot_l(src: &[u8], k: usize) -> Vec<u8> {
    let mut v = src.to_vec();
    v.rotate_left(k);
    normal_hash256(&v)
}

fn h_rot_r(src: &[u8], k: usize) -> Vec<u8> {
    let mut v = src.to_vec();
    v.rotate_right(k);
    normal_hash256(&v)
}

fn mkdir<T: AsRef<Path>>(dirs: &[T]) -> Result<(), Box<dyn Error>> {
    for dir in dirs.iter() {
        if !dir.as_ref().exists() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}
