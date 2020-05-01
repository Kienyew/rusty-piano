use piston_window::Key;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum KeyState {
    Neutral,
    JustPressed,
    Holding,
}

pub enum KeyEventType {
    KeyPressed,
    KeyReleased,
}

pub struct KeyboardControl {
    pub states: HashMap<Key, KeyState>,
    key_events: Vec<(Key, KeyEventType)>,
}

impl KeyboardControl {
    pub fn new() -> KeyboardControl {
        KeyboardControl {
            states: HashMap::new(),
            key_events: Vec::new(),
        }
    }

    pub fn key_press_event(&mut self, key: Key) {
        let key_state = self.states.entry(key).or_insert(KeyState::Neutral);
        *key_state = match *key_state {
            KeyState::Neutral => KeyState::JustPressed,
            KeyState::JustPressed => KeyState::Holding,
            KeyState::Holding => KeyState::Holding,
        };

        if *key_state == KeyState::JustPressed {
            self.key_events.push((key, KeyEventType::KeyPressed));
        }

    }

    pub fn key_release_event(&mut self, key: Key) {
        self.states.insert(key, KeyState::Neutral);
        self.key_events.push((key, KeyEventType::KeyReleased));
    }

    pub fn poll_event(&mut self) -> std::vec::Drain<(Key, KeyEventType)>  {
        self.key_events.drain(..)
    }
}
