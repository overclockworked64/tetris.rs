use crate::tetromino::Tetromino;
use crate::ui::{curses_teardown, Color};

pub const PLAYGROUND_WIDTH: i32 = 10;
pub const PLAYGROUND_HEIGHT: i32 = 16;

pub struct Game {
    pub grid: Grid,
    pub tetromino: Tetromino,
    pub paused: bool,
    pub score: u64,
    counter: u8,
}

impl Game {
    pub fn new() -> Game {
        let grid = Game::create_grid();
        Game {
            tetromino: Tetromino::new(grid),
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
                self.tetromino.grid = self.grid;
                self.score += PLAYGROUND_WIDTH as u64;
            }
        }
    }

    pub fn handle_falling(&mut self) {
        self.counter += 1;
        if self.counter == 5 {
            if self.tetromino.move_down().is_err() {
                if self.land_tetromino().is_err() {
                    curses_teardown();
                    std::process::exit(0);
                } else {
                    self.tetromino = Tetromino::new(self.grid);
                }
            }
            self.counter = 0;
        }
    }

    fn land_tetromino(&mut self) -> Result<(), &'static str> {
        if self.tetromino.topleft.y <= 0 {
            return Err("Game over.");
        }

        let current_rotation = self.tetromino.current_rotation;
        let tetrovec = self.tetromino.shape.to_vec(current_rotation);

        for (rowidx, row) in tetrovec.into_iter().enumerate() {
            for (colidx, column) in row.into_iter().enumerate() {
                if column != 0 {
                    let Coord { y, x } = self.tetromino.topleft;
                    self.grid[rowidx + y as usize][(colidx as i32 + x as i32) as usize] = Block {
                        value: column as u8,
                        color: Some(self.tetromino.color),
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
}
