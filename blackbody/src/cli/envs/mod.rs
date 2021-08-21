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

mod load;
pub use load::Envs;

mod config;

use clap::ArgMatches;

pub fn arg_to_env(args: &ArgMatches, env_prefix: &str, arg_names: &[&str]) {
    for key in arg_names.iter() {
        std::env::set_var(
            std::ffi::OsString::from(format!("{}_{}", env_prefix, key)),
            args.value_of(key).unwrap_or_default(),
        );
    }
}

pub use utils::system::verbose;
