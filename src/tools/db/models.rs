use diesel::prelude::*;
use crate::tools::db::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::music)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Music {
    pub id: i32,
    pub title: String,
    pub path: String
}

#[derive(Insertable)]
#[diesel(table_name = schema::music)]
pub struct NewMusic<'a> {
    pub title: &'a str,
    pub path: &'a str,
}