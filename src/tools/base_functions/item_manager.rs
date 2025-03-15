use gtk::{ListBox, SelectionMode};


// Build a list to add items
pub fn create_list() -> ListBox {
    let list: ListBox = ListBox::builder()
        .margin_top(15)
        .margin_end(15)
        .margin_bottom(15)
        .margin_start(15)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();

    list
}
