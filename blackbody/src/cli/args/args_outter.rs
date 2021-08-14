use super::claps::*;
use crate::envs;

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
                    CSubCommand::new("test", "testing features", "1.0").arg(
                        Arg::with_name("debug")
                            .short("d")
                            .long("debug")
                            .help("print debug information verbosely"),
                    ),
                )
                .get_matches(),
        };
        envs::arg_to_env(&args, "outter", &["verbose"]); // send verbose to env
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
