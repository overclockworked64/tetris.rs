use crate::core::{Coord, Tetromino};
use ncurses as nc;

pub fn curses_init() {
    nc::setlocale(nc::LcCategory::all, "");
    nc::initscr();
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

pub fn draw_tetromino(tetromino: &Tetromino) {
    let tetrovec = tetromino.shape.to_vec(tetromino.current_rotation.unwrap());
    for (rowidx, row) in tetrovec.iter().enumerate() {
        for (colidx, col) in row.iter().enumerate() {
            if tetrovec[rowidx][colidx] != 0 {
                let Coord { y, x } = tetromino.topleft;
                nc::mvaddstr(
                    rowidx as i32 + y as i32,
                    (colidx as i32 + x as i32) * 2,
                    "██",
                );
            }
        }
    }
}
