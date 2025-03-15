use adw::{prelude::ActionRowExt, ActionRow};
use gtk::{prelude::BoxExt, Box, Button, ListBox, PolicyType, ScrolledWindow};
use std::rc::Rc;

use crate::{tools::{base_functions::item_manager::create_list, db::{models, run::DB}}, DATABASE};

pub struct VideoTab {
    pub base_content: Box,
    pub video_check_button: Button
}


impl VideoTab {

    pub fn new(base_content: Box, video_check_button: Button) -> Self {
        Self { base_content, video_check_button }
    }

    fn create_rows(item: models::Video) -> ActionRow {
        // Add item to Rc
        let item_rc = Rc::new(item);
    
        // Create row with click action
        let row = ActionRow::builder()
            .activatable(true)
            .title(item_rc.title.clone())
            .build();
    
        let item_clone = Rc::clone(&item_rc);
        row.connect_activated( move |_| {
            eprintln!("Path: {:?}", item_clone.path);
        });
    
        row
    }
    
    pub fn build(self) -> ScrolledWindow {
        
        // Get all music saved on disk
        let results: Vec<models::Video> = DB::new(DATABASE).get_video();

        // Append the folder select button
        if results.len() == 0 {
            self.base_content.append(&self.video_check_button);
        }

        // Build a list to add items
        let list: ListBox = create_list();

        // Add the items (Music)
        for item in results {
            list.append(&Self::create_rows(item))
        }

        self.base_content.append(&list);

        ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Automatic)
            .child(&self.base_content)
            .build()
    }
}