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
pub use crate::cli::{cat, envs, style, Key, Style};

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

impl ServerList {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
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

    /// ```no_run
    /// let padded = align_center!(term.stdout, "text");
    /// let padded = align_center!(term.stderr, "text");
    /// ```
    #[macro_export]
    macro_rules! align_center {
        ($term:expr, $s:expr) => {{
            console::pad_str(
                $s,
                $term.size().1 as usize,
                console::Alignment::Center,
                None,
            )
        }};
    }
    pub use align_center;

    /// static string
    #[macro_export]
    macro_rules! name {
        (NotMatching) => {
            "not matched."
        };
        (ForgotPassword) => {
            "if you don't really remember your own password, please consider to restart with --reset flag. it just remove envs.locked file, not a wallet."
        };
        (FileIsNotWritten) => {
            "unexpected error occurs. the file is not written."
        };
        (UnexpectedRuntime) => {
            "Because of unexpected panic occured previously, So runtime thread is already occupied. Please restart and clean your threads
            [example]: in linux, command `top` and get the PID, command `kill -9 <PID>`"
        };
        (TITLE) => {
            "BlackBody Node"
        };
        (WELCOME1) => {
            "/*************************************/"
        };
        (WELCOME2) => {
            "   W e l c o m e   B l a c k B o d y   "
        };
        (WELCOME3) => {
            "/*************************************/"
        };
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
        (stop) => {
            "stop"
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

        (reset:l) => {
            "reset"
        };
        (reset:s) => {
            "r"
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
        (DIM) => {
            "icon"
        };
    }
    pub use name;
}
pub use cf::*;
