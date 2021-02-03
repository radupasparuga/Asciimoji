extern crate iui;
use iui::prelude::*;
use iui::controls::{Entry, Label, VerticalBox};


fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Asciimoji", 600, 200, WindowType::NoMenubar);

    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let mut label_text = String::new();
    label_text.push_str("Search for an asciimoji below!\n");
    let label = Label::new(&ui, &label_text);

    let mut search = Entry::new(&ui);

    vbox.append(&ui, label, LayoutStrategy::Stretchy);
    vbox.append(&ui, search, LayoutStrategy::Stretchy);

    win.set_child(&ui, vbox);
    win.show(&ui);
    ui.main();
}
