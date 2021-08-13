/*
    .. + verbose.rs + ..

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

///```no_run
/// let v = init!("envs", "verbose"); // ("env_prefix", "rest_of_name")
///```
#[macro_export]
macro_rules! init {
    ($e:expr, $v:expr) => {
        std::env::var(format!("{}_{}", $e, $v))
            .unwrap_or_default()
            .parse::<u8>()
            .expect("couldn't parse verbose env var.")
    };
}
pub use init;

///```no_run
/// let v = init!("envs", "verbose");
/// einfo!(v;1:f "this is level{} verbose.", 1); // no line
/// einfo!(v;1: "this is level{} verbose.", 1); // line
///```
#[macro_export]
macro_rules! einfo {
    ($env:expr;$lvl:tt:f $($exp:expr),*) => {
        if $env >= $lvl {
            eprint!($($exp),*);
        }
    };
    ($env:expr;$lvl:tt: $($exp:expr),*) => {
        if $env >= $lvl {
            eprintln!($($exp),*);
        }
    };
}
pub use einfo;

///```no_run
/// let v = init!("envs", "verbose");
/// info!(v;1:f "this is level{} verbose.", 1); // no line
/// info!(v;1: "this is level{} verbose.", 1); // line
///```
#[macro_export]
macro_rules! info {
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
pub use info;
