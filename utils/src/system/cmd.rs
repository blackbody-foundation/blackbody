/*
    .. + cmd.rs + ..

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

#[macro_export]
macro_rules! cmd {
    (@env $s:literal) => {
        &std::env::var($s).unwrap_or(String::new())
    };
    (@arg ($c:expr) () ($s:literal)) => {
        $c.arg($s)
    };
    (@arg ($c:expr) ('set_dir) ($s:literal)) => {
        $c.current_dir($s)
    };
    (@arg ($c:expr) ('set_dir_env) ($s:literal)) => {
        $c.current_dir(cmd!(@env $s))
    };
    (@arg ($c:expr) ('env) ($s:literal)) => {
        $c.arg(cmd!(@env $s))
    };
    ($s0:literal $($($env:lifetime)?$s:literal)*) => {
        {
            let mut c = std::process::Command::new($s0);
            $(
                cmd!(@arg (c) ($($env)?) ($s));
            )*
            eprintln!("{:?}", &c);
                c.spawn()
                .unwrap_or_else(|e| panic!("failed to execute: {}", e))
                .wait()
                .unwrap_or_else(|e| panic!("failed to wait on: {}", e))
        }
    };
}

pub use cmd;
