/*
    .. + message.rs + ..

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

///```rust
/// message! {
///     pub name,
///     M = Vec<u8>,
///     K = enum {
///         Kind1,
///         Kind2,
///     }
/// }
///```
/// M = Message Type, K = Kind Enum
#[macro_export]
macro_rules! message {
    (
        $vis:vis $name:ident,
        M = $t:ty,
        K = enum {
            $($kind:ident$(,)?)*
        }
    ) => {
        /// message
        $vis mod $name {
            use utils::types::message;

            pub type Message = message::Message<Kind, $t>;

            pub type TypePayload = $t;

            pub enum Kind {
                $($kind),*
            }
        }
    };
}

pub use message;
