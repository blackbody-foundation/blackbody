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
    ($kind:ty) => {
        Result::Err(Box::new(<$kind>::new(format!("[{}:{}]", file!(), line!()))))
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
                }

                impl $kind {
                    pub fn new(meta: String) -> Self {
                        Self { meta }
                    }
                    pub fn as_string(&self) -> &'static str {
                        $message
                    }
                }

                impl std::error::Error for $kind {
                    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                        Some(self)
                    }
                }
                impl std::fmt::Display for $kind {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{} {}", self.meta, self.as_string())
                    }
                }

            )*

        }
    };
}

pub use errbang;
pub use errors;