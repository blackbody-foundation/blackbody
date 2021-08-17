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

/// for verify
mod verify;

#[inline]
pub fn run(mode: &str) -> ServerList {
    ServerList(match mode {
        rpc::SERVER_NAME => vec![rpc::run()],
        api::SERVER_NAME => vec![api::run()],
        _ => vec![rpc::run(), api::run()], /* start RPC -> API */
    })
}

#[inline]
pub fn stop(servers: &mut ServerList) {
    let v = verbose::init!("outter", "verbose");

    for net in servers.iter().rev() {
        verbose::einfo!(v;1: "stop {} server.", net.name); /* stop API -> RPC */

        rt::System::new(rand::random::<char>()).block_on(net.server.stop(true));
        // wait until server gracefully exit
    }
    thread::sleep(Duration::from_millis(100));
}

pub fn restart(servers: &mut ServerList, mode: &str) {
    match mode {
        api::SERVER_NAME => {}
        rpc::SERVER_NAME => {}
        name!(BOTH) => {
            stop(servers);
            *servers = run(mode);
            return;
        }
        _ => something_wrong!("* Failed to restart invalid mode name")(),
    }

    let _ = find_and_stop(servers, mode);
    let net = run(mode);
    servers.extend(net);
}

pub fn find_and_stop(servers: &mut ServerList, mode: &str) -> Result<()> {
    if mode == name!(BOTH) {
        stop(servers);
        *servers = ServerList::new();
        Ok(())
    } else {
        let mut i = 0;
        let mut found = false;
        for net in servers.iter() {
            if net.name == mode {
                found = true;
                break;
            }
            i += 1;
        }
        if !found {
            return Err("* item not found".into());
        }
        stop(&mut ServerList(vec![(servers.0[i]).clone()]));
        servers.remove(i);
        Ok(())
    }
}
