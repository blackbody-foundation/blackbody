/*
    .. + cmn.rs + ..

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

pub use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    thread, time,
};

pub use actix_web::dev::Server;
pub use crossbeam::channel::TryRecvError;
pub use utils::system::*;

// common in cli
pub use crate::cli::{cat, envs, style};

#[derive(Debug, Clone)]
pub struct Net {
    pub server: Server,
    pub name: &'static str,
}

impl Net {
    pub fn new(name: &'static str, server: Server) -> Self {
        Self { server, name }
    }
}

#[derive(Debug, Clone)]
pub struct ServerList(pub Vec<Net>);

impl IntoIterator for ServerList {
    type Item = Net;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl Deref for ServerList {
    type Target = Vec<Net>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ServerList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[macro_use]
mod cf {
    /// ```no_run
    /// |e| eprintln!("{}", style(e).red()) // term::style
    /// ```
    #[macro_export]
    macro_rules! else_error {
        () => {
            |e| eprintln!("{}", style(e).red())
        };
    }
    pub use else_error;

    /// ```no_run
    /// |$($arg)?| panic!("{}", style($s).red().bold()) // term::style
    /// ```
    /// closure panic! with some text $s.<br>
    /// something_wrong!("someting", _)<br>
    /// something_wrong!("someting")<br>
    /// second argument is useless closure argument.
    #[macro_export]
    macro_rules! something_wrong {
        ($s:expr$(, $arg:tt)?) => {
            |$($arg)?| panic!("{}", style($s).red().bold())
        };
    }
    pub use something_wrong;

    /// "$target --help"
    #[macro_export]
    macro_rules! target_help {
        ($target:expr) => {
            concat!($target, " --help")
        };
    }
    pub use target_help;

    /// static string
    #[macro_export]
    macro_rules! name {
        (COMMAND) => {
            "blackbody "
        };
        (ADMIN) => {
            "blackbody"
        };
        (ICON) => {
            "icon"
        };

        (outter) => {
            "outter"
        };
        (inner) => {
            "inner"
        };

        (restart) => {
            "restart"
        };
        (clear) => {
            "clear"
        };

        (quit) => {
            "quit"
        };
        (p) => {
            "p"
        };

        (echo) => {
            "echo"
        };
        (env) => {
            "$env"
        };

        (INPUT) => {
            "INPUT"
        };
        (TARGET) => {
            "TARGET"
        };

        (verbose:l) => {
            "verbose"
        };
        (verbose:s) => {
            "v"
        };

        (server:l) => {
            "server"
        };
        (server:s) => {
            "s"
        };

        (mode:l) => {
            "mode"
        };
        (mode:s) => {
            "m"
        };

        (API) => {
            "API"
        };
        (RPC) => {
            "RPC"
        };
        (BOTH) => {
            "BOTH"
        };

        (test:l) => {
            "test"
        };
        (test:s) => {
            "t"
        };

        (debug:l) => {
            "debug"
        };
        (debug:s) => {
            "d"
        };

        (SERVER) => {
            "server"
        };
        (ITALIC_ALERT) => {
            "italic_alert"
        };
    }
    pub use name;
}
pub use cf::*;
