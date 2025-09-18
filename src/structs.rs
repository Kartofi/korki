use std::sync::{Arc, Mutex};

pub struct State {
    pub query: String,

    pub results: Vec<Emoji>,



    pub emojis: Vec<Emoji>,

    pub recent_emojis: Vec<Emoji>,

    pub entries_box: Option<Arc<Mutex<gtk4::ListBox>>>,
    pub text_box: Option<Arc<Mutex<gtk4::Text>>>,
}
impl State {
    pub fn new() -> State {
        State {
            query: "".to_string(),
            results: Vec::new(),
           
            emojis: Vec::new(),
            recent_emojis: Vec::new(),

            entries_box: None,
            text_box: None
        }
    }
}
#[derive(Clone, Debug)]
pub struct Emoji {
    pub emoji: String,

    pub name: String,
}

impl Emoji {
    pub fn new(name: &str, emoji: &str) -> Emoji {
        Emoji {
            name: name.to_string(),
            emoji: emoji.to_string(),
        }
    }
    pub fn as_string(&self) -> String {
       format!("{} {}", self.emoji, self.name)
    }
}
