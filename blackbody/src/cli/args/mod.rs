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

mod claps;

use std::ops::Deref;

use clap::{Arg, ArgMatches, SubCommand};
use claps::CApp;

impl<'a> Default for Args<'a> {
    fn default() -> Self {
        let args = Self {
            matches: CApp::new()
                .push(
                    Arg::with_name("mode")
                        .short("m")
                        .long("mode")
                        .value_name("API/RPC")
                        .help("run only `api` mode or `rpc` mode")
                        .takes_value(true),
                )
                .set_verbose("verbose")
                .sink()
                .subcommand(
                    SubCommand::with_name("test")
                        .about("controls testing features")
                        .version("1.0")
                        .author("just-do-halee <just.do.halee@gmail.com>")
                        .arg(
                            Arg::with_name("debug")
                                .short("d")
                                .long("debug")
                                .help("print debug information verbosely"),
                        )
                        .arg(
                            Arg::with_name("otoodb")
                                .short("o")
                                .long("otoodb")
                                .help("test one to one set database"),
                        ),
                )
                .get_matches(),
        };
        super::envs::arg_to_env(&args, &["verbose"]); // send verbose to env
        args
    }
}

pub struct Args<'a> {
    matches: ArgMatches<'a>,
}
impl<'a> Args<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> Deref for Args<'a> {
    type Target = ArgMatches<'a>;
    fn deref(&self) -> &Self::Target {
        &self.matches
    }
}
