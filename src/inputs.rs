use ggez::event::KeyCode;
use std::collections::HashSet;

pub struct Inputs {
    keys_pressed: HashSet<KeyCode>,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            keys_pressed: HashSet::new(),
        }
    }

    pub fn key_down(&mut self, keycode: KeyCode) {
        self.keys_pressed.insert(keycode);
    }

    pub fn key_up(&mut self, keycode: KeyCode) {
        self.keys_pressed.remove(&keycode);
    }

    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.keys_pressed.contains(&keycode)
    }
}
