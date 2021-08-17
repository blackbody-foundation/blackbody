use super::claps::*;

pub struct Args<'a, 'b> {
    saved: clap::App<'a, 'b>,
    pub name: &'static str,
}
impl<'a, 'b> Args<'a, 'b> {
    pub fn new() -> Self {
        Self {
            saved: CApp::new()
                .push(
                    Arg::with_name(name!(server: l))
                        .short(name!(server: s))
                        .long(name!(server: l))
                )
                .sink()
                .subcommand(
                    CSubCommand::plain(name!(echo), concat!(name!(echo), " ", name!(env))).arg(
                        Arg::with_name(name!(env))
                            .help(name!(INPUT))
                            .required(true)
                            .index(1),
                    ),
                )
                .subcommand(CSubCommand::plain(name!(clear), "clear screen"))
                .subcommand(CSubCommand::plain(name!(quit), "quit program"))
                .subcommand(CSubCommand::plain(name!(p), "break process"))
                .subcommand(
                    CSubCommand::new(name!(restart), "restart network", "")
                        .setting(AppSettings::DisableVersion)
                        .arg(
                            Arg::with_name(name!(verbose: s))
                                .short(name!(verbose: s))
                                .multiple(true)
                                .help("Sets the level of verbosity"),
                        )
                        .arg(
                            Arg::with_name(name!(TARGET))
                                .help(concat!(name!(API), " | ", name!(RPC), " | ", name!(BOTH)))
                                .required(true)
                                .index(1)
                                .validator(match_validator!([ name!(TARGET) ] name!(API), name!(RPC), name!(BOTH))),
                        ),
                )
                .subcommand(
                    CSubCommand::new(name!(stop), "stop network", "")
                        .setting(AppSettings::DisableVersion)
                        .arg(
                            Arg::with_name(name!(verbose: s))
                                .short(name!(verbose: s))
                                .multiple(true)
                                .help("Sets the level of verbosity"),
                        )
                        .arg(
                            Arg::with_name(name!(TARGET))
                                .help(concat!(name!(API), " | ", name!(RPC), " | ", name!(BOTH)))
                                .required(true)
                                .index(1)
                                .validator(match_validator!([ name!(TARGET) ] name!(API), name!(RPC), name!(BOTH))),
                        ),
                )
                .subcommand(
                    CSubCommand::new(name!(test: l), "testing features", "1.0")
                        .arg(
                            Arg::with_name(name!(verbose: s))
                                .short(name!(verbose: s))
                                .multiple(true)
                                .help("Sets the level of verbosity"),
                        )
                        .subcommand(
                            CSubCommand::new("otoodb", "test one to one set database", "1.0").arg(
                                Arg::with_name("delete")
                                    .short("d")
                                    .help("| (reset and delete mode)"),
                            ),
                        ),
                ),
            name: clap::crate_name!(),
        }
    }
    pub fn matches(&mut self, arguments: Vec<&str>) -> crate::Result<ArgMatches<'a>> {
        Ok(self.saved.get_matches_from_safe_borrow(arguments)?)
    }
}
