use crate::{
    endpoint::{upload, csv},
    models::NewFile,
    schema::files,
    AppState,
    MIGRATIONS
};
use actix_web::{test, web::Data, web};
use actix_multipart::Multipart;
use diesel::r2d2::{self, Pool};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use std::sync::Arc;
use std::fs;
use std::path::Path;
use futures_util::StreamExt;
use uuid::Uuid;
use dotenv;

fn setup_test_db() -> Pool<diesel::r2d2::ConnectionManager<SqliteConnection>> {
    let test_db_path = format!("/tmp/test_{}.db", Uuid::new_v4());
    let database_url = format!("sqlite://{}", test_db_path);
        
    let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)  // Increase connection pool size
        .connection_timeout(std::time::Duration::from_secs(30))  // Increase timeout
        .build(manager)
        .expect("Failed to create test pool");
    
    // Run migrations on the test database
    let mut conn = pool.get().expect("Failed to get connection");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
    
    pool
}

#[actix_web::test]
async fn test_upload_endpoint() {
    let pool = setup_test_db();
    let app_state = Data::new(AppState::new(Arc::new(pool)));
    
    // Create uploads directory with explicit permissions
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/uploads");
    if fs::metadata(upload_dir).is_ok() {
        fs::remove_dir_all(upload_dir).unwrap();
    }
    fs::create_dir_all(upload_dir).unwrap();
    
    let content = "test,data\n1,2\n";
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    
    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"file\"; filename=\"test.csv\"\r\n\
        Content-Type: text/csv\r\n\r\n\
        {content}\r\n\
        --{boundary}--\r\n"
    );

    let mut app = test::init_service(
        actix_web::App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/csv")
                    .route("/", web::post().to(upload))
                    .route("/{id}", web::get().to(csv))
            )
    ).await;

    let req = test::TestRequest::post()
        .uri("/csv/")
        .set_payload(body)
        .append_header(("content-type", format!("multipart/form-data; boundary={}", boundary)))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    
    if status != 200 {
        let body = test::read_body(resp).await;
        println!("Response body: {:?}", String::from_utf8_lossy(&body));
    }
    
    assert_eq!(status, 200);
    
    // Cleanup - only remove if it exists
    if fs::metadata(upload_dir).is_ok() {
        fs::remove_dir_all(upload_dir).unwrap();
    }
}

#[actix_web::test]
async fn test_csv_endpoint() {
    let pool = setup_test_db();
    let app_state = Data::new(AppState::new(Arc::new(pool)));
    
    let file_id = Uuid::new_v4().to_string();
    let test_content = "test,data\n1,2\n";
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/uploads");
    let test_file_path = Path::new(upload_dir).join(&file_id).with_extension("csv");
    
    // Ensure upload directory exists and is empty
    if fs::metadata(upload_dir).is_ok() {
        fs::remove_dir_all(upload_dir).unwrap();
    }
    fs::create_dir_all(upload_dir).unwrap();
    
    // Write test file
    fs::write(&test_file_path, test_content).unwrap();
    println!("Created test file at: {}", test_file_path.display());
    
    // Scope the database operations
    {
        let conn = &mut app_state.get_conn().unwrap();
        let new_file = NewFile::new(
            &file_id,
            "test.csv",
            test_file_path.to_str().unwrap(),
            test_content.len() as i64,
        );
        
        diesel::insert_into(files::table)
            .values(&new_file)
            .execute(conn)
            .unwrap();
    } // Connection is dropped here

    let mut app = test::init_service(
        actix_web::App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/csv")
                    .route("/", web::post().to(upload))
                    .route("/{id}", web::get().to(csv))
            )
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/csv/{}", file_id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    let status = resp.status();
    
    if status != 200 {
        let body = test::read_body(resp).await;
        println!("Response error: {:?}", String::from_utf8_lossy(&body));
    }
    
    assert_eq!(status, 200);
    
    // Cleanup
    fs::remove_file(test_file_path).unwrap();
    fs::remove_dir_all(upload_dir).unwrap();
}

#[actix_web::test]
async fn test_upload_invalid_extension() {
    let pool = setup_test_db();
    let app_state = Data::new(AppState::new(Arc::new(pool)));
    
    let content = "test data";
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    
    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\n\
        Content-Type: text/plain\r\n\r\n\
        {content}\r\n\
        --{boundary}--\r\n",
    );

    let mut app = test::init_service(
        actix_web::App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/csv")
                    .route("/", web::post().to(upload))
                    .route("/{id}", web::get().to(csv))
            )
    ).await;

    let req = test::TestRequest::post()
        .uri("/csv/")
        .set_payload(body)
        .append_header(("content-type", format!("multipart/form-data; boundary={}", boundary)))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 400);
}