mod endpoint;
pub mod models;
pub mod schema;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use clap::Parser;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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

#[derive(Clone)]
pub struct AppState {
    pool: Arc<SqlitePool>,
}

impl AppState {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub fn get_conn(&self) -> Result<r2d2::PooledConnection<ConnectionManager<SqliteConnection>>, r2d2::PoolError> {
        self.pool.get()
    }
}

fn run_migrations(connection: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn get_connection_pool(url: &str) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)?;
    
    let mut conn = pool.get()?;
    run_migrations(&mut conn)?;
    
    Ok(pool)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    dotenv::dotenv().ok();
    let args = Args::parse();

    let pool = match get_connection_pool(&args.db_url) {
        Ok(pool) => Arc::new(pool),
        Err(e) => {
            log::error!("Failed to create connection pool: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ));
        }
    };

    log::info!("Starting server at {}:{}", args.api_host, args.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(Arc::clone(&pool))))
            .wrap(actix_web::middleware::Logger::default())
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