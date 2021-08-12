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

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

const VERBOSE_DEFAULT: &str = "1";
const VERBOSE_HELP: &str = "Sets the level of verbosity 0 to 3";

/// ## Custom Clap App
pub struct CApp<'a, 'b>(App<'a, 'b>);

impl<'a, 'b> CApp<'a, 'b> {
    pub fn new() -> Self {
        Self(
            App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!()),
        )
    }
    /// you can change some of default values as this const variables. (claps.rs)
    pub fn set_verbose(self, env_name: &'static str) -> Self {
        self.push(
            Arg::with_name(env_name)
                .short("v")
                .long(env_name)
                .default_value(VERBOSE_DEFAULT)
                .global(true)
                .takes_value(true)
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
