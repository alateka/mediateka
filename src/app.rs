use gtk::{prelude::BoxExt, Box, Label, Orientation};

use crate::{tools::i18n::en::get_en, ui::{header::{about::build_about, header::HeaderApp}, home::home_content::HomeContent}};

pub struct MediaTekaApp<'a> {
    pub title: &'a str
}

impl<'a> MediaTekaApp<'a> {
    
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }
    
    pub fn build(&self) -> Box {

        // Create the box to append main items
        let main_box: Box = Box::new(Orientation::Vertical, 7);

        // Create the app header
        let header_app: HeaderApp = HeaderApp::new(build_about());

        //Create the main 
        let home_content: HomeContent = HomeContent::new(
            Label::new(Some(get_en().music)),
            Label::new(Some(get_en().video)),
            Label::new(Some(get_en().image))
        );

        // Add header bar on main window
        main_box.append(&header_app.build());

        // Add home grid on main window
        main_box.append(&home_content.build());

        main_box
    }
}