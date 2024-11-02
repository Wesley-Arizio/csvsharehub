mod endpoint;

use actix_web::{App, HttpServer, web};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Host of the server
    #[arg(short, env = "CSV_SHARE_HUB_HOST")]
    api_host: String,

    /// Port in which the server will run
    #[arg(short, default_value_t = 8080, env = "CSV_SHARE_HUB_PORT")]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        eprintln!("Could not load environment variables: {:?}", e);
        std::process::exit(1);
    }

    let args = Args::parse();

    HttpServer::new(|| App::new().service(
        web::scope("/csv")
            .route("/", web::post().to(endpoint::upload))
            .route("/{id}", web::get().to(endpoint::csv))
    ))
        .bind((args.api_host, args.port))?
        .run()
        .await
}
