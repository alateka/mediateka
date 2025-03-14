use adw::{prelude::ActionRowExt, ActionRow};
use gtk::{prelude::BoxExt, Box, Button, ListBox, PolicyType, ScrolledWindow};

use crate::{tools::{base_functions::item_manager::create_list, db::{models, run::DB}}, DATABASE};

pub struct ImageTab {
    pub base_content: Box,
    pub image_check_button: Button
}


impl ImageTab {

    pub fn new(base_content: Box, image_check_button: Button) -> Self {
        Self { base_content, image_check_button }
    }
    
    pub fn build(self) -> ScrolledWindow {
        
        // Get all music saved on disk
        let results: Vec<models::Image> = DB::new(DATABASE).get_image();

        // Append the folder select button
        if results.len() == 0 {
            self.base_content.append(&self.image_check_button);
        }

        // Build a list to add items
        let list: ListBox = create_list();

        // Add the items (Music)
        for item in results {

            let row = ActionRow::builder()
                .activatable(true)
                .title(item.title)
                .build();

            row.connect_activated(|_| {
                eprintln!("Clicked");
            });

            list.append(&row)
        }

        self.base_content.append(&list);

        ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Automatic)
            .child(&self.base_content)
            .build()
    }
}