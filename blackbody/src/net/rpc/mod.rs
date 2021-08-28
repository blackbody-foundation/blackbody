/*
    .. + rpc/mod.rs + ..

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

use super::*;

pub const SERVER_IP: &str = "127.0.0.1:4200";
pub const SERVER_NAME: &str = name!(RPC);

#[get("/{id}/{name}/index.html")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

pub fn run() -> Result<Net> {
    let (tx, rx) = unbounded();
    let v = verbose::init!(name!(outter), name!(verbose: l));

    verbose::einfo_styled!(v;1: Style::new().dim() => "start {} server.", SERVER_NAME);

    thread::spawn(move || -> Result<()> {
        let mut sys = rt::System::new(rand::random::<char>());

        let srv = HttpServer::new(|| App::new().service(index))
            .bind(SERVER_IP)?
            .run();
        // send server controller to main thread
        let _ = tx.send(srv.clone());

        // run future
        sys.block_on(srv)?;

        Ok(())
    });

    Ok(Net::new(
        SERVER_NAME,
        errcast!(rx.recv(), err::ThreadReceiving, name!(UnexpectedRuntime)),
    ))
}
