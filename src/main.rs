use ncurses as nc;

mod core;
mod ui;

fn main() {
    ui::curses_init();
    let mut tetromino = core::Tetromino::new();
    tetromino.pick_random_rotation();
    loop {
        nc::clear();
        ui::draw_tetromino(&tetromino);
        nc::refresh();
        tetromino.move_down();
    }
    ui::curses_teardown();
}
