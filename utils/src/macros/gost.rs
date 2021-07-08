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
    // create structure
    (@go fn default $var:ident $def:ident$t:ty)
    => {
        impl $var {
            pub fn default() -> $t {
                $def
            }
        }
    };
    (@go fn default $var:ident$t:ty)
    => {
        impl $var {
            pub fn default() -> Self {
                Self::default()
            }
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
    // (pub) struct_name { var_name type, ... }
    (@go $($vis:vis $var:ident => {$($field_vis:vis $val:ident $($def:ident)?$t:ty)+})+)
    => {

        $(
            #[allow(non_snake_case, non_camel_case_types)]
            #[derive(Default)]
            $vis struct $var {
                $(
                    $field_vis $val: $t
                ),*
            }
            $(
                gost!(@go fn default $val $($def)?$t);
            )+
        )+

    };

    // Super Struct
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $var:ident => {$($val_vis:vis $val:ident $t:ty$( => $def:ident)?),+})+
        }
    ) => {
        #[allow(non_snake_case, unused_braces)]
        $vis mod $name {
            use super::*;

            gost!(@go $($field_vis $var => {$($val_vis $val $($def)?$t)+})+);

            $([$meta])*
            #[derive(Default)]
            pub struct $name {
                $(
                    $field_vis $var: $var
                ),*
            }
            impl $name {
                fn new() -> Self {
                    Self::default()
                }
            }
            pub fn new() -> $name {
                $name::new()
            }
        }
    };


    // Super Enum

    // single type
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident : $t:ty {
            $(#[$field_meta:meta])*
            $($var:ident => $exp:expr)+
        }
    ) => {
        ordering! {
            $([$meta])*
            $vis enum $name {
                $([$field_meta])*
                $($var),*
            }
        }
        impl $name {
            $vis fn value(&self) -> $t {
                match self {
                    $($name::$var => $exp),*
                }
            }
        }
        #[allow(unreachable_code)]
        impl Default for $name {
            fn default() -> Self {
                $(
                return Self::$var;
                )+
            }
        }
    };

    // multi type
    (
        $vis:vis enum $name:ident {
            $(
                $(#[$meta:meta])*
                $var:ident : $t:ty => $input:tt
            )+
        }

    ) => {
        #[allow(non_snake_case, unused_braces)]
        $vis mod $name {
            use super::*;

            gost!(@go $($vis $var => {$vis $var $input$t})+);

        }

        // $vis mod $name {
        //     use super::*;
        //     $(
        //         ordering! {
        //             $([$meta])*
        //             #[derive(Default)]
        //             pub struct $var;
        //         }
        //         impl $var {
        //             pub fn value(&self) -> $t {
        //                 gost!(@go_enum $var : $t => $input)
        //             }
        //             pub fn default(&self) -> $t {
        //                 gost!(@go_enum $var : $t => $input)
        //             }
        //         }
        //     )*
        // }
    };
    // (@go_enum $var:ident : $t:ty => {$($key:tt : $val:tt),+}) => {
    //     <$t>::from($($val),+)
    // };
    // (@go_enum $var:ident : $t:ty => $input:tt) => {
    //     $input
    // };
}

pub use gost;
