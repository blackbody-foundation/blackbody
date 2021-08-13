use super::claps::*;
use crate::envs;
pub struct Args<'a, 'b> {
    saved: clap::App<'a, 'b>,
    pub name: &'static str,
}
impl<'a, 'b> Args<'a, 'b> {
    pub fn new() -> Self {
        Self {
            saved: CApp::new()
                .sink()
                .subcommand(SubCommand::with_name("clear").about("clear screen"))
                .subcommand(
                    SubCommand::with_name("echo")
                        .about("echo $env")
                        .arg(Arg::with_name("$env").help("INPUT").required(true).index(1)),
                )
                .subcommand(SubCommand::with_name("quit").about("quit program"))
                .subcommand(
                    SubCommand::with_name("test")
                        .about("controls testing features")
                        .version("1.0")
                        .author("just-do-halee <just.do.halee@gmail.com>")
                        .arg(
                            Arg::with_name("v")
                                .short("v")
                                .multiple(true)
                                .help("Sets the level of verbosity"),
                        )
                        .subcommand(
                            SubCommand::with_name("otoodb")
                                .version("1.0")
                                .about("test one to one set database"),
                        ),
                ),
            name: clap::crate_name!(),
        }
    }
    pub fn matches(&mut self, arguments: Vec<&str>) -> crate::Result<ArgMatches<'a>> {
        Ok(self.saved.get_matches_from_safe_borrow(arguments)?)
    }
    pub fn arg_to_env(&self, args: &ArgMatches<'a>, arg_names: &[&str]) {
        envs::arg_to_env(args, "inner", arg_names); // send verbose to env
    }
}
