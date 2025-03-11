use adw::{prelude::ActionRowExt, ActionRow};
use gtk::{prelude::BoxExt, Box, Button, ListBox, SelectionMode};

use crate::{tools::db::run::DB, DATABASE};

pub struct ImageTab {
    pub base_content: Box,
    pub image_check_button: Button
}


impl ImageTab {

    pub fn new(base_content: Box, image_check_button: Button) -> Self {
        Self { base_content, image_check_button }
    }
    
    pub fn build(self) -> Box {
        
        // Prepare base content to show all music on the tab
        self.base_content.set_spacing(45);

        // Append the folder select button
        self.base_content.append(&self.image_check_button);

        // Build a list to add items
        let list: ListBox = ListBox::builder()
            .margin_top(15)
            .margin_end(15)
            .margin_bottom(15)
            .margin_start(15)
            .selection_mode(SelectionMode::None)
            .css_classes(vec![String::from("boxed-list")])
            .build();

        // Get all music saved on disk
        let results = DB::new(DATABASE).get_image();


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

        self.base_content
    }
}