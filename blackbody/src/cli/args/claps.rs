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
    pub fn push(self, arg: Arg<'a, 'b>) -> Self {
        Self(self.0.arg(arg))
    }
    pub fn sink(self) -> App<'a, 'b> {
        self.0
    }
}
