/*
    .. + types.rs + ..

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

/// ```no_run
/// SelItem("for display", "for result")
/// ```
pub struct SelItem<T: AsRef<str>>(pub T, pub T);

impl<T: AsRef<str>> SelItem<T> {
    pub fn display(&self) -> &str {
        self.0.as_ref()
    }
    pub fn result(&self) -> &str {
        self.1.as_ref()
    }
}

/// key name & help list , |any key, result value| { return (true, String) } if `true` -> `return`
pub type OtherKeys<'a> = (
    &'a [&'a str],
    Box<dyn Fn(console::Key, String) -> (bool, String)>,
);

/// encrypt
pub use vep::{Digester, Vep};

pub struct PasswordHasher;
impl Digester for PasswordHasher {
    fn digest(&mut self, bytes: &[u8]) -> Vec<u8> {
        blake3::hash(bytes).as_bytes().to_vec()
    }
}

// option structure
use super::name;
use optionee::optionee;

optionee! {
    pub TermOption {
        Password {
            max_opportunity: u8 [=] 3, name!(ForgotPassword)
            encrypt: bool [=] false
            min_length: u8 [>] 7, "password must be more than 8 lengths bytes."
            max_length: u8 [<] 21, "password must be less than 20 lengths bytes."
        }
    }
}

pub use optionees::TermOption::Password as TermPassword;
