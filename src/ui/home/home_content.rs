use gtk::{prelude::WidgetExt, Box, Button, Label, Notebook, Orientation};

use crate::{tools::{enums::table::TableType, i18n::en::get_en}, ui::base::buttons::folder_add_button::build};

use super::tabs::{image::image_container::ImageTab, music::music_container::MusicTab, video::video_container::VideoTab};

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
        let home_content: Notebook = Notebook::builder()
            .tab_pos(gtk::PositionType::Top)
            .build();

        // Build button to check music folder
        let music_check_button: Button = build(
            get_en().check_music_folder,
            TableType::Music
        );

        // Build music tab
        let music_tab: MusicTab = MusicTab::new(
            Box::builder()
                .orientation(Orientation::Vertical)
                .build(),
            music_check_button
        );

        // Build button to check video folder
        let video_check_button: Button = build(
            get_en().check_video_folder,
            TableType::Video
        );

        // Build video tab
        let video_tab: VideoTab = VideoTab::new(
            Box::builder()
                .orientation(Orientation::Vertical)
                .build(),
            video_check_button
        );

        // Build button to check image folder
        let image_check_button: Button = build(
            get_en().check_image_folder,
            TableType::Image
        );

        // Build image tab
        let image_tab: ImageTab = ImageTab::new(
            Box::builder()
                .orientation(Orientation::Vertical)
                .build(),
            image_check_button
        );

        // Expand content on the window
        home_content.set_vexpand(true);
        home_content.set_hexpand(true);

        // Prepare tabs
        home_content.append_page(&music_tab.build(), Some(&self.music_label));
        home_content.append_page(&video_tab.build(), Some(&self.video_label));
        home_content.append_page(&image_tab.build(), Some(&self.image_label));


        home_content
    }
}


  