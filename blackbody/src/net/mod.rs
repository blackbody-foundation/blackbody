/*
    .. + mod.rs + ..

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

pub mod cmn;
use cmn::*;

pub mod api;
pub mod rpc;

pub struct ServerList(pub Vec<Net>);

#[inline(always)]
pub fn run(mode: &str) -> ServerList {
    ServerList(match mode {
        "rpc" => vec![rpc::run()],
        "api" => vec![api::run()],
        _ => vec![rpc::run(), api::run()], /* start RPC -> API */
    })
}

#[inline(always)]
pub fn stop(servers: ServerList) {
    let v = verbose::init!("verbose");

    for net in servers.0.into_iter().rev() {
        verbose::einfo!(v;1: "stop {} server.", net.name); /* stop API -> RPC */

        rt::System::new(net.name).block_on(net.server.stop(true));
        // wait until server gracefully exit
    }
}
