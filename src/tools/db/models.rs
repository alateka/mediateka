use diesel::prelude::*;
use crate::tools::db::schema;
// ================( Models )===================================
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::music)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Music {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub artist: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::video)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub path: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::image)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Image {
    pub id: i32,
    pub title: String,
    pub path: String
}
// ================( Models )===================================

// ================( Insertable )===================================
#[derive(Insertable)]
#[diesel(table_name = schema::music)]
pub struct NewMusic<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub artist: &'a str
}
// ================( Insertable )===================================