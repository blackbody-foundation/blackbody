/*
    .. + api/mod.rs + ..

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

const SERVER_IP: &str = "127.0.0.1:4000";
const SERVER_NAME: &str = "API";

#[get("/{id}/{name}/index.html")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

pub fn run() -> Net {
    let (tx, rx) = unbounded();
    let v = verbose::init!("verbose");

    verbose::einfo!(v;1: "start {} server.", SERVER_NAME);

    thread::spawn(move || -> ResultSend<()> {
        let mut sys = rt::System::new(SERVER_NAME);

        let srv = HttpServer::new(|| App::new().service(index))
            .bind(SERVER_IP)?
            .run();
        // send server controller to main thread
        let _ = tx.send(srv.clone());

        // run future
        sys.block_on(srv)?;
        Ok(())
    });

    Net::new(SERVER_NAME, rx.recv().unwrap())
}
