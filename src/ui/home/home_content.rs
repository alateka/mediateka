use gtk::{Box, Label, Notebook, Orientation};

use crate::{tools::{enums::table::TableType, i18n::en::get_en}, ui::base::buttons::folder_add_button::GetPathButton};

use super::tabs::music::container::MusicTab;

pub struct HomeContent {
    music_label: Label,
    video_label: Label,
    image_label: Label
}

impl HomeContent {
    
    pub fn new(music_label: Label, video_label: Label, image_label: Label) -> Self {
        Self { music_label, video_label, image_label }
    }
    
    pub fn build(self) -> Notebook {

        // Build items content 
        let home_content: Notebook = Notebook::new();

        // Build button to check music folder
        let music_check_button: GetPathButton = GetPathButton::new(
            get_en().check_music_folder,
            TableType::Music
        );

        // Build music tab
        let music_tab: MusicTab = MusicTab::new(
            Box::new(Orientation::Vertical, 7),
            music_check_button.build()
        );

        // Prepare tabs
        home_content.append_page(&music_tab.build(), Some(&self.music_label));
        home_content.append_page(&Label::new(Some("Hola")), Some(&self.video_label));
        home_content.append_page(&Label::new(Some("Adios")), Some(&self.image_label));


        home_content
    }
}


  