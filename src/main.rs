use ncurses as nc;

mod core;
mod ui;

const KEY_A: i32 = b'a' as i32;
const KEY_D: i32 = b'd' as i32;
const ESC: i32 = 27;

fn main() {
    ui::curses_init();

    let (border_screen, inner_screen) = ui::create_screens();

    nc::wtimeout(inner_screen, 100);
    nc::keypad(inner_screen, true);

    let mut game = core::Game::new();

    loop {
        ui::erase_screens(inner_screen, border_screen);
        nc::box_(border_screen, 0, 0);
        ui::draw_landed_tetrominos(inner_screen, &game.grid);
        ui::draw_tetromino(inner_screen, &game.tetromino);
        ui::refresh_screens(inner_screen);

        game.handle_falling();

        let user_input = nc::wgetch(inner_screen);
        match user_input {
            nc::KEY_LEFT => {
                if let Err(_) = game.tetromino.move_sideways(core::Direction::Left) {
                    continue;
                }
            }
            nc::KEY_RIGHT => {
                if let Err(_) = game.tetromino.move_sideways(core::Direction::Right) {
                    continue;
                }
            }
            nc::KEY_DOWN => {
                if let Err(_) = game.tetromino.move_down() {
                    continue;
                }
            }
            KEY_A => {
                game.tetromino.rotate(core::Direction::Left);
            }
            KEY_D => {
                game.tetromino.rotate(core::Direction::Right);
            }
            ESC => break,
            _ => {}
        }
    }
    ui::curses_teardown();
}
