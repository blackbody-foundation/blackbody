/*
    .. + errbang.rs + ..

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

#[macro_export]
macro_rules! errbang {
    ($kind:ty$(, $message:expr)?) => {
        Result::Err(Box::new(<$kind>::new(format!(concat!("[{}:{}]", $($message)?), file!(), line!()))))
    };
}

#[macro_export]
macro_rules! errmatch {
    ($err:expr, $kind:ty) => {
        match $err.downcast_ref::<$kind>() {
            Some(e) => true,
            None => false,
        }
    };
}

#[macro_export]
macro_rules! errors {
    (
            $($kind:ident => $message:tt$(,)?)*
    ) => {

        pub mod err {

            $(
                #[derive(Debug)]
                pub struct $kind {
                    meta: String,
                    message: &'static str,
                }

                impl $kind {
                    pub fn new(meta: String) -> Self {
                        Self { meta, message: $message }
                    }
                    pub fn as_combination(&self) -> String {
                        format!("{} {}", self.meta, self.message)
                    }
                }

                impl std::error::Error for $kind {
                    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                        Some(self)
                    }
                }
                impl std::fmt::Display for $kind {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.as_combination())
                    }
                }

            )*

        }
    };
}

pub use errbang;
pub use errmatch;
pub use errors;
