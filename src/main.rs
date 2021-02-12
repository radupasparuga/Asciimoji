extern crate iui;
use iui::prelude::*;
use iui::controls::{Entry, VerticalBox, Group, Label };
use std::rc::Rc;
use std::cell::RefCell;

struct State {
    entry_val: String,
}

fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");
    let state = Rc::new(RefCell::new(State { entry_val: "".into() }));

    let (input_group, mut entry) = {
        let mut input_group = Group::new(&ui, "Search");
        let mut input_vbox = VerticalBox::new(&ui);
        input_vbox.set_padded(&ui, true);

        let entry = Entry::new(&ui);
        input_vbox.append(&ui, entry.clone(), LayoutStrategy::Compact);

        input_group.set_child(&ui, input_vbox);
        (input_group, entry)
    };

    let (output_group, text_label) = {
        let mut output_group = Group::new(&ui, "Results");
        let mut output_vbox = VerticalBox::new(&ui);
        let text_label = Label::new(&ui, "");

        output_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);
        output_group.set_child(&ui, output_vbox);
        (output_group, text_label)
    };

    let mut vbox = VerticalBox::new(&ui);
    vbox.append(&ui, input_group, LayoutStrategy::Compact);
    vbox.append(&ui, output_group, LayoutStrategy::Stretchy);

    let mut window = Window::new(&ui, "Asciimoji", 1280, 720, WindowType::NoMenubar);
    window.set_child(&ui, vbox);
    window.show(&ui);

    entry.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().entry_val = val; }
    });

    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut text_label = text_label.clone();
        move || {
            let state = state.borrow();

            text_label.set_text(&ui, &format!("Text: {}", state.entry_val));
        }
    });

    event_loop.run(&ui);
}
