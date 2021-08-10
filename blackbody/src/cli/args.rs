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
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true) // required true => can be unwraped
                        .index(1),
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
                        .version("1.3")
                        .author("just-do-halee <just.do.halee@gmail.com>")
                        .arg(
                            Arg::with_name("debug")
                                .short("d")
                                .help("print debug information verbosely"),
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
