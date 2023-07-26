use actix_web::{App, HttpServer};
use crate::controller::{get_index, post_index};

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {

    HttpServer::new(||{
        App::new().service(get_index)
            .service(post_index)
    }).bind(("127.0.0.1",8080))?.run().await

}