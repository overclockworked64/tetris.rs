#[cfg(test)]
use rstest_reuse;

use crate::core::{Direction, Game};
use ncurses as nc;

mod core;
mod ui;

const KEY_A: i32 = b'a' as i32;
const KEY_D: i32 = b'd' as i32;
const KEY_S: i32 = b's' as i32;
const KEY_P: i32 = b'p' as i32;
const ESC: i32 = 27;

fn main() {
    ui::curses_init();

    let (border_screen, inner_screen) = ui::create_screens();

    nc::wtimeout(inner_screen, 100);
    nc::keypad(inner_screen, true);

    let mut game = Game::new();

    loop {
        ui::erase_screens(inner_screen, border_screen);
        nc::box_(border_screen, 0, 0);

        ui::draw_landed_tetrominos(inner_screen, &game.grid);
        ui::draw_tetromino(inner_screen, &game.tetromino);
        ui::draw_score(game.score);

        ui::refresh_screens(inner_screen);

        let user_input = nc::wgetch(inner_screen);

        if user_input == KEY_P {
            game.paused = !game.paused;
        }

        if !game.paused {
            game.handle_falling();
            game.clear_rows();
            match user_input {
                nc::KEY_LEFT => {
                    if game.tetromino.move_sideways(Direction::Left).is_err() {
                        continue;
                    }
                }
                nc::KEY_RIGHT => {
                    if game.tetromino.move_sideways(Direction::Right).is_err() {
                        continue;
                    }
                }
                nc::KEY_DOWN => {
                    if game.tetromino.move_down().is_err() {
                        continue;
                    }
                }
                KEY_A => {
                    if game.tetromino.rotate(Direction::Left).is_err() {
                        continue;
                    }
                }
                KEY_D | nc::KEY_UP => {
                    if game.tetromino.rotate(Direction::Right).is_err() {
                        continue;
                    }
                }
                KEY_S => {
                    game.tetromino.move_all_the_way_down();
                }
                ESC => break,
                _ => {}
            }
        }
    }
    ui::curses_teardown();
}
