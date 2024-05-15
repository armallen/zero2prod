use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::{HttpRequest, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

/*
> curl http://127.0.0.1:8000
Hello World!%
> curl http://127.0.0.1:8000/abc
Hello abc!%
> curl http://127.0.0.1:8000/abc.txt
Hello abc.txt!%
> curl http://127.0.0.1:8000/abc/def

*/
#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder {
    // Responder is a trait that allows the function to return a variety of types
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async // without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
