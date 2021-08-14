/*
    .. + mod.rs + ..

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

mod etc;
mod stdin;
mod stdout;

pub use utils::system::style::style;

pub use etc::*;
pub use stdin::*;
pub use stdout::*;

use crate::cli::args::ADMIN_NAME;

#[inline]
pub fn print_domain() {
    print!(
        "{} {} ",
        style(ADMIN_NAME).blue().italic().bold(),
        style("✗").dim().bold()
    );
    flush();
}

/// ```no_run
/// let input = String::from("test"); // == program name
/// let out = read_command(&mut input); // stdin() <- 'some1 something2'
/// assert_eq!(out, vec!["test","some1","something2"]);
/// ```
#[inline]
pub fn read_command(prefix: &mut String) -> Vec<&str> {
    let mut buf = String::new();
    readln(&mut buf);
    prefix.push(' ');
    prefix.push_str(&buf);
    prefix.split_whitespace().collect()
}
