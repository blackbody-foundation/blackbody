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

#[macro_export]
macro_rules! derives {
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
derives! {
    serialize => #[derive(Serialize, Deserialize, Debug, PartialEq)],
    ordering => #[derive(Eq, PartialEq, PartialOrd, Debug)],
}

pub use ordering;
pub use serialize;
