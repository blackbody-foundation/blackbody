/*
    .. + macros.rs + ..

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

pub use serde::{Deserialize, Serialize};
pub use Box;

#[macro_export]
macro_rules! derive_alias {
    ($name:ident => #[derive($($derive:ident),*)]) => {
        #[macro_export]
        macro_rules! $name {
            ($i:item) => {
                #[derive($($derive),*)]
                $i
            }
        }
    }
}
derive_alias! {
    serialize => #[derive(Serialize, Deserialize, Debug, PartialEq)]
}

#[macro_export]
macro_rules! epool {
    (pub enum $name:ident<T> {
        $($variant:ident(T)),*,
    }) => {

        #[derive(PartialEq, Debug)]
        pub enum $name<T> {
            $($variant(T)),*
        }

        impl<T> $name<T> {
            pub fn dump(&self) -> &T {
                match self {
                    $($name::$variant(val) => val),*
                }
            }
            pub fn flee(&self) -> $name<()> {
                match self {
                    $($name::$variant(T) => $name::$variant(())),*
                }
            }
            pub fn discriminate(&self) -> ($name<()>, &T) {
                match self {
                    $($name::$variant(val) => ($name::$variant(()), val)),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! errbang {
    ($kind:expr) => {
        Result::Err(Box::new($kind))
    };
}
#[macro_export]
macro_rules! errors {
    (pub enum $name:ident {
        $($variant:ident),*,
    }) => {

        pub(crate) mod err {

            pub trait ErrKind {
                fn as_string(&self) -> &'static str;
            }
            $(
                #[derive(Debug)]
                pub struct $variant;
                impl $variant {
                    pub fn new() -> Self { Self }
                }
                impl std::error::Error for $variant {
                    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                        Some(&$variant)
                    }
                }
                impl std::fmt::Display for $variant {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.as_string())
                    }
                }
                impl ErrKind for $variant {
                    fn as_string(&self) -> &'static str {
                        super::$name::$variant($variant).as_string()
                    }
                }
            )*

        }

        #[derive(Debug)]
        pub enum $name {
            $($variant(err::$variant)),*
        }
        impl std::error::Error for $name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    $($name::$variant(val) => Some(val)),*
                }
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }


    };
}

pub use epool;
pub use errbang;
pub use errors;
pub use serialize;
