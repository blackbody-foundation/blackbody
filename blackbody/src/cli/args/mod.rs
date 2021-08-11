mod claps;

use std::ops::Deref;

use clap::{Arg, ArgMatches, SubCommand};
use claps::CApp;

impl<'a> Default for Args<'a> {
    fn default() -> Self {
        Self {
            matches: CApp::new()
                .push(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file")
                        .takes_value(true),
                )
                .push(
                    Arg::with_name("MODE")
                        .short("m")
                        .long("mode")
                        .help("run only `api` mode or `rpc` mode")
                        .takes_value(true),
                )
                .push(
                    Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Sets the level of verbosity"),
                )
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
        }
    }
}

pub struct Args<'a> {
    matches: ArgMatches<'a>,
}
impl<'a> Deref for Args<'a> {
    type Target = ArgMatches<'a>;
    fn deref(&self) -> &Self::Target {
        &self.matches
    }
}
impl<'a> Args<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
