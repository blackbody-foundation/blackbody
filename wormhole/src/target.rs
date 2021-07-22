/*
    .. + process.rs + ..

    Copyright (C) 2021 Hwakyeom Kim(=just-do-halee)

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

use super::cmn::*;

pub struct Target {
    otoodb: DB,
    pub file_path: PathBuf,
    pub src_bytes_size: LS,
    pub dst_bytes_size: LS,
}

impl Target {
    pub fn new(file_path: &str, otoodb: DB) -> Self {
        let (_, src_bytes_size, dst_bytes_size) = otoodb.get_info();
        let file_path = pathy!(file_path);
        Self {
            otoodb,
            file_path,
            src_bytes_size,
            dst_bytes_size,
        }
    }
}
