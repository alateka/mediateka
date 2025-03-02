
use adw::Window;
use gtk::{prelude::{ButtonExt, DialogExt, WidgetExt, GtkWindowExt}, Button, FileChooserAction, FileChooserDialog, ResponseType};
use crate::tools::db::{models::Music, run, schema::music::dsl::*};
use diesel::prelude::*;

use crate::tools::i18n::en::get_en;

pub struct GetPathButton<'a> {
    button_text: &'a str
}

impl<'a> GetPathButton<'a> {

    pub fn new(button_text: &'a str) -> Self {
        Self { button_text }
    }
    
    pub fn build(self) -> Button {
        let button: Button = Button::with_label(self.button_text);

        button.connect_clicked( move |_| {

            // Build folder select dialog
            let dialog = FileChooserDialog::new(
                Some(get_en().choose_a_folder),
                Some(&Window::builder().build()),
                FileChooserAction::SelectFolder,
                &[
                    (get_en().select, ResponseType::Accept),
                    (get_en().cancel, ResponseType::Cancel)
                ]
            );
            dialog.show();

            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {

                    let connection: &mut SqliteConnection = &mut run::connect();
                    let results = music
                        .select(Music::as_select())
                        .load(connection)
                        .expect("Error loading music");
                
                    println!("Displaying {} posts", results.len());
                    for item in results {
                        println!("{}", item.title);
                    }
                }
                dialog.close();
            });
        });

        button
    }
}