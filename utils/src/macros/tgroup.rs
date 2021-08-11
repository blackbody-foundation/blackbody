/*
    .. + tgroup.rs + ..

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

///```no_run
///tgroup! {
///     pub TG,
///     R = Requirement,
///     O = Vec<u8>,
///     pipeline(msg::Message, 1024), // optional
///     [
///         TRead,
///         TWrite,
///     ]
/// }
///```
/// R = Requirement Type<br>
/// O = JoinHandler Result Ok Type<br>
/// pipeline(MssageType, BoundedCap)<br>
/// or
/// pipeline(MessageType)<br>
/// this creates unbounded channels<br><br>
/// if pipeline is actived,
/// * TRead, TWrite, ... <- must be ordered by pipechan
/// * pipeline channels are connected as circular way
#[macro_export]
macro_rules! tgroup {
    (@count) => {0usize};
    (@count $_head:tt$($tail:tt)*) => {1usize + tgroup!(@count $($tail)*)};
    (
        $vis:vis $name:ident,
        R = $requirement:ty,
        O = $output:ty,
        $(
        pipeline($message_type:ty$(,$cap:expr)?),
        )?
        [
        $($sub_group:ty$(,)?)+
        ]$(,)?
    ) => {


        $vis struct $name {
            /// sub group thread handlers
            $vis sub: Vec<std::thread::JoinHandle<ResultSend<$output>>>,
        }
        impl TGroup for $name {
            type R = $requirement;
            type O = $output;
            fn new(requirement: Self::R) -> Self {
                let mut sub = Vec::new();
                let count = tgroup!(@count $($sub_group)+); // count number of sub groups
                let mut chan_iter = utils::pipechan!(count$( $(,cap:$cap)?, msg:$message_type )?).into_iter(); // create pipe channels

                $(

                    sub.push(<$sub_group>::new(&requirement, chan_iter.next().unwrap()));

                )+

                Self{sub}
            }
            /// returns Result::Err if any thread in the tgroup has an error
            fn join(self) -> Result<Vec<Self::O>> {
                let mut res = Vec::new();
                for handles in self.sub.into_iter() {
                    res.push(resultcast!(handles.join().unwrap())?);
                }
                Ok(res)
            }
        }

    };
}

pub use tgroup;
