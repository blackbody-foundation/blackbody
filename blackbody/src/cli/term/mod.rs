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

mod types;
pub use types::*;

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
    pub fn reset_screen(&mut self) {
        self.clear_all_entirely();
        self.init();
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
    pub fn read_password(&self, encrypt: bool) -> String {
        match self.stdout.read_secure_line() {
            Ok(v) => {
                let mut pass = v.nfkd().to_string();
                if encrypt {
                    pass = hex::encode(Vep(PasswordHasher).expand(pass.as_bytes()));
                }
                pass
            }
            Err(e) => {
                eprintln!("{}", style(e).red());
                String::new()
            }
        }
    }

    /// if `f` is (true, _) then break the loop<br>
    /// f(password: String) -> (bool, T)<br>
    /// `T` will be out this entire function
    pub fn read_password_op<T, F: Fn(String) -> (bool, T)>(
        &self,
        option: &TermPassword,
        f: F,
    ) -> T {
        let max_opportunity = *option.max_opportunity.get_value();
        let encrypt = *option.encrypt.get_value();

        for _ in 0..max_opportunity {
            let mut password = loop {
                let password = self.read_password(false); // get password

                if let Err(e) = option.max_length.check(password.len() as u8) {
                    self.eprintln(cat!("{}", e));
                    continue;
                }
                if let Err(e) = option.min_length.check(password.len() as u8) {
                    self.eprintln(cat!("{}", e));
                    continue;
                }
                break password;
            };
            if encrypt {
                password = hex::encode(Vep(PasswordHasher).expand(password.as_bytes()));
            }

            let (check, output) = f(password);

            if check {
                return output;
            }

            self.eprintln(name!(NotMatching));
        }
        something_wrong!(name!(ForgotPassword))()
    }
    /// stacked
    pub fn read_command(&mut self) -> String {
        let command = self.stdout.read_line().unwrap_or_default();
        self.stack.push(&command);
        command
    }
    // nfkd encoded
    pub fn read_line(&mut self) -> String {
        self.stdout
            .read_line()
            .unwrap_or_default()
            .nfkd()
            .to_string()
    }
    pub fn move_cursor_up(&self, n: usize) {
        self.stdout.move_cursor_up(n).unwrap_or_else(else_error!());
    }
    pub fn move_cursor_down(&self, n: usize) {
        self.stdout
            .move_cursor_down(n)
            .unwrap_or_else(else_error!());
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
    pub fn clear_all_entirely(&self) {
        self.stderr
            .clear_to_end_of_screen()
            .unwrap_or_else(else_error!());
        self.stdout
            .clear_to_end_of_screen()
            .unwrap_or_else(else_error!());
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
    pub fn hide_cursor(&self) {
        self.stdout.hide_cursor().unwrap_or_else(else_error!());
        self.stderr.hide_cursor().unwrap_or_else(else_error!());
    }
    pub fn show_cursor(&self) {
        self.stdout.show_cursor().unwrap_or_else(else_error!());
        self.stderr.show_cursor().unwrap_or_else(else_error!());
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
    pub fn get_select<T: AsRef<str>>(
        &mut self,
        list: &[SelItem<T>],
        other_key: Option<OtherKeys>,
    ) -> String {
        let mut number = 0;
        let max_number = list.len() - 1;
        let none_style = Style::new().white();
        let selected_style = Style::new().on_magenta();
        self.eprintln("\n");
        // render all
        for (i, item) in list.iter().enumerate() {
            self.eprintln(cat!(
                "  {}. {}  ",
                i + 1,
                none_style.apply_to(item.display())
            ));
        }
        self.eprintln("\n");
        self.move_cursor_up(max_number + 3);
        // set closure
        let render_item = |number: usize, style: &Style| {
            self.eprint(cat!(
                "\r  {}. {}  ",
                number + 1,
                style.apply_to(list[number].display())
            ));
        };
        if let Some((help, _)) = other_key {
            self.eprintln("\n\n\n\n\n");
            for other in help.iter() {
                self.eprintln(other.as_ref());
            }
            self.move_cursor_up(help.len() + 6);
        }
        loop {
            render_item(number, &selected_style);
            match self.read_key() {
                Key::ArrowUp => {
                    if number > 0 {
                        render_item(number, &none_style);
                        self.move_cursor_up(1);
                        number -= 1;
                    }
                }
                Key::ArrowDown => {
                    if number < max_number {
                        render_item(number, &none_style);
                        self.move_cursor_down(1);
                        number += 1;
                    }
                }
                Key::Enter => {
                    self.eprintln("");
                    return String::from(list[number].result());
                }
                c => {
                    if let Some((_, ref f)) = other_key {
                        let result = f(c, String::from(list[number].result()));
                        if result.0 {
                            return result.1;
                        }
                    }
                }
            }
        }
    }
}
