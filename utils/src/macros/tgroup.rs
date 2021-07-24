/*
    .. + tgroup.rs + ..

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

///```rust
///tgroup! {
///     pub TG,
///     R = Requirement,
///     O = ResultSend<()>,
///     TRead,
///     TWrite,
/// }
///```
#[macro_export]
macro_rules! tgroup {
    (
        $vis:vis $name:ident,
        R = $requirement:ty,
        O = $output:ty,
        $($sub_group:ty$(,)?)*
    ) => {

        $vis struct $name {
            $vis requirement: $requirement,
            /// sub group thread handlers
            $vis sub: Vec<std::thread::JoinHandle<$output>>,
        }
        impl TGroup for $name {
            type R = $requirement;
            type O = $output;
            fn new(requirement: Self::R) -> Self {
                let mut sub = Vec::new();
                $(
                    sub.push(<$sub_group>::new(&requirement));
                )*
                Self{requirement, sub}
            }
            fn join(self) -> Vec<Self::O> {
                let mut res = Vec::new();
                for handles in self.sub.into_iter() {
                    res.push(handles.join().unwrap());
                }
                res
            }
        }

    };
}

pub use tgroup;
