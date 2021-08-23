use super::claps::*;
use crate::envs;

impl<'a> Default for Args<'a> {
    fn default() -> Self {
        let args = Self {
            matches: CApp::new()
                .push(
                    Arg::with_name(name!(mode: l))
                        .short(name!(mode: s))
                        .long(name!(mode: l))
                        .value_name(name!(TARGET))
                        .help(concat!(
                            "run only `",
                            name!(API),
                            "` mode or `",
                            name!(RPC),
                            "` mode"
                        ))
                        .possible_values(&[name!(API), name!(RPC)])
                        .takes_value(true),
                )
                .push(
                    Arg::with_name(name!(mnemonic: l))
                        .short(name!(mnemonic: s))
                        .long(name!(mnemonic: l))
                        .value_name(name!(MNEMONIC))
                        .help(name!(FromMnemonic))
                        .takes_value(true),
                )
                .set_verbose(name!(verbose: l))
                .sink()
                .subcommand(
                    CSubCommand::new(name!(test: l), "testing features", "1.0").arg(
                        Arg::with_name(name!(debug: l))
                            .short(name!(debug: s))
                            .long(name!(debug: l))
                            .help("print debug information verbosely"),
                    ),
                )
                .subcommand(
                    CSubCommand::new(name!(reset: l), "reset account", "1.0")
                        .help(name!(ForgotPassword)),
                )
                .get_matches(),
        };
        envs::arg_to_env(&args, name!(outter), &[name!(verbose: l)]); // send verbose to env
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
