use adw::HeaderBar;
use gtk::Button;

pub struct HeaderApp {
    about_button: Button
}

impl HeaderApp {
    pub fn new(about_button: Button) -> Self {
        Self { about_button }
    }

    pub fn build(&self) -> HeaderBar {
        let header_bar: HeaderBar = HeaderBar::new();
        header_bar.pack_start(&self.about_button);
        header_bar
    }
}

