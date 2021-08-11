use actix_web::{get, web, App, HttpServer, Responder};
use utils::system::*;
// use blackbody::net::api::*;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

pub async fn run() -> Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:80")?
        .run()
        .await?;
    Ok(())
}
