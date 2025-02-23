// Load modules
pub mod app;
pub mod ui;
pub mod tools;

// Load libraries
use adw::{Application, ApplicationWindow};
use app::MediaTekaApp;
use gio::{glib, prelude::{ApplicationExt, ApplicationExtManual}};
use gtk::{prelude::GtkWindowExt, License};

const APP_ID: &str = "es.alateka.mediateka";
const PROGRAM_NAME: &str = "MediaTeka";
const VERSION: &str = "0.0.1-alpha";
const WEBSITE: &str = "https://alateka.es";
const AUTHORS: [&str; 1] = ["ALATEKA"];
const LICENSE: License = License::Gpl30Only;

fn main() -> glib::ExitCode {

    // Start application
    let app = Application::builder()
    .application_id(APP_ID)
    .build();

    // Run application
    app.connect_activate(|app: &Application| {

        // Prepare app info
        let mediateka: MediaTekaApp = MediaTekaApp::new(
            PROGRAM_NAME
        );

        // Build main window
        let main_window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(mediateka.title)
        .default_width(900)
        .default_height(700)
        .content(&mediateka.build())
        .build();

        // Show main window
        main_window.present();
    });

    app.run()
}