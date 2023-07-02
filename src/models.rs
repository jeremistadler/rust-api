use super::schema::source_files;
use diesel::prelude::*;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct SourceFile {
    path: String,
    hash: String,
    size: i32,
    date_created: String,
}

#[derive(serde::Deserialize, Insertable, Debug)]
#[diesel(table_name = source_files)]
pub struct NewUser {
    path: String,
    hash: String,
    size: i32,
    date_created: String,
}
