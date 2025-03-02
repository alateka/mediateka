use gtk::{Button, Image, AboutDialog, License};
use gio::glib;
use adw::{prelude::{ButtonExt, GtkWindowExt}, ApplicationWindow};

use crate::{AUTHORS, PROGRAM_NAME, VERSION, WEBSITE};


pub fn build_about() -> Button {
    // build about window
    let about_window: ApplicationWindow = ApplicationWindow::builder().build();

    // Build about button with help icon
    let about_button: Button = Button::new();
    let icon: Image = Image::from_icon_name("gtk-info");

    about_button.set_child(Some(&icon));
    about_button.connect_clicked(glib::clone!(
        #[weak]
        about_window,
        move |_| {
            let dialog = AboutDialog::builder()
                .transient_for(&about_window)
                .modal(false)
                .program_name(PROGRAM_NAME)
                .version(VERSION)
                .website(WEBSITE)
                .license_type(License::Gpl30)
                .authors(AUTHORS)
                .build();

            dialog.present();
        }
    ));
    return about_button;
}

