use crate::AppState;
use actix_web::web;

pub async fn upload(_data: web::Data<AppState>) -> String {
    // TODO - Save file locally
    String::from("Hello World!")
}

pub async fn csv(path: web::Path<String>) -> String {
    let _id = path.into_inner();
    // TODO - Retrieve file by id
    String::from("Hello World!")
}
