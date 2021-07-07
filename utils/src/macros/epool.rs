/*
    .. + epool.rs + ..

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

pub use epool;
