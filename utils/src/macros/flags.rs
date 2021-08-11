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

/// simply create `struct`
///```no_run
/// flags! {
///     pub name
///     Foo enum {
///         Kind1,
///         Kind2,
///     }
///     Bar struct {
///         var1: Type1,
///         var2: Type2,
///     }
///     Foo2 bool
///     Foo3 u8
///     Foo4 SomeStruct
/// }
///```
#[macro_export]
macro_rules! flags {
    (
        @create $vis:vis $var:ident enum { $val_0:ident, $( $val:ident ),* }
    ) => {
        #[derive(Debug, PartialEq, Clone)]
        $vis enum $var {
            $val_0,
            $(
                $val
            ),*
        }
        impl Default for $var {
            fn default() -> Self {
                Self::$val_0
            }
        }
    };
    (
        @create $vis:vis $var:ident struct { $( $val:ident: $val_t:ty ),+ }
    ) => {
        #[derive(Debug, Default, Clone)]
        $vis struct $var {
            $(
                $vis $val: $val_t
            ),+
        }
    };
    (
        @create $vis:vis $var:ident $t:ty
    ) => {

    };
    (
        @filter $var:ident enum
    ) => {
        $var
    };
    (
        @filter $var:ident struct
    ) => {
        $var
    };
    (
        @filter $var:ident $ty:ty
    ) => {
        $ty
    };
    (
        @default_or (), $t:ty
    ) => {
        <$t>::default()
    };
    (
        @default_or ($val:expr), $t:ty
    ) => {
        $val
    };
    (
        @apply $vis:vis $name:ident ($($var:ident: $t:tt$(=> $default:expr)?),+)
    ) => {
        #[allow(non_snake_case)]
        #[derive(Debug, Clone)]
        $vis struct $name {
            $(
                $vis $var: flags!(@filter $var $t)
            ),+
        }
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $var: flags!(@default_or ($($default)?), $t)
                    ),*
                }
            }
        }
    };
    (
        $vis:vis $name:ident$(,)?
        $($var:ident $t:tt $({ $($val:ident$(,)? $(:$val_t:ty $(,)? )? )+ })?$( => $default:expr)?)*
    ) => {
        $(
            #[allow(non_snake_case)]
            flags!(@create $vis $var $t $({ $($val $(:$val_t)? ),+ })?);
        )*
        flags!(@apply $vis $name ($($var: $t$(=> $default)?),+));
    };
}

pub use flags;
