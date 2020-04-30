use crate::game::{Game, GameSettings};
use piston_window::{Context, Graphics, Rectangle};

pub struct GameView {
    game_settings: GameSettings,
}

impl GameView {
    pub fn new(settings: GameSettings) -> GameView {
        GameView {
            game_settings: settings,
        }
    }

    pub fn draw<G: Graphics>(&self, game: &Game, c: &Context, g: &mut G) {
        self.draw_background(c, g);
        // self.draw_piano_lines_background(c, g);
        // self.draw_piano_lines_sidelines(c, g);
        self.draw_touch_blocks(game, c, g);
        self.draw_music_nodes(game, c, g);
    }

    fn draw_background<G: Graphics>(&self, c: &Context, g: &mut G) {
        let rect = [0.0, 0.0, self.game_settings.screen_width, self.game_settings.screen_height];
        Rectangle::new(self.game_settings.background_color).draw(rect, &c.draw_state, c.transform, g);
    }


    fn draw_music_nodes<G: Graphics>(
        &self,
        game: &Game,
        c: &Context,
        g: &mut G,
    ) {
        for (i, piano_line) in game.piano_lines.iter().enumerate() {
            let i = i as f64;
            for music_node in &piano_line.music_nodes {
                let left = self.compute_left_offset() + i * (self.game_settings.line_width + self.game_settings.line_margin);
                let rect = [left, music_node.y, self.game_settings.line_width, music_node.height];
                Rectangle::new(self.game_settings.music_node_color).draw(rect, &c.draw_state, c.transform, g)
            }
        }
    }

    fn draw_touch_blocks<G: Graphics>(
        &self,
        game: &Game,
        c: &Context,
        g: &mut G,
    ) {
        for (i, piano_line) in game.piano_lines.iter().enumerate() {
            let i = i as f64;
            let touch_block = &piano_line.touch_block;
            let left = self.compute_left_offset() + i * (self.game_settings.line_width + self.game_settings.line_margin);
            let rect = [left, touch_block.y, self.game_settings.line_width, touch_block.height];
            Rectangle::new(self.game_settings.touch_block_color).draw(rect, &c.draw_state, c.transform, g)
        }
    }


    fn compute_left_offset(&self) -> f64 {
        (self.game_settings.screen_width
         - (self.game_settings.line_width * self.game_settings.line_count as f64)
         - (self.game_settings.line_margin * (self.game_settings.line_count - 1) as f64))
            / 2.0
    }
}