use crate::core::{Coord, Grid, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH};
use crate::tetromino::Tetromino;
use ncurses as nc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const SCREEN_WIDTH: i32 = PLAYGROUND_WIDTH * 2;
const SCREEN_HEIGHT: i32 = PLAYGROUND_HEIGHT;

pub fn curses_init() {
    nc::setlocale(nc::LcCategory::all, "");
    nc::initscr();
    nc::nodelay(nc::stdscr(), true);
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nc::noecho();
    nc::keypad(nc::stdscr(), true);
    init_color_pairs();
}

pub fn curses_teardown() {
    nc::clear();
    nc::refresh();
    nc::doupdate();
    nc::endwin();
}

pub fn init_color_pairs() {
    nc::start_color();
    nc::init_color(nc::COLOR_YELLOW, 1000, 1000, 0);
    Color::iter().for_each(|color| {
        nc::init_pair(color as i16, color as i16, color as i16);
    });
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
    for (rowidx, row) in tetrovec.into_iter().enumerate() {
        for (colidx, column) in row.into_iter().enumerate() {
            if column != 0 {
                let Coord { y, x } = tetromino.topleft;
                nc::wattr_on(screen, nc::COLOR_PAIR(tetromino.color as i16));
                nc::mvwaddstr(
                    screen,
                    rowidx as i32 + y as i32,
                    (colidx as i32 + x as i32) * 2,
                    "██",
                );
                nc::wattroff(screen, nc::COLOR_PAIR(tetromino.color as i16));
            }
        }
    }
}

pub fn draw_landed_tetrominos(screen: nc::WINDOW, grid: &Grid) {
    for (rowidx, row) in grid.iter().enumerate() {
        for (colidx, block) in row.iter().enumerate() {
            if block.value != 0 {
                nc::wattr_on(screen, nc::COLOR_PAIR(block.color.unwrap() as i16));
                nc::mvwaddstr(screen, rowidx as i32, colidx as i32 * 2, "██");
                nc::wattroff(screen, nc::COLOR_PAIR(block.color.unwrap() as i16));
            }
        }
    }
}

pub fn draw_score(score: u64) {
    let y = (nc::LINES() - SCREEN_HEIGHT) / 2 + SCREEN_HEIGHT + 1;
    let x = (nc::COLS() - SCREEN_WIDTH) / 2 - 1;
    nc::mvwaddstr(nc::stdscr(), y, x, &format!("SCORE: {}", score));
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Color {
    Yellow = nc::COLOR_YELLOW as isize,
    Blue = nc::COLOR_BLUE as isize,
    Green = nc::COLOR_GREEN as isize,
    Red = nc::COLOR_RED as isize,
    Magenta = nc::COLOR_MAGENTA as isize,
    Cyan = nc::COLOR_CYAN as isize,
    White = nc::COLOR_WHITE as isize,
}
