use ordered_float::OrderedFloat;
use piston_window::types::Color;
use piston_window::{Button, Context, GenericEvent, Graphics, Key, UpdateArgs};
use rand;
use rand::seq::SliceRandom;

use crate::game_object::PianoLine;
use crate::keyboard::{KeyEventType, KeyboardControl};
use crate::view::GameView;

#[derive(Clone)]
pub struct GameSettings {
    pub screen_height: f64,
    pub screen_width: f64,
    pub line_margin: f64,
    pub line_width: f64,
    pub line_height: f64,
    pub margin_bottom: f64,
    pub drop_speed: f64,
    pub touch_block_height: f64,
    pub music_node_height: f64,
    pub line_sideline_radius: f64,
    pub line_count: usize,
    pub keybinds: Vec<Key>,
    pub line_background_color: Color,
    pub background_color: Color,
    pub touch_block_color: Color,
    pub music_node_color: Color,
    pub line_sideline_color: Color,
}

impl GameSettings {
    pub fn build(self) -> Game {
        assert_eq!(self.line_count, self.keybinds.len());

        let piano_lines = vec![PianoLine::new(&self); self.line_count];

        Game {
            keyboard_control: KeyboardControl::new(),
            view: GameView::new(self.clone()),
            settings: self,
            piano_lines,
        }
    }
}

pub struct Game {
    pub keyboard_control: KeyboardControl,
    pub settings: GameSettings,
    pub piano_lines: Vec<PianoLine>,
    view: GameView,
}

impl Game {
    pub fn exit(&mut self) {
        println!("Good bye (o´ω`o) Have a nice day ( •̀ω•́ )")
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(u) = e.update_args() {
            self.update(u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.keyboard_control.key_press_event(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            self.keyboard_control.key_release_event(key);
        }

        for (key, event_type) in self.keyboard_control.poll_event().collect::<Vec<_>>(){
            match event_type {
                KeyEventType::KeyPressed => {
                    self.key_press(key);
                },
                KeyEventType::KeyReleased => {
                    self.key_release(key);
                },
            }
        }
    }

    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        self.view.draw(&self, c, g);
    }

    pub fn update(&mut self, u: UpdateArgs) {
        for piano_line in &mut self.piano_lines {
            piano_line.slide_down(u.dt);

            if let Some(bottom_music_node) = piano_line.get_bottom_node() {
                if bottom_music_node.y >= piano_line.line_height {
                    piano_line.pop_bottom_node();
                }
            }
        }

        let top_music_node = self
            .piano_lines
            .iter()
            .filter_map(|line| line.get_top_node())
            .min_by_key(|&music_node| OrderedFloat(music_node.y));

        let mut to_push = false;
        let mut new_music_node_y = 0.0;
        if let Some(top_music_node) = top_music_node {
            if top_music_node.y >= 0.0 {
                new_music_node_y = top_music_node.y - top_music_node.height;
                to_push = true;
            }
        } else {
            to_push = true;
        }

        if to_push {
            self.piano_lines
                .choose_mut(&mut rand::thread_rng())
                .unwrap()
                .push_new_node(new_music_node_y);
        }
    }

    fn key_press(&mut self, key: Key) {
        if let Some(line) = self
            .settings
                .keybinds
                .iter()
                .position(|&keybind| keybind == key)
        {
            self.as_press_touch_block(line);
        }
    }

    fn key_release(&mut self, _key: Key) { }

    pub fn as_press_touch_block(&mut self, line: usize) {
        use crate::math::line_intersect_length;
        use std::cmp::min;

        if let Some(piano_line) = self.piano_lines.get_mut(line) {
            if let Some(node) = piano_line.get_bottom_node() {
                let touch_block = &piano_line.touch_block;
                let intersect_length =
                    line_intersect_length(node.y, node.height, touch_block.y, touch_block.height);
                let intersect_ratio = intersect_length
                    / min(OrderedFloat(node.height), OrderedFloat(touch_block.height)).into_inner();

                if intersect_ratio > 0.0 {
                    piano_line.pop_bottom_node();
                }
            }
        }
    }
}
