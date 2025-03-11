use diesel::prelude::*;

use crate::tools::db::models::NewMusic;
use crate::tools::db::schema::music;

use super::{models::{self, Image, Music, NewImage, NewVideo, Video}, schema::{image, video}};

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

    pub fn get_music(&mut self) -> Vec<models::Music> {
    
        let results: Vec<Music> = music::table.load(
            &mut self.connect()
        ).expect("Error loading items");

        results
    }

    pub fn get_video(&mut self) -> Vec<models::Video> {
    
        let results: Vec<Video> = video::table.load(
            &mut self.connect()
        ).expect("Error loading items");

        results
    }

    pub fn get_image(&mut self) -> Vec<models::Image> {
    
        let results: Vec<Image> = image::table.load(
            &mut self.connect()
        ).expect("Error loading items");

        results
    }
}