use std::sync::MutexGuard;

use crate::{structs::Emoji, ui_options};
use gtk4::{CssProvider, StyleContext, gdk::Display, prelude::*};

pub const EMOJIS_TEXT: &str = include_str!("../emojies.txt");
pub const STYLES_CSS: &str = include_str!("../styles.css");

pub fn update_entries(
    query: &str,
    selected_index: &usize,
    emojis: &[Emoji],
    entries_box: &MutexGuard<'_, gtk4::Box>,
) -> Vec<Emoji> {
    let tokens: Vec<&str> = query.split(" ").collect();

    let mut filtered: Vec<(i32, &Emoji)> = emojis
        .iter()
        .map(|emoji| {
            let mut rating = 0;
            for token in &tokens {
                if emoji.name.to_lowercase().contains(&token.to_lowercase()) {
                    rating += 1;
                }
            }
            (rating, emoji)
        })
        .filter(|item| item.0 > 0)
        .take(ui_options::MAX_RESULTS)
        .collect();

    filtered.sort_by(|a, b| b.0.cmp(&a.0));

    // Clear old ones
    while let Some(first) = entries_box.first_child() {
        entries_box.remove(&first);
    }

    let mut result: Vec<Emoji> = Vec::new();
    // Re "draw" them
    for (i, emoji) in filtered.into_iter().enumerate() {
        let mut name: String = emoji.1.name.clone();
        if name.len() > ui_options::MAX_EMOJI_NAME {
            name = name.split_at(ui_options::MAX_EMOJI_NAME - 1).0.to_string() + "...";
        }
        let label = gtk4::Label::new(Some(&format!("{} - {}", emoji.1.emoji, name)));
        if &i == selected_index {
            label.add_css_class("entry-selected");
        } else {
            label.add_css_class("entry");
        }

        entries_box.append(&label);

        let emoji = emoji.1.clone();
        result.push(emoji);
    }
    result
}
pub fn update_selected(
    old_selected_index: &usize,
    selected_index: &usize,
    entries_box: &MutexGuard<'_, gtk4::Box>,
) {
    entries_box
        .observe_children()
        .item(old_selected_index.to_owned() as u32)
        .unwrap()
        .downcast::<gtk4::Widget>()
        .unwrap()
        .set_css_classes(&["entry"]);

    entries_box
        .observe_children()
        .item(selected_index.to_owned() as u32)
        .unwrap()
        .downcast::<gtk4::Widget>()
        .unwrap()
        .set_css_classes(&["entry-selected"]);
}

pub fn load_emojis(emojis: &mut Vec<Emoji>, data: &str) {
    let lines = data.lines();

    // To remove dublicates
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();

    for line in lines {
        if !seen.insert(line) {
            continue;
        }

        let parts = line.split_once(" ").unwrap_or_default();
        let sanitized_name = parts.1.replace("&", "and");

        emojis.push(Emoji::new(&sanitized_name, parts.0));
    }
}
pub fn load_recent_emojis(recent_emojis: &mut Vec<Emoji>) {
    if !std::fs::exists("recent.txt").unwrap_or_default() {
        std::fs::write("recent.txt", "").unwrap();
        return;
    }
    let data = std::fs::read_to_string("recent.txt").unwrap();

    load_emojis(recent_emojis, &data);
}
pub fn save_recent_emojis(recent_emojis: &[Emoji]) {
    let mut lines: Vec<String> = Vec::new();

    for emoji in recent_emojis.iter().take(ui_options::MAX_RESULTS) {
        lines.push(emoji.as_string());
    }

    let data = lines.join("\n");

    std::fs::write("recent.txt", data).unwrap();
}

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(STYLES_CSS);
    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn write_to_clipboard(data: &str) {
    // Check if wl-copy work

    if std::process::Command::new("which")
        .arg("wl-copy")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        std::process::Command::new("wl-copy")
            .arg(data)
            .spawn()
            .map_err(|e| format!("Failed to execute wl-copy: {}", e))
            .unwrap()
            .wait()
            .unwrap();
    }
}
