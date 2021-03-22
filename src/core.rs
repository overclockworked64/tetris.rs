use crate::tetromino::Tetromino;
use crate::ui::{curses_teardown, Color};

pub const PLAYGROUND_WIDTH: i32 = 10;
pub const PLAYGROUND_HEIGHT: i32 = 16;

pub struct Game {
    pub grid: Grid,
    pub current_tetromino: Tetromino,
    pub next_tetromino: Tetromino,
    pub paused: bool,
    pub score: u64,
    counter: u8,
}

impl Game {
    pub fn new() -> Game {
        let grid = Game::create_grid();
        Game {
            current_tetromino: Tetromino::new(grid),
            next_tetromino: Tetromino::new(grid),
            grid,
            score: 0,
            counter: 0,
            paused: false,
        }
    }

    pub fn create_grid() -> Grid {
        [Game::create_empty_row(); PLAYGROUND_HEIGHT as usize]
    }

    fn create_empty_row() -> [Block; PLAYGROUND_WIDTH as usize] {
        [Block::new(0, None); PLAYGROUND_WIDTH as usize]
    }

    pub fn clear_rows(&mut self) {
        for i in 0..self.grid.len() {
            if self.grid[i].iter().fold(0, |acc, x| acc + x.value) as i32 == PLAYGROUND_WIDTH {
                let row = Game::create_empty_row();
                self.grid[i] = row;
                self.grid[..i + 1].rotate_right(1);
                self.current_tetromino.grid = self.grid;
                self.score += PLAYGROUND_WIDTH as u64;
            }
        }
    }

    pub fn handle_falling(&mut self) {
        self.counter += 1;
        if self.counter == 5 {
            if self.current_tetromino.move_down().is_err() {
                if self.land_tetromino().is_err() {
                    curses_teardown();
                    std::process::exit(0);
                } else {
                    self.current_tetromino = self.next_tetromino.clone();
                    self.current_tetromino.grid = self.grid;
                    self.next_tetromino = Tetromino::new(self.grid);
                }
            }
            self.counter = 0;
        }
    }

    fn land_tetromino(&mut self) -> Result<(), &'static str> {
        if self.current_tetromino.topleft.y <= 0 {
            return Err("Game over.");
        }

        let current_rotation = self.current_tetromino.current_rotation;
        let tetrovec = self.current_tetromino.shape.to_4x4(current_rotation);

        for (rowidx, row) in tetrovec.into_iter().enumerate() {
            for (colidx, column) in row.into_iter().enumerate() {
                if column != 0 {
                    let Coord { y, x } = self.current_tetromino.topleft;
                    self.grid[rowidx + y as usize][(colidx as i32 + x as i32) as usize] = Block {
                        value: column as u8,
                        color: Some(self.current_tetromino.color),
                    }
                }
            }
        }
        Ok(())
    }
}

pub type Grid = [[Block; PLAYGROUND_WIDTH as usize]; PLAYGROUND_HEIGHT as usize];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block {
    pub value: u8,
    pub color: Option<Color>,
}

impl Block {
    pub fn new(value: u8, color: Option<Color>) -> Block {
        Block { value, color }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left = -1,
    Right = 1,
}
#[derive(Clone, Copy)]
pub struct Coord {
    pub y: i32,
    pub x: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grid() {
        let grid = Game::create_grid();
        assert_eq!(grid.len(), PLAYGROUND_HEIGHT as usize);
        for i in 0..PLAYGROUND_HEIGHT {
            assert_eq!(grid[i as usize].len(), PLAYGROUND_WIDTH as usize);
        }
    }

    #[test]
    fn create_empty_row() {
        let row = Game::create_empty_row();
        assert_eq!(row.len(), PLAYGROUND_WIDTH as usize);
        for i in 0..PLAYGROUND_WIDTH {
            assert_eq!(
                row[i as usize],
                Block {
                    value: 0,
                    color: None
                }
            );
        }
    }

    #[test]
    fn clear_rows() {
        let mut game = Game::new();
        for row in 0..PLAYGROUND_HEIGHT {
            for column in 0..PLAYGROUND_WIDTH {
                if row > ((PLAYGROUND_HEIGHT / 4) * 3) - 1 {
                    game.grid[row as usize][column as usize] = Block::new(1, None);
                }
            }
        }

        let row_above_last_quarter = (((PLAYGROUND_HEIGHT / 4) * 3) - 1) as usize;
        for column in 4..7 {
            game.grid[row_above_last_quarter][column] = Block::new(1, None);
        }

        game.clear_rows();

        for column in 4..7 {
            assert_eq!(
                game.grid[PLAYGROUND_HEIGHT as usize - 1][column],
                Block::new(1, None)
            );
        }
    }
}
