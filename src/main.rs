use std::sync::{Arc, Mutex};

use gtk4::{Application, ApplicationWindow, Window, prelude::*};
use gtk4_layer_shell::{Edge, KeyboardMode, LayerShell};

use crate::structs::{Emoji, State};

mod structs;
mod ui_options;
mod utils;

fn main() {
    let application = gtk4::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_activate(|app| {
        utils::load_css();

        let mut state = State::new();

        utils::load_emojis(&mut state.emojis, utils::EMOJIS_TEXT);
        utils::load_recent_emojis(&mut state.recent_emojis);

        let arc_state = Arc::new(Mutex::new(state));

        let window = build_ui(app, arc_state.clone());

        keyboard_events(&window, arc_state);
    });

    application.run();
}

fn build_ui(app: &Application, state: Arc<Mutex<State>>) -> ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    window.add_css_class("body");

    window.init_layer_shell();
    // Set it to overlay's wayland layer
    window.set_layer(gtk4_layer_shell::Layer::Overlay);
    // Capture all keybard input
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    // Set margin and anchors
    for edge in [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom] {
        window.set_margin(edge, ui_options::MARGIN);

        window.set_anchor(edge, false);
    }
    // Ui elements
    // Inside scrolled window to be able to contain everything
    let root = gtk4::ScrolledWindow::new();
    root.add_css_class("root");
    root.set_size_request(ui_options::WIDTH, ui_options::HEIGHT);

    // Main box for holding everything
    let main = gtk4::Box::new(gtk4::Orientation::Vertical, ui_options::MARGIN);
    main.add_css_class("main");
    // Scrollable window to be able to hold entries
    let entries = gtk4::ScrolledWindow::new();
    entries.add_css_class("entries-box");

    entries.set_size_request(ui_options::WIDTH, 435);

    let entries_box = gtk4::ListBox::new();
    entries_box.add_css_class("entries-box");
    entries_box.set_selection_mode(gtk4::SelectionMode::Single);

    entries.set_child(Some(&entries_box));

    let arc_entries_box = Arc::new(Mutex::new(entries_box));

    {
        let mut state = state.lock().unwrap();
        state.entries_box = Some(arc_entries_box.clone());

        let entries_box = arc_entries_box.lock().unwrap();

        let results = utils::update_entries("", &state.recent_emojis, &entries_box);
        state.results = results;
    }

    let text_box = gtk4::Text::new();

    text_box.set_placeholder_text(Some("Search..."));
    text_box.add_css_class("input-field");

    let arc_text_box = Arc::new(Mutex::new(text_box.clone()));

    {
        let mut state = state.lock().unwrap();
        state.text_box = Some(arc_text_box.clone());
    }
    arc_text_box.lock().unwrap().connect_changed(move |text| {
        let query = text.text().to_string();
        let mut state = state.lock().unwrap();

        let entries_box = arc_entries_box.lock().unwrap();

        let results = if query.is_empty() {
            utils::update_entries("", &state.recent_emojis, &entries_box)
        } else {
            utils::update_entries(&query, &state.emojis, &entries_box)
        };

        state.results = results;

        state.query = query.clone();
    });

    main.append(&text_box);
    main.append(&entries);

    root.set_child(Some(&main));

    window.set_child(Some(&root));

    window.show();

    window
}
fn keyboard_events(window: &ApplicationWindow, state: Arc<Mutex<State>>) {
    let keyboard_event = gtk4::EventControllerKey::new();

    let state_clone = state.clone();

    keyboard_event.connect_key_pressed(move |_, key, _, _| {
        let grab_focus_entries = || {
            // Focus on select
            let state = state.lock().unwrap();

            let arc_entries_box = state.entries_box.clone().unwrap();
            let entries_box = arc_entries_box.lock().unwrap();

            if state.results.is_empty() {
                return;
            }

            entries_box.row_at_index(0).unwrap().grab_focus();
        };

        match key.name().unwrap_or_default().to_lowercase().as_str() {
            "escape" => {
                // Exit on escape
                std::process::exit(0);
            }
            "up" => {
                grab_focus_entries();
            }
            "down" => {
                grab_focus_entries();
            }
            _ => {
                // Focus on text_box
                let state = state.lock().unwrap();

                let text_box_arc = state.text_box.clone().unwrap();
                let text_box = text_box_arc.lock().unwrap();

                text_box.grab_focus();
            }
        }
        false.into()
    });

    keyboard_event.connect_key_released(move |_, key, _, _| {
        let mut state = state_clone.lock().unwrap();

        // Check if enter is pressed
        if key.name().unwrap_or_default().to_lowercase() == "return" {
            let arc_entries_box = state.entries_box.clone().unwrap();
            let entries_box = arc_entries_box.lock().unwrap();

            let index = if let Some(selected_row) = entries_box.selected_row() {
                selected_row.index()
            } else {
                -1
            };

            if index == -1 {
                return;
            }

            let emoji = state.results[index as usize].clone();
            utils::write_to_clipboard(&emoji.emoji);

            state.recent_emojis.insert(0, emoji);

            utils::save_recent_emojis(&state.recent_emojis);

            std::process::exit(0);
        }
    });

    window.add_controller(keyboard_event);
}
