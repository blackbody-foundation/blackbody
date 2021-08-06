/*
    .. + main.rs + ..

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

// blackbody run
use rand::Rng;
use rand_chacha::{self, rand_core::SeedableRng};
// const U32MAX: i64 = u32::MAX as i64;
// const U16MAX: i64 = u16::MAX as i64;

use utils::system::*;
const FILE_PATH: &str = "db_test.hawking";
use otoodb::*;

fn main() -> Result<()> {
    // q = 1 => -1.7320508075688772, q = u32::MAX => 1.7320772365667954
    // let mut prev_fx = decode(get_fx()) / 1000000_f64;
    // let mut curr_fx;
    // for _ in 0..1000000 {
    //     curr_fx = prev_fx + (decode(get_fx()) / 1000000_f64);
    //     prev_fx = curr_fx;
    //     println!("{}", prev_fx);
    // }
    // for _ in 0..200 {

    // let q = rand_chacha::ChaCha20Rng::from_entropy().gen::<u32>();

    // let q = 0b00000000000000000000000000000000 as i64;
    // let x = 2 * q;
    // let a = U32MAX - x;
    // let b = (a / U16MAX) as f64;
    // let c = b * 3f64.sqrt();
    // let c2 = (U16MAX + 1) as f64;
    // // let c2 = ((U32MAX - 2) as f64).sqrt();
    // let d = c / c2;
    // let f = -d;
    // println!("[{}] -> {}", q, f);
    // assert_eq!(q, encode(f));
    // }

    // get_value(fetch(CHAINS::BTC, 0..)) == log( all_sum(delta_tx_time[.] * delta_tx_count[.]) * blocks_owner_key_on_blackbody_count )
    // sum_all_values() == 1 plank
    // highest value chain's block adding moment == all the chains value updating time

    // let a = get_fx(10000);

    // let mut b = U256::default();

    // let b: Vec<String> = a.to_le_bytes().iter().map(|x| format!("{:b}", x)).collect();
    // println!("{:b} -> {:?}", a, b);

    // let _ = DB::open(FILE_PATH, 32, 4)?;
    _otoodb()?;
    Ok(())
}

fn _get_fx(n: usize) -> u32 {
    let mut prev = rand_chacha::ChaCha20Rng::from_entropy().gen::<u32>();
    let mut curr;
    let mut tmp;
    for _ in 0..n {
        loop {
            tmp = rand_chacha::ChaCha20Rng::from_entropy().gen::<u32>();
            if tmp != prev && tmp != 0_u32 && tmp != u32::MAX {
                break;
            }
        }
        curr = prev ^ tmp;
        prev = curr;
    }
    prev
}
fn _rand_bytes32(buf: &mut [u8]) {
    let rand_u8 = || rand_chacha::ChaCha20Rng::from_entropy().gen::<u8>();
    for space in buf.iter_mut() {
        *space = rand_u8();
    }
}

// fn decode(x: u32) -> f64 {
//     let q = x as i64;
//     let x = 2 * q;
//     let a = U32MAX - x;
//     let b = (a / U16MAX) as f64;
//     let c = b * 3f64.sqrt();
//     let c2 = (U16MAX + 1) as f64;
//     // let c2 = ((U32MAX - 2) as f64).sqrt();
//     let d = c / c2;
//     let f = -d;
//     f
// }

// fn encode(fx: f64) -> i64 {
//     let mut fx = -fx;
//     fx *= (U16MAX + 1) as f64;
//     fx /= 3f64.sqrt();
//     fx *= U16MAX as f64;
//     let mut fx = U32MAX - (fx as i64);
//     fx /= 2;
//     fx
// }

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use super::*;

//     use utils::system::err;

//     #[test]
fn _otoodb() -> Result<()> {
    if std::path::Path::new(FILE_PATH).exists() {
        std::fs::remove_file(FILE_PATH)?;
    }
    let mut db = DB::open(FILE_PATH, 32, 4)?;

    let mut packet = Vec::new();
    let mut bytes32 = [0_u8; 32];
    let mut bytes4: [u8; 4];

    for i in 1..=100000u128 {
        loop {
            _rand_bytes32(&mut bytes32);
            bytes4 = _get_fx(2).to_le_bytes();
            // no interrupted
            errextract!(db.define(&bytes32, &bytes4), err::Interrupted => continue);
            break;
        }
        eprint!("\r{} set pushed.", i);
        packet.push((bytes32, bytes4));
    }

    eprintln!();

    // test
    let (mut a, mut b);
    for (i, (bytes32, bytes4)) in packet.into_iter().enumerate() {
        a = db.get(&bytes32)?.unwrap();
        b = db.get(&bytes4)?.unwrap();
        assert_eq!(a, &bytes4);
        assert_eq!(b, &bytes32);
        eprint!("\rpair found: {}", i + 1);
    }

    eprintln!();
    // db.debug();
    db.close();
    // std::fs::remove_file(FILE_PATH)?;
    Ok(())
}
// }
