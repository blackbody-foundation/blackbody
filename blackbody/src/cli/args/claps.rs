/*
    .. + claps.rs + ..

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

pub use crate::name;

pub use clap::{AppSettings, Arg, ArgMatches, SubCommand};
pub use std::ops::Deref;

use clap::{crate_authors, crate_description, crate_version, App};

pub const VERBOSE_DEFAULT: &str = "1";
pub const VERBOSE_HELP: &str = "Sets the level of verbosity 0 to 3";

pub const ADMIN_NAME: &str = name!(ADMIN);

/// ## Custom Clap App
pub struct CApp<'a, 'b>(App<'a, 'b>);

impl<'a, 'b> CApp<'a, 'b> {
    pub fn new() -> Self {
        Self(
            App::new(ADMIN_NAME)
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .help_message("")
                .version_message("")
                .setting(AppSettings::DisableHelpSubcommand),
        )
    }
    /// you can change some of default values as this const variables. (claps.rs)
    pub fn set_verbose(self, env_name: &'static str) -> Self {
        self.push(
            Arg::with_name(env_name)
                .short(name!(verbose: s))
                .default_value(VERBOSE_DEFAULT)
                .takes_value(true)
                .validator(match_validator!([ name!(verbose: s) ] "0", "1", "2", "3"))
                .help(VERBOSE_HELP),
        )
    }
    pub fn push(self, arg: Arg<'a, 'b>) -> Self {
        Self(self.0.arg(arg))
    }
    pub fn sink(self) -> App<'a, 'b> {
        self.0
    }
}

#[allow(non_snake_case)]
pub mod CSubCommand {
    use super::*;
    pub fn new<'a, 'b>(name: &'a str, about: &'a str, version: &'a str) -> App<'a, 'b> {
        SubCommand::with_name(name)
            .setting(AppSettings::DisableHelpSubcommand)
            .about(about)
            .version(version)
            .author(crate_authors!())
            .help_message("")
            .version_message("")
    }
    pub fn plain<'a, 'b>(name: &'a str, about: &'a str) -> App<'a, 'b> {
        SubCommand::with_name(name)
            .settings(&[
                AppSettings::DisableHelpFlags,
                AppSettings::DisableVersion,
                AppSettings::DisableHelpSubcommand,
            ])
            .help_message(about)
            .about(about)
    }
}

#[macro_export]
macro_rules! match_validator {
    ([$name:expr] $($val:pat),+) => {
        |v: String| -> Result<(), String> {
            match v.as_ref() {
                $(
                    $val => return Ok(()),
                )+
                _ => Err(format!("{} --help", $name))
            }
        }

    };
}
pub use match_validator;
