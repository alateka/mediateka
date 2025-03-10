use gtk::{prelude::BoxExt, Box, Button};

use crate::tools::db::run::DB;

pub struct MusicTab {
    pub base_content: Box,
    pub music_check_button: Button
}


impl MusicTab {

    pub fn new(base_content: Box, music_check_button: Button) -> Self {
        Self { base_content, music_check_button }
    }
    
    pub fn build(self) -> Box {
        self.base_content.set_spacing(45);
        self.base_content.append(&self.music_check_button);
        DB::new("mediateka.db").get_music();
        self.base_content 
    }
}