/*
    .. + results.rs + ..

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

///```rust
/// resultcast!(errbangsend!(err::UnexpectedEof), Result::<()>);
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcast {
    ($result:expr, $new_result:ty) => {
        match $result {
            Ok(o) => <$new_result>::Ok(o),
            Err(e) => <$new_result>::Err(e),
        }
    };
}

pub use resultcast;
