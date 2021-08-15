/*
    .. + mod.rs + ..

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

use crate::cmn::*;

pub use console::style;
use console::{Style as Se, Term as Tm};

#[macro_use]
mod etc;
pub use etc::*;

pub struct Term {
    pub stdout: Tm,
    pub stderr: Tm,
    styles: HashMap<&'static str, Se>,
}
impl Term {
    pub fn new() -> Term {
        let mut styles = HashMap::new();
        styles.insert(name!(ADMIN), Se::new().blue().italic().bold());
        styles.insert(name!(ICON), Se::new().dim().bold());
        styles.insert(name!(SERVER), Se::new().cyan().underlined().italic());
        styles.insert(name!(ITALIC_ALERT), Se::new().red().italic().bold());
        Self {
            stdout: Tm::stdout(),
            stderr: Tm::stderr(),
            styles,
        }
    }
    pub fn style(&self, name: &str) -> &Se {
        self.styles
            .get(name)
            .unwrap_or_else(something_wrong!("Invalid style name"))
    }
    pub fn print_domain(&self) {
        let a = self.style(name!(ADMIN));
        let b = self.style(name!(ICON));
        self.stdout
            .write_str(&format!(
                "{} {} ",
                a.apply_to(name!(ADMIN)),
                b.apply_to("✗")
            ))
            .unwrap_or_else(else_error!());
    }
    pub fn read_password(&self) -> String {
        self.stdout.read_secure_line().unwrap_or_else(|e| {
            eprintln!("{}", style(e).red());
            String::new()
        })
    }
    pub fn read_command(&self) -> String {
        let mut command = String::from(name!(COMMAND));
        command.push_str(&self.stdout.read_line().unwrap_or_default());
        command
    }
    pub fn print(&self, s: &str) {
        self.stdout.write_str(s).unwrap_or_else(else_error!());
    }
    pub fn println(&self, s: &str) {
        self.stdout.write_line(s).unwrap_or_else(else_error!());
    }
    pub fn eprint(&self, s: &str) {
        self.stderr.write_str(s).unwrap_or_else(else_error!());
    }
    pub fn eprintln(&self, s: &str) {
        self.stderr.write_line(s).unwrap_or_else(else_error!());
    }
    pub fn clear_all(&self) {
        self.stderr.clear_screen().unwrap_or_else(else_error!());
        self.stdout.clear_screen().unwrap_or_else(else_error!());
    }
}
