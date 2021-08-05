/*
    .. + chan.rs + ..

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

/// pipechan!(num_pipe, $(bounded_cap)?, MessageType);
/// if bounded_cap is empty then creates unbounded channels
#[macro_export]
macro_rules! pipechan {
    (@create_channel $cap:expr, $message_type:ty) => {
        crossbeam::channel::bounded::<$message_type>($cap)
    };
    (@create_channel $message_type:ty) => {
        crossbeam::channel::unbounded::<$message_type>()
    };
    (
        $number:expr
    ) => {
        vec![utils::types::chan::Chan::<()>::none(); $number]
    };
    (
        $number:expr, $(cap:$cap:expr,)? msg:$message_type:ty
    ) => {
        {
            use utils::types::chan::Chan;

            assert!(
                $number < 3,
                "pipechan's the number of channels must be more than 3."
            );
            let mut pipe = Vec::new();

            let (root_tx, root_rx) = utils::pipechan!(@create_channel $($cap,)? $message_type);

            let (tx, mut prev_rx) = utils::pipechan!(@create_channel $($cap,)? $message_type);

            pipe.push(Chan::new(Some(tx), Some(root_rx)));

            for _ in 1..$number { // number of channels = n - 1
                let (tx, rx) = utils::pipechan!(@create_channel $($cap,)? $message_type);
                let chan = Chan::new(Some(tx), Some(prev_rx));
                prev_rx = rx;
                pipe.push(chan);
            }

            pipe.push(Chan::new(Some(root_tx), Some(prev_rx)));

            pipe
        }
    };
}

pub use pipechan;
