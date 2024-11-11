// endpoint.rs
use crate::{models::NewFile, schema::files, AppState};
use actix_multipart::Multipart;
use actix_web::{error::ErrorInternalServerError, web, Error, HttpResponse};
use diesel::prelude::*;
use futures_util::TryStreamExt;
use serde_json::json;
use std::{fs, io::Write, path::Path};
use std::os::unix::fs::PermissionsExt;
use actix_web::error::ErrorNotFound;
// Add this import for Unix permissions
use uuid::Uuid;
use log::info;

// Make this an absolute path
const UPLOAD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/uploads");


pub async fn upload(
    data: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    // Create uploads directory with explicit permissions
    if let Err(e) = fs::create_dir_all(UPLOAD_DIR) {
        log::error!("Failed to create upload directory: {}", e);
        return Err(ErrorInternalServerError(format!("Failed to create upload directory: {}", e)));
    }

    info!("Upload directory created successfully");

    while let Some(mut field) = payload.try_next().await.map_err(|e| {
        log::error!("Failed to get next field: {}", e);
        ErrorInternalServerError(format!("Failed to get next field: {}", e))
    })? {
        let content_disposition = field.content_disposition();
        
        let filename = match content_disposition.get_filename() {
            Some(name) => name.to_owned(),
            None => {
                log::error!("No filename provided");
                return Err(ErrorInternalServerError("No filename provided"));
            }
        };

        info!("Processing file: {}", filename);
        
        if !filename.to_lowercase().ends_with(".csv") {
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "Only CSV files are allowed"
            })));
        }

        let file_id = Uuid::new_v4().to_string();
        let file_path = Path::new(UPLOAD_DIR)
            .join(&file_id)
            .with_extension("csv");
        
        info!("Attempting to create file at: {}", file_path.display());

        let mut file = match fs::File::create(&file_path) {
            Ok(file) => {
                // Set file permissions to 644 (rw-r--r--)
                if let Ok(metadata) = file.metadata() {
                    let mut perms = metadata.permissions();
                    perms.set_mode(0o644);
                    if let Err(e) = fs::set_permissions(&file_path, perms) {
                        log::error!("Failed to set file permissions: {}", e);
                    }
                }
                file
            },
            Err(e) => {
                log::error!("Failed to create file: {}", e);
                return Err(ErrorInternalServerError("Failed to create file"));
            }
        };
        
        let mut size = 0i64;
        while let Some(chunk) = field.try_next().await? {
            size += chunk.len() as i64;
            if let Err(e) = file.write_all(&chunk) {
                log::error!("Failed to write chunk: {}", e);
                return Err(ErrorInternalServerError("Failed to write file"));
            }
        }

        // Ensure all data is written to disk
        if let Err(e) = file.sync_all() {
            log::error!("Failed to sync file: {}", e);
            return Err(ErrorInternalServerError("Failed to sync file"));
        }

        let file_path_str = file_path.to_str()
            .ok_or_else(|| ErrorInternalServerError("Invalid file path"))?;

        let new_file = NewFile::new(
            &file_id,
            &filename,
            file_path_str,
            size,
        );

        let conn = &mut data.get_conn()
            .map_err(ErrorInternalServerError)?;

        diesel::insert_into(files::table)
            .values(&new_file)
            .execute(conn)
            .map_err(ErrorInternalServerError)?;

        info!("File successfully saved: {}", file_path.display());

        return Ok(HttpResponse::Ok().json(json!({
            "id": file_id,
            "message": "File uploaded successfully",
            "size": size,
            "path": file_path_str
        })));
    }

    Err(ErrorInternalServerError("No file provided"))
}

pub async fn csv(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let conn = &mut data.get_conn()
        .map_err(ErrorInternalServerError)?;

    let file = files::table
        .find(id)
        .first::<crate::models::File>(conn)
        .map_err(|_| ErrorNotFound("File not found"))?;

    let file_contents = fs::read_to_string(&file.file_path)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", file.original_name),
        ))
        .content_type("text/csv")
        .body(file_contents))
}
