
use adw::Window;
use gtk::{prelude::{ButtonExt, DialogExt, WidgetExt, FileChooserExt, FileExt, GtkWindowExt}, Button, FileChooserAction, FileChooserDialog, ResponseType};
use std::fs::read_dir;

use crate::tools::{fs::FileIO, i18n::en::get_en};

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

                    // If the folder is selected, then it will save its paths
                    if let Some(file) = dialog.file() {
                        
                        let paths = read_dir(
                            file.path().unwrap()
                        ).unwrap();

                        let file_io: FileIO = FileIO::new("db.csv");
                        let mut content: String = String::from("");

                        for path in paths {
                            content += path.unwrap().path().to_str().expect("Failed to get string representation of Path");
                            content += ";";
                        }
                        
                        let _ = file_io.write(content);
                    }
                }
                dialog.close();
            });
        });

        button
    }
}