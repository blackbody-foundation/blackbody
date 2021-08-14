/*
    .. + etc.rs + ..

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
use super::*;

#[allow(dead_code)]
#[inline]
pub fn hide_cursor() {
    print!("\x1b[?25l"); // hide cursor
    flush();
}
#[inline]
pub fn show_cursor() {
    print!("\x1b[?25h"); // hide cursor
    flush();
}
#[inline]
pub fn clear() {
    print!("\r\x1b[2J\r\x1b[H"); // hide cursor
    flush();
}

///```no_run
/// if let Ok(_) = $rx.try_recv() {
///     return Ok(());
/// }
///```
#[macro_export]
macro_rules! check_rx_return {
    ($rx:expr) => {
        if let Ok(_) = $rx.try_recv() {
            return Ok(());
        }
    };
}
pub use check_rx_return;
