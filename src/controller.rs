use actix_web::{get, web, HttpResponse, Responder, post};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Point {
    x:i32,
    y:i32,
    length:Option<f64>
}


impl Point {
    fn calc(&mut self) {
        let res:f64 = ((self.x * self.x + self.y * self.y) as f64).sqrt();
        self.length = Some(res);
    }
}

#[get("/")]
pub async fn get_index() -> impl Responder {
    let mut p = Point{
        x:3,
        y:4,
        length:None
    };
    p.calc();
    HttpResponse::Ok().json(p)
}

#[post("/")]
pub async fn post_index(mut item: web::Json<Point>) -> impl Responder {
    item.calc();
    HttpResponse::Ok().json(item)
}