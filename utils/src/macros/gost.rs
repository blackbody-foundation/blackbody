/*
    .. + gost.rs + ..

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

//! go + rust
//!
//! ## What is Gost?
//!
//! _Gost_ is a light way, powerful and stylish execution which is Rust macro based language.
//!
//! Key points gost features:
//!
//! * Super Enum
//!
//!
//! ```
//! gost! {
//!     enum Err {
//!         BrokenHeader : &str => "broken header."
//!         NotFound : &str => "file not found."
//!         Full : EKind::Full => { limit: 1024, msg: "Full." }
//!     }
//! }
//!
//! assert_eq!(Err::BrokenHeader.value(), "broken header")
//!
//! ```
//! * Super Struct
//!
//!
//! ```
//! gost! {
//!     enum JobKind : &'static str {
//!         Student => "studying"
//!         Salesman => "working"
//!     }
//! }
//! gost! {
//!     struct PersonPool {
//!         person => { name &'static str, age u8 }
//!         pub job => { kind JobKind }
//!         pub kim => { info person, job job }
//!         pub james => { info person, job job }
//!     }
//! }
//!
//! let a = PersonPool::new();
//! let b = a.kim.job;
//! let c = a.james;
//! ```

/// go + rust
///
/// ## What is Gost?
///
/// _Gost_ is a light way, powerful and stylish execution which is Rust macro based language.
///
/// Key points gost features:
///
/// * Super Enum
///
///
/// ```
/// gost! {
///     enum Err {
///         BrokenHeader : &str => "broken header."
///         NotFound : &str => "file not found."
///         Full : EKind::Full => { limit: 1024, msg: "Full." }
///     }
/// }
///
/// assert_eq!(Err::BrokenHeader.value(), "broken header")
///
/// ```
/// * Super Struct
///
///
/// ```
/// gost! {
///     enum JobKind : &'static str {
///         Student => "studying"
///         Salesman => "working"
///     }
/// }
/// gost! {
///     struct PersonPool {
///         person => { name &'static str, age u8 }
///         pub job => { kind JobKind }
///         pub kim => { info person, job job }
///         pub james => { info person, job job }
///     }
/// }
///
/// let a = PersonPool::new();
/// let b = a.kim.job;
/// let c = a.james;
/// ```

#[macro_export]
macro_rules! gost {

    // internal rules
    (@out_of_bracket ($t:ty) {$($key:tt : $val:tt),*})
    => {
       <$t>::from($($val),*)
    };
    (@out_of_bracket ($t:ty) $i:expr)
    => {
        $i
    };
    (@go_make $vis:vis fn default($var:ident ($def:tt) $t:ty))
    => {
        impl $var {
            $vis fn default() -> $t {
                gost!(@out_of_bracket ($t) $def)
            }
        }
    };
    (@go_make $vis:vis fn default($var:ident () $t:ty))
    => {
        impl $var {
            $vis fn default() -> $t {
                <$t>::default()
            }
        }
    };
    (@go_make $vis:vis fn default($var:ident () Self))
    => {
        impl $var {
            $vis fn default() -> Self {
                Self::new()
            }
        }
    };
    (@go_make
        $vis:vis struct $name:ident
    )
    => {
        snake_case! {
            #[derive(Debug)]
            $vis struct $name;
        }
            impl $name {
                $vis fn new() -> Self {
                    Self
                }
                $vis fn from() -> Self {
                    Self
                }
            }

    };
    (@go_make
        $vis:vis struct $name:ident {
            $(
                $l_vis:vis $val:ident: $t:ty
            ),*
        }
    )
    => {
        snake_case! {
            #[derive(Debug)]
            $vis struct $name {
                $(
                    $l_vis $val : $t
                ),*
            }
        }
            impl $name {
                $vis fn new() -> Self {
                    Self { $($val : <$t>::default()),* }
                }
                $vis fn from($($val : $t),*) -> Self {
                    Self { $($val),* }
                }
            }

    };
    //

    // Super Struct
    (
        $vis:vis struct $name:ident {
            $(
                $r_vis:vis $var:ident => {
                    $(
                        $l_vis:vis $val:ident $t:ty $(=> $default:tt)?
                    ),*
                }
            )*
        }
    )
    => {
        camelCase! {
            $vis mod $name {
                use super::*;


                    $(
                        gost!(@go_make
                                pub struct $var {
                                    $(
                                        $l_vis $val: $t
                                    ),*
                                }
                        );
                        gost!(
                            @go_make fn default($var () Self)
                        );
                    )+


                gost!(@go_make
                        pub struct $name {
                            $(
                                $r_vis $var: $var
                            ),*
                        }
                );

                pub fn new() -> $name {
                    $name::new()
                }

            }
        }
    };
    //

    // Super Enum
    (
        $vis:vis enum $name:ident : $t:ty $(=> $def:tt)? {   // single type
            $($var:ident => $val:expr)+
        }
    )
    => {
        ordering! {
            #[derive(Debug)]
            $vis enum $name {
                $($var),+
            }
        }
        impl $name {
            $vis fn value(&self) -> $t {
                match self {
                    $($name::$var => $val),+
                }
            }
        }
        gost!(
            @go_make pub fn default($name ($($def)?) $t)
        );

    };
    (
        $vis:vis enum $name:ident {    // multi type
            $(
                $var:ident : $t:ty => $input:tt
            )+
        }
    ) => {

        camelCase! {
            $vis mod $name {
                use super::*;

                $(
                    gost!(@go_make
                            pub struct $var
                    );
                    gost!(
                        @go_make pub fn default($var ($input) $t)
                    );
                    impl $var {
                        pub fn value(&self) -> $t {
                            $var::default()
                        }
                    }
                )+

            }
        }
    };
}

pub use gost;
