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

pub use console::{style, Key, Style};
use console::{Style as Se, Term as Tm};

#[macro_use]
mod etc;
pub use etc::*;

const STACK_SIZE: usize = 16;

pub struct Term {
    locked: bool,
    pub stdout: Tm,
    pub stderr: Tm,
    styles: HashMap<&'static str, Se>,
    pub stack: CommandStack,
}

#[allow(dead_code)]
impl Term {
    pub fn new() -> Term {
        let mut styles = HashMap::new();
        styles.insert(name!(ADMIN), Se::new().blue().italic().bold());
        styles.insert(name!(ICON), Se::new().dim().bold());
        styles.insert(name!(SERVER), Se::new().cyan().underlined().italic());
        styles.insert(name!(ITALIC_ALERT), Se::new().red().italic().bold());
        styles.insert(name!(DIM), Se::new().dim());
        let stack = CommandStack::new(STACK_SIZE);
        Self {
            stdout: Tm::stdout(),
            stderr: Tm::stderr(),
            styles,
            stack,
            locked: false,
        }
    }
    pub fn init(&mut self) {
        self.stdout.set_title(name!(TITLE));
        self.clear_all();
        self.println("");
        let color = self.style(name!(DIM));
        self.println(cat!(
            "{}",
            color.apply_to(align_center!(self.stdout, name!(WELCOME1)))
        ));
        self.println(cat!(
            "{}",
            style(align_center!(self.stdout, name!(WELCOME2)))
                .bright()
                .bold()
        ));
        self.println(cat!(
            "{}",
            color.apply_to(align_center!(self.stdout, name!(WELCOME3)))
        ));
        self.println("");
    }
    pub fn lock(&mut self) {
        self.locked = true;
    }
    pub fn unlock(&mut self) {
        self.locked = false;
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
    /// stacked
    pub fn read_command(&mut self) -> String {
        let command = self.stdout.read_line().unwrap_or_default();
        self.stack.push(&command);
        command
    }
    pub fn move_cursor_left(&self, n: usize) {
        self.stdout
            .move_cursor_left(n)
            .unwrap_or_else(else_error!());
    }
    pub fn move_cursor_right(&self, n: usize) {
        self.stdout
            .move_cursor_right(n)
            .unwrap_or_else(else_error!());
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
    pub fn clear_line(&self) {
        self.stdout.clear_line().unwrap_or_else(else_error!());
    }
    pub fn clear_chars(&self, n: usize) {
        self.stdout.clear_chars(n).unwrap_or_else(else_error!());
    }
    pub fn read_key(&self) -> Key {
        self.stdout.read_key().unwrap_or(Key::Unknown)
    }
    /// basic terminal implement.<br>
    /// domain_name must be "name " <- one whitespace added.
    pub fn base_loop(&mut self, domain_name: &str) -> String {
        let mut command_stack = String::new();
        let command: String;
        self.stack.reset_ptr();
        loop {
            if self.locked {
                continue;
            }
            match self.read_key() {
                Key::ArrowUp => {
                    let traverse = self.stack.traverse_up();
                    if let Some(prev_command) = traverse {
                        self.clear_chars(command_stack.len());
                        self.print(&prev_command);
                        command_stack = prev_command;
                    }
                }
                Key::ArrowDown => match self.stack.traverse_down() {
                    Some(next_command) => {
                        self.clear_chars(command_stack.len());
                        self.print(&next_command);
                        command_stack = next_command;
                    }
                    None => {
                        self.clear_chars(command_stack.len());
                        command_stack.clear();
                    }
                },
                Key::Char(c) => {
                    self.print(&c.to_string());
                    command_stack.push(c);
                }
                Key::Backspace if !command_stack.is_empty() => {
                    self.clear_chars(1);
                    command_stack.pop();
                }
                Key::Enter => {
                    let mut admin = String::from(domain_name);
                    admin.push_str(&command_stack);
                    command = admin;
                    self.println("");
                    self.stack.push(&command_stack);
                    break;
                }
                _ => {}
            }
        }
        command
    }
}
