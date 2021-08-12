/*
    .. + envs.rs + ..

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

use super::Args;

pub fn arg_to_env(args: &Args, arg_names: &[&str]) {
    for &key in arg_names.iter() {
        std::env::set_var(
            format!("envs_{}", &key),
            args.value_of(&key).unwrap_or_default(),
        );
    }
}

pub fn get_env(key: &str) -> String {
    std::env::var(format!("envs_{}", key)).unwrap_or_default()
}

///```no_run
/// let v = init_verbose!("verbose"); // "clap_env_name"
///```
#[macro_export]
macro_rules! init_verbose {
    ($v:expr) => {
        envs::get_env($v)
            .parse::<u8>()
            .expect("couldn't parse verbose env var.")
    };
}
pub use init_verbose;

///```no_run
/// let v = init_verbose!("verbose");
/// verbose!(v;1:f "this is level{} verbose.", 1); // no line
/// verbose!(v;1: "this is level{} verbose.", 1); // line
///```
#[macro_export]
macro_rules! verbose {
    ($env:expr;$lvl:tt:f $($exp:expr),*) => {
        if $env >= $lvl {
            print!($($exp),*);
        }
    };
    ($env:expr;$lvl:tt: $($exp:expr),*) => {
        if $env >= $lvl {
            println!($($exp),*);
        }
    };
}
pub use verbose;
