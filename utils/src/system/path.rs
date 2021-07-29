/*
    .. + path.rs + ..

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

pub use std::path::{Path, PathBuf};

/// &str change into PathBuf
#[macro_export]
macro_rules! pathy {
    ($path_str:expr) => {
        PathBuf::from($path_str);
    };
}
pub use pathy;

/// return Result<&str>
///```rust
/// match $path_buf.to_str() {
///     Some(path) if $path_buf.is_file() => Ok(path),
///     Some(path) => errbang!(err::FileNotFound, "cannot access target file: {}", path),
///     None => errbang!(err::ValidationFailed, "invalid path."),
/// }
///```
#[macro_export]
macro_rules! valid_path {
    ($path_buf:expr) => {
        match $path_buf.to_str() {
            Some(path) if $path_buf.is_file() => Ok(path),
            Some(path) => errbang!(err::FileNotFound, "cannot access target file: {}", path),
            None => errbang!(err::ValidationFailed, "invalid path."),
        }
    };
}
pub use valid_path;
