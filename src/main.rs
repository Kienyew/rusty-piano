mod game;
mod game_object;
mod math;
mod view;

use piston_window::Key;
use piston_window::PistonWindow;
use piston_window::RenderEvent; // e.render_args()

fn main() {
    let window_width = 1024.0;
    let window_height = 768.0;

    let mut window: PistonWindow =
        piston_window::WindowSettings::new("piano-tiles-rs", [window_width, window_height])
            .exit_on_esc(true)
            .build()
            .expect("Piston Window could not be initialized");

    let game_settings = game::GameSettings {
        screen_width: window_width,
        screen_height: window_height,
        line_height: window_height,
        line_margin: 40.0,
        line_width: 100.0,
        margin_bottom: 20.0,
        line_sideline_radius: 1.5,
        background_color: [0.9, 0.9, 0.9, 1.0],
        touch_block_color: [0.1, 0.2, 0.3, 1.0],
        music_node_color: [0.1, 0.1, 0.1, 0.9],
        line_background_color: [0.8, 0.8, 0.8, 1.0],
        line_sideline_color: [0.6, 0.6, 0.6, 1.0],
        line_count: 4,
        drop_speed: 500.0,
        music_node_height: 161.8,
        touch_block_height: 161.8 / 4.0, 
        keybinds: vec![Key::H, Key::J, Key::K, Key::L],
    };

    let mut game = game_settings.build();
    while let Some(e) = window.next() {
        game.event(&e);
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                game.draw(&c, g);
            });
        }
    }
    game.exit();
}
