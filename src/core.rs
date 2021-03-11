use rand::{
    distributions::{Distribution, Standard},
    prelude::SliceRandom,
    Rng,
};

pub const PLAYGROUND_WIDTH: i32 = 10;
pub const PLAYGROUND_HEIGHT: i32 = 16;

pub struct Game {
    pub tetromino: Tetromino,
    counter: u8,
}

impl Game {
    pub fn new() -> Game {
        let mut tetromino = Tetromino::new();
        tetromino.pick_random_rotation();
        Game {
            tetromino,
            counter: 0,
        }
    }

    pub fn handle_falling(&mut self) {
        self.counter += 1;
        if self.counter == 5 {
            self.tetromino.move_down();
            self.counter = 0;
        }
    }
}
pub struct Tetromino {
    pub shape: Shape,
    pub current_rotation: Option<u16>,
    pub topleft: Coord,
}

impl Tetromino {
    pub fn new() -> Tetromino {
        Tetromino {
            shape: rand::random::<Shape>(),
            current_rotation: None,
            topleft: Coord { y: 0, x: 0 },
        }
    }

    pub fn move_down(&mut self) {
        self.topleft.y += 1;
    }

    pub fn pick_random_rotation(&mut self) {
        self.current_rotation = self
            .shape
            .get_possible_rotations()
            .choose(&mut rand::thread_rng())
            .copied();
    }

    fn rotate(&mut self, direction: Direction) {
        let rotations = self.shape.get_possible_rotations();
        let current_index = rotations
            .iter()
            .position(|x| *x == self.current_rotation.unwrap())
            .unwrap();
        let next_index = i32::checked_rem_euclid(
            current_index as i32 + direction as i32,
            rotations.len() as i32,
        );
        self.current_rotation = Some(rotations[next_index.unwrap() as usize]);
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

enum Direction {
    Left = -1,
    Right = 1,
}
pub struct Coord {
    pub y: i32,
    pub x: i32,
}