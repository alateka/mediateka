use crate::tools::{db::run::DB, enums::table::TableType};
use crate::tools::i18n::en::get_en;

use adw::Window;
use gtk::{prelude::{DialogExt, WidgetExt, GtkWindowExt}, Button, FileChooserAction, FileChooserDialog, ResponseType};
use std::fs::read_dir;
use gtk::prelude::{ButtonExt, FileChooserExt};
use gtk::prelude::FileExt;

    
pub fn build(button_text: &str, table_type: TableType) -> Button {
    let button: Button = Button::with_label(button_text);
    button.connect_clicked( move |_| click(table_type.clone()));
    button
}

fn click(table_type: TableType) {
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

            let mut db: DB = DB::new("mediateka.db");

            // If the folder is selected, then it will save its paths
            if let Some(file) = dialog.file() {
                
                for file_path in read_dir(file.path().unwrap()).unwrap() {
                    
                    let f_path: String = file_path.unwrap().path().to_str().expect("Failed to get string representation of Path").to_string();
                    
                    let splited_path: Vec<&str> = f_path.split('/').collect();

                    let file_name: &str = splited_path[splited_path.len()-1];
                    let splited_file_name: Vec<&str> = file_name.split('.').collect();

                    match table_type {
                        TableType::Music => db.create_music(splited_file_name[0], &f_path, "..."),
                        TableType::Video => db.create_video(splited_file_name[0], &f_path),
                        TableType::Image => db.create_image(splited_file_name[0], &f_path)
                    }
                }
            }
        }
        dialog.close();
    });
}
