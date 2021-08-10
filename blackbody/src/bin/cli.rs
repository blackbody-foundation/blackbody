/*
    .. + cli.rs + ..

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

// use blackbody::cli;
// use blackbody::net;

// use rand::Rng;
// use rand_chacha::{self, rand_core::SeedableRng};
// const U32MAX: i64 = u32::MAX as i64;
// const U16MAX: i64 = u16::MAX as i64;

use utils::system::*;
use utils::types::hash::*;
const FILE_PATH: &str = "test.hawking";
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

    _otoodb()?;
    let _ = DB::open(FILE_PATH, 32, 4, None)?;
    Ok(())
}

fn _gen_bytes4(src: &[u8; 32]) -> [[u8; 4]; 8] {
    // let mut prev = rand_chacha::ChaCha20Rng::from_entropy().gen::<u32>();
    // let mut curr;
    // let mut tmp;
    // for _ in 0..n {
    //     loop {
    //         tmp = rand_chacha::ChaCha20Rng::from_entropy().gen::<u32>();
    //         if tmp != prev && tmp != 0_u32 && tmp != u32::MAX {
    //             break;
    //         }
    //     }
    //     curr = prev ^ tmp;
    //     prev = curr;
    // }
    // prev
    let mut buf: [[u8; 4]; 8] = [[0_u8; 4]; 8];
    for (i, bytes4) in src.chunks(4).enumerate() {
        buf[i].copy_from_slice(bytes4);
    }
    buf
}
/// hashing by self count n - 1
fn _gen_bytes32(buf: &[u8], n: usize) -> [u8; 32] {
    // let rand_u8 = || rand_chacha::ChaCha20Rng::from_entropy().gen::<u8>();
    // for space in buf.iter_mut() {
    //     *space = rand_u8();
    // }
    HashCoverIter256::new(buf)
        .into_iter()
        .take(n)
        .last()
        .unwrap()
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
const NUM_COVERING: usize = 350;
const NUM_PUSHED: u128 = 500000;

fn _otoodb() -> Result<()> {
    if std::path::Path::new(FILE_PATH).exists() {
        std::fs::remove_file(FILE_PATH)?;
    }
    let mut db = DB::open(FILE_PATH, 32, 4, None)?;

    let mut packet = Vec::new();

    let (tx, rx) = crossbeam::channel::bounded::<([u8; 32], [[u8; 4]; 8])>(1024);
    let handle = std::thread::spawn(move || -> ResultSend<()> {
        let mut bytes32: [u8; 32];
        let mut bytes4: [[u8; 4]; 8];
        let mut i: u32 = 0;
        loop {
            bytes32 = _gen_bytes32(&i.to_le_bytes(), NUM_COVERING);
            bytes4 = _gen_bytes4(&bytes32);
            if tx.send((bytes32, bytes4)).is_err() {
                return Ok(());
            }
            i += 1;
        }
    });
    let start = Instant::now();
    let mut timer = Timer::new();
    timer.period = Duration::from_millis(60);

    'out: for i in 1..=NUM_PUSHED {
        'pushed: loop {
            timer.update();
            if let Ok((bytes32, bytes4_list)) = rx.recv() {
                // no interrupted
                for bytes4 in bytes4_list {
                    errextract!(db.define(&bytes32, &bytes4), err::Interrupted => {
                        eprintln!("\n\n{}. ip already exists. {}\n\n", i, Hex(bytes4));
                        continue;
                    });
                    packet.push((bytes32, bytes4));
                    break 'pushed;
                }
                continue;
            } else {
                break 'out;
            }
        }
        if timer.ready || i == NUM_PUSHED {
            let push_per_second = 1.0 / timer.delta.as_secs_f64();
            timer.ready = false;
            eprint!(
                "\r[{}]  {} set pushed.  ({:.0} p/s)   ",
                start.elapsed().as_secs().as_time(),
                i,
                push_per_second,
            );
        }
    }

    drop(rx);
    resultcast!(handle.join().unwrap())?;
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
    db.close()?;
    // std::fs::remove_file(FILE_PATH)?;
    Ok(())
}
// }
