use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: String,
    pub original_name: String,
    pub file_path: String,
    pub content_type: String,
    pub size: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::files)]
pub struct NewFile<'a> {
    pub id: &'a str,
    pub original_name: &'a str,
    pub file_path: &'a str,
    pub content_type: &'a str,
    pub size: i64,
}

impl<'a> NewFile<'a> {
    pub fn new(
        id: &'a str,
        original_name: &'a str,
        file_path: &'a str,
        size: i64,
    ) -> Self {
        Self {
            id,
            original_name,
            file_path,
            content_type: "text/csv",
            size,
        }
    }
}