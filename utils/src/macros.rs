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

pub use crate::types::Value;
pub use serde::{Deserialize, Serialize};
pub use Box;

#[macro_export]
macro_rules! derive_alias {
    ($( $name:ident => #[derive($($derive:ident),*)], )*) => {
        $(
            #[macro_export]
            macro_rules! $name {
                ($i:item) => {
                    #[derive($($derive),*)]
                    $i
                }
            }
        )*
    };
}
derive_alias! {
    serialize => #[derive(Serialize, Deserialize, Debug, PartialEq)],
    ordering => #[derive(Eq, PartialEq, PartialOrd, Debug)],
}

#[macro_export]
macro_rules! gost {
    (
        $vis:vis enum $name:ident {
            $($var:ident : $t:ty => $exp:expr),*
        }
    ) => {
        ordering! {
            $vis enum $name {
                $($var),*
            }
        }
        pub mod (concat_idents!(_, $name)) {
            $(
                #[derive(Debug)]
                pub struct $var($t);
                impl $var {
                    pub fn new(var: $t) -> Self {
                        Self(var)
                    }
                }
            )*
        }

        impl $name {
            $vis fn value(&self) -> Value<Box<dyn std::any::Any>> {
                match self {
                    $($name::$var => Value::new(Box::new($exp as $t)) ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! epool {
    ($vis:vis enum $name:ident<T> {
        $($variant:ident(T)),*,
    }) => {

        #[derive(PartialEq, Debug)]
        $vis enum $name<T> {
            $($variant(T)),*
        }

        impl<T> $name<T> {
            $vis fn dump(&self) -> &T {
                match self {
                    $($name::$variant(val) => val),*
                }
            }
            $vis fn flee(&self) -> $name<()> {
                match self {
                    $($name::$variant(_t) => $name::$variant(())),*
                }
            }
            $vis fn discriminate(&self) -> ($name<()>, &T) {
                match self {
                    $($name::$variant(val) => ($name::$variant(()), val)),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! errbang {
    ($kind:ty) => {
        Result::Err(Box::new(<$kind>::new(format!("[{}:{}]", file!(), line!()))))
    };
}
#[macro_export]
macro_rules! errors {
    (

        pub enum $name:ident
        {
            $($variant:ident => $str:expr),*,
        }

    ) => {

        #[derive(Debug)]
        pub enum $name {
            $($variant),*
        }

        pub mod err {

            $(
                #[derive(Debug)]
                pub struct $variant {
                    meta: String,
                }

                impl $variant {
                    pub fn new(meta: String) -> Self {
                        Self { meta }
                    }
                    pub fn as_string(&self) -> &'static str {
                        $str
                    }
                }

                impl std::error::Error for $variant {
                    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                        Some(self)
                    }
                }
                impl std::fmt::Display for $variant {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{} {}", self.meta, self.as_string())
                    }
                }
            )*

        }
    };
}

#[macro_export]
macro_rules! fheader {
    ($vis:vis struct $name:ident {
        $($variant:ident: $t:ty => $val:expr),*,
    }) => {

        serialize! {
            $vis struct $name {
                $($variant: $t),*
            }
        }
        impl $name {
            $vis fn new() -> Self {
                Self {
                    $($variant: $val),*
                }
            }
        }
    }
}

pub use epool;
pub use errbang;
pub use errors;
pub use fheader;
pub use gost;
pub use ordering;
pub use serialize;
