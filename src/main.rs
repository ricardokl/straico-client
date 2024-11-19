use actix_web::{web, App, HttpResponse, HttpServer};

mod server;

struct AppState {
    client: straico::client::StraicoClient,
    key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    println!("Server is running at http://{}", addr);
    println!("Completions endpoint is at /v1/chat/completions");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                client: straico::client::StraicoClient::new(),
                key: std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY is not set"),
            }))
            .service(server::openai_completion)
            // Maybe  implement a default service to respond also a json.
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(addr)?
    .run()
    .await
}
