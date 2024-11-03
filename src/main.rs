mod endpoint;
pub mod models;
pub mod schema;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use clap::Parser;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

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

    /// Url to connect to the database
    #[arg(short, env = "CSV_SHARE_HUB_DATABASE_URL")]
    db_url: String,
}

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

struct AppState {
    pool: Arc<SqlitePool>,
}

impl AppState {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

pub fn get_connection_pool(url: &str) -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        eprintln!("Could not load environment variables: {:?}", e);
        std::process::exit(1);
    }

    let args = Args::parse();

    let pool = Arc::new(get_connection_pool(&args.db_url));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(Arc::clone(&pool))))
            .service(
                web::scope("/csv")
                    .route("/", web::post().to(endpoint::upload))
                    .route("/{id}", web::get().to(endpoint::csv)),
            )
    })
    .bind((args.api_host, args.port))?
    .run()
    .await
}
