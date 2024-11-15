use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod server;

#[allow(dead_code)]
struct AppState {
    client: straico::client::StraicoClient,
    key: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hi!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                client: straico::client::StraicoClient::new(),
                key: std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY is not set"),
            }))
            .service(server::openai_completion)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
