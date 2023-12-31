use actix_web::{get,post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| {
        App::new().service(hello_world)
    }).bind(("0.0.0.0", 9000)).unwrap().run().await;
}
