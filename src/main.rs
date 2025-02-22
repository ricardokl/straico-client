use actix_web::{web, App, HttpResponse, HttpServer};
use clap::Parser;

mod server;

#[derive(Parser)]
#[command(
    name = "straico-proxy",
    about = "A proxy server for Straico API that provides OpenAI-compatible endpoints",
    version
)]
struct Cli {
    /// Host address to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(long, default_value = "8000")]
    port: u16,

    /// API key for Straico (alternatively use STRAICO_API_KEY env var)
    #[arg(long, env = "STRAICO_API_KEY", hide_env_values = true)]
    api_key: Option<String>,

    /// Enable debug logging of requests and responses
    #[arg(long)]
    debug: bool,
}

// pub fn completion_with_key(
//     api_key: impl Display,
// ) -> Result<StraicoRequestBuilder<ApiKeySet, CompletionRequest<'a>, CompletionData>> {
//     let client = StraicoClient::default();
//     Ok(client.completion().bearer_auth(api_key))
// }

/// Represents the application state shared across HTTP request handlers.
///
/// This struct contains all the necessary components for handling requests,
/// including the Straico API client, authentication key, and debug settings.
#[derive(Clone)]
struct AppState {
    /// The Straico API client used for making requests
    client: straico::client::StraicoClient,
    /// API authentication key for Straico
    key: String,
    /// Flag to enable debug logging of requests/responses
    debug: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let api_key = cli.api_key.expect("API key not set");

    let addr = format!("{}:{}", cli.host, cli.port);
    println!("Starting Straico proxy server...");
    println!("Server is running at http://{}", addr);
    println!("Completions endpoint is at /v1/chat/completions");
    if cli.debug {
        println!("Debug mode enabled - requests and responses will be logged");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                client: straico::client::StraicoClient::new(),
                key: api_key.clone(),
                debug: cli.debug,
            }))
            .service(server::openai_completion)
            .default_service(web::to(HttpResponse::NotFound))
    })
    .bind(addr)?
    .run()
    .await
}
