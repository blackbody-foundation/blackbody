/*
    .. + config.rs + ..

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

use std::ffi::OsString;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    readed: bool,
    pub one_shot: OneShot,
    pub env: Vec<(OsString, OsString)>,
}
impl Config {
    pub fn readed(&mut self) {
        self.readed = true;
    }
    pub fn is_empty(&self) -> bool {
        self.readed
    }
    pub fn drop(self) {}
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OneShot {
    pub lang: String,
    pub hd_dirs: Vec<String>,
}
