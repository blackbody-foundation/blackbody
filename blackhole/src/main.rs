/*
    .. + main.rs + ..

    Copyright (C) 2021 Hwakyeom Kim(=just-do-halee)

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

// blackhole (file) or (file.cccs)
// cccs = collapsed and compressed core star

// use std::{
//     self, env,
//     io::{ErrorKind, Result},
//     path::Path,
// };

// use utils::{fs::File, u256::U256};
// use wormhole;

// const EXTENSION: &str = "cccs";

// #[tokio::main]
// async fn main() -> Result<()> {
//     let mut args = env::args().skip(1);

//     let in_path = args.next().unwrap();
//     let out_path = Path::new(&in_path).with_extension(EXTENSION);

//     let mut f_in = File::open(in_path).await?;
//     let mut f_out = File::create(out_path).await?;

//     let mut buf = U256::new();
//     let mut total_bytes: usize = 0;

//     loop {
//         let num_read = match f_in.read(buf.as_mut_u8()).await {
//             Ok(0) => break,
//             Ok(x) => x,
//             Err(_) => break,
//         };
//         total_bytes += num_read;
//         // Wormhole(&mut buffer)
//         eprint!("\r{}", total_bytes);
//         if let Err(e) = f_out.write_all(buf.as_u8_slice(num_read)).await {
//             if e.kind() == ErrorKind::BrokenPipe {
//                 break;
//             }
//             return Err(e);
//         }
//     }

//     Ok(())
// }

// let tq = ITQ::new();
// let start = Instant::now();
// for index in 0..100000 {
//     tq.push(Box::new(move || {
//         for _ in 0..5 {
//             print!("\r{}", index);
//         }
//     }));
// }
// tq.drop();
// let duration = start.elapsed();
// println!("\nDuration: {:?}", duration);

use utils::result::*;
use wormhole::DB;
fn main() -> Result<()> {
    let db = DB::open("/Volumes/programs/code/blackchain/test", 4, 32)?;
    println!("{:#?}", db.file.header);
    db.close();
    Ok(())
}
