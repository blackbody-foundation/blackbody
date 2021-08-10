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

use blackbody::cli::Args;

fn main() -> Result<()> {
    let args = Args::new();
    let config = args.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    println!("Using input file: {}", args.value_of("INPUT").unwrap());

    match args.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their args by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(args) = args.subcommand_matches("test") {
        if args.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // _otoodb()?;
    // let _ = DB::open(FILE_PATH, 32, 4, None)?;
    Ok(())
}

fn _gen_bytes4(src: &[u8; 32]) -> [[u8; 4]; 8] {
    let mut buf: [[u8; 4]; 8] = [[0_u8; 4]; 8];
    for (i, bytes4) in src.chunks(4).enumerate() {
        buf[i].copy_from_slice(bytes4);
    }
    buf
}
/// hashing by self count n - 1
fn _gen_bytes32(buf: &[u8], n: usize) -> [u8; 32] {
    HashCoverIter256::new(buf)
        .into_iter()
        .take(n)
        .last()
        .unwrap()
}

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
