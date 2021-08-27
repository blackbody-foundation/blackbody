/*
    .. + outter_conf.rs + ..

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

use crate::{args, name};
pub struct OutterConfig {
    pub reset_mode: bool,
    pub net_mode: String,
}

pub fn get() -> OutterConfig {
    let args_outter = args::outter::new();
    let reset_mode = args_outter.subcommand_name().unwrap_or("") == name!(reset: l);
    let net_mode = args_outter.value_of("mode").unwrap_or_default().to_string();
    OutterConfig {
        reset_mode,
        net_mode,
    }
}
