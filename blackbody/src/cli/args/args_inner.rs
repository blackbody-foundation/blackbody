use super::claps::*;

pub struct Args<'a, 'b> {
    saved: clap::App<'a, 'b>,
    pub name: &'static str,
}
impl<'a, 'b> Args<'a, 'b> {
    pub fn new() -> Self {
        Self {
            saved: CApp::new()
                .sink()
                .subcommand(
                    CSubCommand::plain("echo", "echo $env")
                        .arg(Arg::with_name("$env").help("INPUT").required(true).index(1)),
                )
                .subcommand(CSubCommand::plain("clear", "clear screen"))
                .subcommand(CSubCommand::plain("quit", "quit program"))
                .subcommand(CSubCommand::plain("p", "break process"))
                .subcommand(
                    CSubCommand::new("test", "testing features", "1.0")
                        .arg(
                            Arg::with_name("v")
                                .short("v")
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
