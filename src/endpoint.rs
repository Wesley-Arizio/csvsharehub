use actix_web::web;

pub async fn upload() -> String {
    String::from("Hello World!")
}

pub async fn csv(path: web::Path<String>) -> String {
    let id = path.into_inner();
    format!("Hello {} csv file", id)
}