use super::I18n;

pub fn get_en() -> I18n<'static> {
    return I18n {
        music:              "Music",
        video:              "Video",
        image:              "Image",
        check_music_folder: "Check music folder",
        choose_a_folder:    "Choose a folder",
        open:               "Open",
        cancel:             "Cancel",
        select:             "Select"
     };
}