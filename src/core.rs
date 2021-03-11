use rand::{
    distributions::{Distribution, Standard},
    prelude::SliceRandom,
    Rng,
};

pub const PLAYGROUND_WIDTH: i32 = 10;
pub const PLAYGROUND_HEIGHT: i32 = 16;

pub type Grid = Vec<Vec<u16>>;

pub struct Game {
    pub grid: Grid,
    pub tetromino: Tetromino,
    counter: u8,
}

impl Game {
    pub fn new() -> Game {
        let mut grid = vec![];
        for _ in 0..PLAYGROUND_HEIGHT {
            let mut row = vec![];
            for _ in 0..PLAYGROUND_WIDTH {
                row.push(0);
            }
            grid.push(row);
        }
        Game {
            grid,
            tetromino: Tetromino::new(),
            counter: 0,
        }
    }

    pub fn handle_falling(&mut self) {
        self.counter += 1;
        if self.counter == 5 {
            if let Err(_) = self.tetromino.move_down(&self.grid) {
                if let Err(_) = self.land_tetromino() {
                    std::process::exit(0);
                } else {
                    self.tetromino = Tetromino::new();
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
        for (rowidx, row) in tetrovec.iter().enumerate() {
            for (colidx, _) in row.iter().enumerate() {
                if tetrovec[rowidx][colidx] != 0 {
                    let Coord { y, x } = self.tetromino.topleft;
                    self.grid[rowidx + y as usize][(colidx as i32 + x as i32) as usize] =
                        tetrovec[rowidx][colidx]
                }
            }
        }
        Ok(())
    }
}

pub struct Tetromino {
    pub shape: Shape,
    pub current_rotation: u16,
    pub topleft: Coord,
}

impl Tetromino {
    pub fn new() -> Tetromino {
        let shape = rand::random::<Shape>();
        let current_rotation = shape
            .get_possible_rotations()
            .choose(&mut rand::thread_rng())
            .copied()
            .unwrap();
        Tetromino {
            shape,
            current_rotation,
            topleft: Coord {
                y: 0,
                x: PLAYGROUND_WIDTH / 2 - 1,
            },
        }
    }

    pub fn move_sideways(&mut self, grid: &Grid, direction: Direction) -> Result<(), &'static str> {
        let tetrovec = self.shape.to_vec(self.current_rotation);
        for (rowidx, row) in tetrovec.iter().enumerate() {
            for (colidx, _) in row.iter().enumerate() {
                if tetrovec[rowidx][colidx] != 0 {
                    let Coord { y, x } = self.topleft;
                    if !(0..PLAYGROUND_WIDTH).contains(&(colidx as i32 + x + direction as i32)) {
                        return Err("Out of bounds.");
                    }
                    if grid[rowidx + y as usize]
                        [(colidx as i32 + x as i32 + direction as i32) as usize]
                        != 0
                    {
                        return Err("Collision.");
                    }
                }
            }
        }
        self.topleft.x += direction as i32;
        Ok(())
    }

    pub fn move_down(&mut self, grid: &Grid) -> Result<(), &'static str> {
        let tetrovec = self.shape.to_vec(self.current_rotation);
        for (rowidx, row) in tetrovec.iter().enumerate() {
            for (colidx, _) in row.iter().enumerate() {
                if tetrovec[rowidx][colidx] != 0 {
                    let Coord { y, x } = self.topleft;
                    if rowidx + y as usize + 1 >= PLAYGROUND_HEIGHT as usize {
                        return Err("Out of bounds.");
                    }
                    if grid[(rowidx as i32 + y + 1) as usize][(colidx as i32 + x) as usize] != 0 {
                        return Err("Collision.");
                    }
                }
            }
        }
        self.topleft.y += 1;
        Ok(())
    }

    pub fn rotate(&mut self, direction: Direction) {
        let rotations = self.shape.get_possible_rotations();
        let current_index = rotations
            .iter()
            .position(|x| *x == self.current_rotation)
            .unwrap();
        let next_index = i32::checked_rem_euclid(
            current_index as i32 + direction as i32,
            rotations.len() as i32,
        );
        self.current_rotation = rotations[next_index.unwrap() as usize];
    }
}

pub enum Shape {
    O,
    I,
    S,
    Z,
    J,
    L,
    T,
}

impl Shape {
    fn get_possible_rotations(&self) -> Vec<u16> {
        match self {
            Shape::O => vec![51],
            Shape::I => vec![8738, 240],
            Shape::S => vec![54, 561],
            Shape::Z => vec![99, 306],
            Shape::J => vec![275, 71, 802, 113],
            Shape::L => vec![547, 116, 785, 23],
            Shape::T => vec![114, 305, 39, 562],
        }
    }

    pub fn to_vec(&self, rotation: u16) -> ShapeVec {
        (0..16)
            .map(|i| (rotation >> 15 - i) & 1)
            .collect::<Vec<u16>>()
            .chunks(4)
            .map(|x| x.to_owned())
            .collect::<ShapeVec>()
    }
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..=6) {
            0 => Shape::O,
            1 => Shape::I,
            2 => Shape::S,
            3 => Shape::Z,
            4 => Shape::J,
            5 => Shape::L,
            _ => Shape::T,
        }
    }
}

type ShapeVec = Vec<Vec<u16>>;

#[derive(Clone, Copy)]
pub enum Direction {
    Left = -1,
    Right = 1,
}
pub struct Coord {
    pub y: i32,
    pub x: i32,
}
