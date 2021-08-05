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
use utils::types::bytes::U256;
// const U32MAX: i64 = u32::MAX as i64;
// const U16MAX: i64 = u16::MAX as i64;

use utils::system::*;
const FILE_PATH: &str = "test2";
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
    Ok(())
}

fn get_fx(n: usize) -> u32 {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use utils::types::bytes::ByteOrder;

    use super::*;

    use utils::system::err;

    #[test]
    fn otoodb() -> Result<()> {
        if std::path::Path::new(FILE_PATH).exists() {
            std::fs::remove_file(FILE_PATH)?;
        }
        let mut db = DB::open(FILE_PATH, 32, 4)?;
        db.bst.byte_order = ByteOrder::BigEndian;

        let mut packet = HashMap::new();
        for i in 1..=1000000u128 {
            packet.insert(U256::from(i), get_fx(2));
        }

        let mut le_bytes = [0_u8; 32];

        let vec = packet.clone();

        for p in vec.iter() {
            p.0.to_big_endian(&mut le_bytes);
            errextract!(db.define(&le_bytes, &p.1.to_be_bytes()), err::Interrupted => { packet.remove(p.0); });
        }
        drop(vec);

        let (mut a, mut b);
        for p in packet {
            p.0.to_big_endian(&mut le_bytes);
            a = db.get(&le_bytes)?.unwrap();

            let le_bytes2 = p.1.to_be_bytes();
            b = db.get(&le_bytes2)?.unwrap();

            assert_eq!(a, &le_bytes2);
            assert_eq!(b, &le_bytes);
        }

        // db.debug();
        db.close();
        // std::fs::remove_file(FILE_PATH)?;
        Ok(())
    }
}
