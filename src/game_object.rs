use std::collections::VecDeque;
use crate::game::GameSettings;

#[derive(Clone)]
pub struct PianoLine {
    pub line_height: f64,
    pub drop_speed: f64,
    pub music_node_height: f64,
    pub music_nodes: VecDeque<MusicNode>,
    pub touch_block: TouchBlock,
}

impl PianoLine {
    pub fn new(game_settings: &GameSettings) -> PianoLine
    {
        PianoLine {
            line_height: game_settings.line_height,
            drop_speed: game_settings.drop_speed,
            music_node_height: game_settings.music_node_height,
            music_nodes: VecDeque::new(),
            touch_block: TouchBlock {
                y: game_settings.line_height - game_settings.touch_block_height - game_settings.margin_bottom,
                height: game_settings.touch_block_height,
            },
        }
    }

    pub fn slide_down(&mut self, dt: f64) {
        for music_node in &mut self.music_nodes {
            music_node.y += self.drop_speed * dt;
        }
    }

    pub fn pop_bottom_node(&mut self) {
        self.music_nodes.pop_back();
    }

    pub fn get_bottom_node(&self) -> Option<&MusicNode> {
        self.music_nodes.back()
    }

    pub fn get_top_node(&self) -> Option<&MusicNode> {
        self.music_nodes.front()
    }

    pub fn push_new_node(&mut self, y: f64) {
        self.music_nodes.push_front(MusicNode { y, height: self.music_node_height })
    }
}

#[derive(Clone)]
pub struct MusicNode {
    pub y: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct TouchBlock {
    pub y: f64,
    pub height: f64,
}
