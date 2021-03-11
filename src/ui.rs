use crate::core::{Coord, Tetromino, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH};
use ncurses as nc;

const SCREEN_WIDTH: i32 = PLAYGROUND_WIDTH * 2;
const SCREEN_HEIGHT: i32 = PLAYGROUND_HEIGHT;

pub fn curses_init() {
    nc::setlocale(nc::LcCategory::all, "");
    nc::initscr();
    nc::nodelay(nc::stdscr(), true);
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nc::noecho();
    nc::keypad(nc::stdscr(), true);
}

pub fn curses_teardown() {
    nc::clear();
    nc::refresh();
    nc::doupdate();
    nc::endwin();
}

pub fn create_screens() -> (nc::WINDOW, nc::WINDOW) {
    let border_screen = nc::subwin(
        nc::stdscr(),
        1 + SCREEN_HEIGHT + 1,
        1 + SCREEN_WIDTH + 1,
        (nc::LINES() - SCREEN_HEIGHT) / 2 - 1,
        (nc::COLS() - SCREEN_WIDTH) / 2 - 1,
    );
    let inner_screen = nc::subwin(
        nc::stdscr(),
        SCREEN_HEIGHT,
        SCREEN_WIDTH,
        (nc::LINES() - SCREEN_HEIGHT) / 2,
        (nc::COLS() - SCREEN_WIDTH) / 2,
    );
    (border_screen, inner_screen)
}

pub fn erase_screens(border_screen: nc::WINDOW, inner_screen: nc::WINDOW) {
    nc::werase(inner_screen);
    nc::werase(border_screen);
    nc::werase(nc::stdscr());
}

pub fn refresh_screens(inner_screen: nc::WINDOW) {
    nc::refresh();
    nc::wrefresh(inner_screen);
}

pub fn draw_tetromino(screen: nc::WINDOW, tetromino: &Tetromino) {
    let tetrovec = tetromino.shape.to_vec(tetromino.current_rotation);
    for (rowidx, row) in tetrovec.iter().enumerate() {
        for (colidx, col) in row.iter().enumerate() {
            if tetrovec[rowidx][colidx] != 0 {
                let Coord { y, x } = tetromino.topleft;
                nc::mvwaddstr(
                    screen,
                    rowidx as i32 + y as i32,
                    (colidx as i32 + x as i32) * 2,
                    "██",
                );
            }
        }
    }
}
