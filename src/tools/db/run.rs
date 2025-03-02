use diesel::prelude::*;

use crate::tools::db::models::NewMusic;

use super::models::Music;

pub struct DB<'a> {
    path: &'a str
}

impl<'a> DB<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    fn connect(&mut self) -> SqliteConnection {
        SqliteConnection::establish(self.path)
            .unwrap_or_else(|_| panic!("Error connecting to {}", self.path))
    }
    
    pub fn create_music(&mut self, title: &str, path: &str) -> Music {
        use crate::tools::db::schema::music;
    
        let new_music: NewMusic<'_> = NewMusic { title, path };
    
        diesel::insert_into(music::table)
            .values(&new_music)
            .returning(Music::as_returning())
            .get_result(&mut self.connect())
            .expect("Error saving new music")
    }
}