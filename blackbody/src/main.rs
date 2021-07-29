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
// use rand::Rng;
// use rand_chacha::{self, rand_core::SeedableRng};
// const U32MAX: i64 = u32::MAX as i64;
// const U16MAX: i64 = u16::MAX as i64;

fn main() {
    // // q = 1 => -1.7320508075688772, q = u32::MAX => 1.7320772365667954
    // for _ in 0..200 {
    //     let q = rand_chacha::ChaCha20Rng::from_entropy().gen::<u32>() as i64;
    //     let x = 2 * q;
    //     let a = U32MAX - x;
    //     let b = (a / U16MAX) as f64;
    //     let c = b * 3f64.sqrt();
    //     let c2 = (U16MAX + 1) as f64;
    //     // let c2 = ((U32MAX - 2) as f64).sqrt();
    //     let d = c / c2;
    //     let f = -d;
    //     println!("[{}] => ( {} )", q, f);
    // }
}
