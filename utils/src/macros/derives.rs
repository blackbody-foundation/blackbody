/*
    .. + derives.rs + ..

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

/// custom derives
#[macro_export]
macro_rules! derives {
    ($( $name:ident => { $(#[$id:tt($($derive:expr),*)]),* } )*) => {
        $(
            #[macro_export]
            macro_rules! $name {
                ($i:item) => {
                    $(
                        #[$id($($derive),*)]
                    )*
                    $i
                }
            }
        )*
    };
}
derives! {
    ordering => { #[derive(Eq, PartialEq, PartialOrd)] }
    camelCase => { #[allow(non_snake_case)], #[warn(non_camel_case_types)] }
    snake_case => { #[allow(non_camel_case_types)], #[warn(non_snake_case)] }
}

pub use camelCase;
pub use ordering;
pub use snake_case;

///
///
///
#[macro_export]
macro_rules! derive_new {
    (
        $vis:vis struct $name:ident {
            $($f_vis:vis $var:ident: $t:ty),*$(,)?
        }
    ) => {
        $vis struct $name {
            $($f_vis $var: $t),*
        }
        impl $name {
            $vis fn new($($var: $t),*) -> Self {
                Self {
                    $($var),*
                }
            }
        }
    };
}
pub use derive_new;

///
///
///
pub mod serde {
    #[macro_export]
    macro_rules! serialize {
        ($i:item) => {
            #[derive(Clone, Debug, Deserialize, Serialize)]
            #[serde(crate = "self::serde")]
            $i
        };
    }
    pub use bincode;
    pub use serde::{self, Deserialize, Serialize};
    pub use serialize;
}
