use diesel::prelude::*;

use crate::tools::db::models::NewMusic;
use crate::tools::db::schema::music;

use super::{models::{Image, Music, NewImage, NewVideo, Video}, schema::{image, video}};

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
    
    pub fn create_music(&mut self, title: &str, path: &str, artist: &str) {
    
        let new_music: NewMusic<'_> = NewMusic { title, path, artist };
    
        diesel::insert_into(music::table)
            .values(&new_music)
            .returning(Music::as_returning())
            .get_result(&mut self.connect())
            .expect("Error saving new music");
    }

    pub fn create_video(&mut self, title: &str, path: &str) {
    
        let new_video: NewVideo<'_> = NewVideo { title, path };
    
        diesel::insert_into(video::table)
            .values(&new_video)
            .returning(Video::as_returning())
            .get_result(&mut self.connect())
            .expect("Error saving new video");
    }

    pub fn create_image(&mut self, title: &str, path: &str) {
    
        let new_image: NewImage<'_> = NewImage { title, path };
    
        diesel::insert_into(image::table)
            .values(&new_image)
            .returning(Image::as_returning())
            .get_result(&mut self.connect())
            .expect("Error saving new image");
    }

    pub fn get_music(&mut self) {
    
        let music = music::dsl::music
        .select(Music::as_select())
        .first(&mut self.connect())
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

        match music {
            Ok(Some(music)) => println!("Music with id: {} has a title: {}", music.id, music.title),
            Ok(None) => println!("Unable to find music"),
            Err(_) => println!("An error occured while fetching music"),
        }
    }
}