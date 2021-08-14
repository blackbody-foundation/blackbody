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

// use blackbody::cli;
// use blackbody::net;

// use rand::Rng;
// use rand_chacha::{self, rand_core::SeedableRng};
// const U32MAX: i64 = u32::MAX as i64;
// const U16MAX: i64 = u16::MAX as i64;

mod cmn;
use std::io::Write;

use cmn::*;

mod cli;
mod net;

use cli::*;

fn main() -> Result<()> {
    let args_outter = args::outter::new();

    let sl = net::run(args_outter.value_of("mode").unwrap_or_default());

    let mut args_inner = args::inner::new();

    loop {
        let mut input = String::new();
        print!(
            "{} {} ",
            term::style(args_inner.name).blue().italic().bold(),
            term::style("✗").dim().bold()
        );
        std::io::stdout()
            .flush()
            .unwrap_or_else(|e| eprintln!("{}", e));
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        input = format!("{} {}", args_inner.name, input);

        let arguments = input.split_whitespace().collect();

        let matches = args_inner.matches(arguments);
        match matches {
            Ok(args) => match args.subcommand() {
                ("clear", Some(_)) => eprint!("\r\x1b[2J\r\x1b[H"),
                ("quit", Some(_)) => break,
                ("test", Some(m)) => {
                    if let Some(mm) = m.subcommand_matches("otoodb") {
                        let test_mode = mm.is_present("delete");
                        otoodb(test_mode, m.occurrences_of("v") as u8)?;
                    }
                }
                ("echo", Some(m)) => {
                    let env_name = m.value_of("$env").unwrap_or("");
                    eprintln!(
                        "{}",
                        std::env::var(env_name).unwrap_or_else(|_| "[None]".to_string())
                    );
                }
                _ => eprintln!("* Invalid command"),
            },
            Err(e) => eprintln!("{}\n", e),
        }
    }

    net::stop(sl);

    Ok(())
}

fn _gen_bytes8(src: &[u8; 64]) -> [[u8; 8]; 8] {
    let mut buf: [[u8; 8]; 8] = [[0_u8; 8]; 8];
    for (i, bytes8) in src.chunks(8).enumerate() {
        buf[i].copy_from_slice(bytes8);
    }
    buf
}
/// hashing by self count n - 1
fn _gen_bytes64(buf: &[u8], n: usize) -> [u8; 64] {
    sha512::HashCoverIter::new(buf)
        .into_iter()
        .take(n)
        .last()
        .unwrap()
}

use otoodb::*;
use utils::types::hash::*;

const FILE_PATH: &str = "test.hawking";
const NUM_COVERING: usize = 32;
const NUM_PUSHED: u128 = 50000;

fn otoodb(delete: bool, verbose: u8) -> Result<()> {
    let path = if delete { "./__null__" } else { FILE_PATH };

    let exist = std::path::Path::new(path).exists();
    if delete && exist {
        std::fs::remove_file(path)?;
    }

    let mut db = DB::open(path, 64, 8, Some(Flags { verbose }))?;

    if !exist {
        let mut packet = Vec::new();

        let (tx, rx) = crossbeam::channel::bounded::<([u8; 64], [[u8; 8]; 8])>(1024);
        let handle = std::thread::spawn(move || -> ResultSend<()> {
            let mut bytes64: [u8; 64];
            let mut bytes8: [[u8; 8]; 8];
            let mut i: u64 = 0;
            loop {
                bytes64 = _gen_bytes64(&i.to_le_bytes(), NUM_COVERING);
                bytes8 = _gen_bytes8(&bytes64);
                if tx.send((bytes64, bytes8)).is_err() {
                    return Ok(());
                }
                i += 1;
            }
        });
        let mut timer = Timer::new();
        timer.period = Duration::from_millis(60);

        let start = Instant::now();

        'out: for i in 1..=NUM_PUSHED {
            'pushed: loop {
                timer.update();
                if let Ok((bytes64, bytes8_list)) = rx.recv() {
                    // no interrupted
                    for bytes8 in bytes8_list {
                        errextract!(db.define(&bytes64, &bytes8), err::Interrupted => {
                            eprintln!("\n\n{}. ip already exists. {}\n\n", i, Hex(bytes8));
                            continue;
                        });
                        packet.push((bytes64, bytes8));
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
        for (i, (bytes64, bytes8)) in packet.into_iter().enumerate() {
            a = db.get(&bytes64)?.unwrap();
            b = db.get(&bytes8)?.unwrap();
            assert_eq!(a, &bytes8);
            assert_eq!(b, &bytes64);
            eprint!("\rpair found: {}", i + 1);
        }
        eprintln!();
    }
    // db.debug();
    db.close()?;
    if delete {
        std::fs::remove_file(path)?;
    }
    Ok(())
}
