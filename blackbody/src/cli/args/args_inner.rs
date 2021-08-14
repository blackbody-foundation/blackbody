use super::claps::*;
// use crate::envs;
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
    // pub fn arg_to_env(&self, args: &ArgMatches<'a>, arg_names: &[&str]) {
    //     envs::arg_to_env(args, "inner", arg_names); // send verbose to env
    // }
}
