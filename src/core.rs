use crate::ui::{curses_teardown, Color};
use rand::{
    distributions::{Distribution, Standard},
    prelude::SliceRandom,
    Rng,
};

#[cfg(test)]
use rstest_reuse::{self, *};

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

    fn create_grid() -> Grid {
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
    fn new(value: u8, color: Option<Color>) -> Block {
        Block { value, color }
    }
}

pub struct Tetromino {
    grid: Grid,
    pub shape: Shape,
    pub color: Color,
    pub topleft: Coord,
    pub current_rotation: Rotation,
}

impl Tetromino {
    pub fn new(grid: Grid) -> Tetromino {
        let shape = rand::random::<Shape>();
        let current_rotation = shape
            .get_possible_rotations()
            .choose(&mut rand::thread_rng())
            .copied()
            .unwrap();
        let color = shape.get_color();
        Tetromino {
            grid,
            shape,
            color,
            current_rotation,
            topleft: Coord {
                y: 0,
                x: PLAYGROUND_WIDTH / 2 - 1,
            },
        }
    }

    pub fn move_sideways(&mut self, direction: Direction) -> Result<(), &'static str> {
        let tetrovec = self.shape.to_vec(self.current_rotation);
        for (rowidx, row) in tetrovec.into_iter().enumerate() {
            for (colidx, column) in row.into_iter().enumerate() {
                if column != 0 {
                    let Coord { y, x } = self.topleft;
                    let next_step = colidx as i32 + x + direction as i32;
                    if !(0..PLAYGROUND_WIDTH).contains(&next_step) {
                        return Err("Out of bounds.");
                    }
                    if self.grid[rowidx + y as usize][next_step as usize].value != 0 {
                        return Err("Collision.");
                    }
                }
            }
        }
        self.topleft.x += direction as i32;

        Ok(())
    }

    pub fn move_all_the_way_down(&mut self) {
        while let Ok(()) = self.move_down() {
            continue;
        }
    }

    pub fn move_down(&mut self) -> Result<(), &'static str> {
        let tetrovec = self.shape.to_vec(self.current_rotation);
        for (rowidx, row) in tetrovec.into_iter().enumerate() {
            for (colidx, column) in row.into_iter().enumerate() {
                if column != 0 {
                    let Coord { y, x } = self.topleft;
                    let next_step = Coord {
                        y: rowidx as i32 + y + 1,
                        x: colidx as i32 + x,
                    };
                    if next_step.y >= PLAYGROUND_HEIGHT {
                        return Err("Out of bounds.");
                    }
                    if self.grid[next_step.y as usize][next_step.x as usize].value != 0 {
                        return Err("Collision.");
                    }
                }
            }
        }
        self.topleft.y += 1;

        Ok(())
    }

    pub fn rotate(&mut self, direction: Direction) -> Result<(), &'static str> {
        let rotations = self.shape.get_possible_rotations();
        let current_index = rotations
            .iter()
            .position(|x| *x == self.current_rotation)
            .unwrap();
        let next_index = i32::checked_rem_euclid(
            current_index as i32 + direction as i32,
            rotations.len() as i32,
        );
        let potential_rotation = rotations[next_index.unwrap() as usize];
        let tetrovec = self.shape.to_vec(potential_rotation);
        for (rowidx, row) in tetrovec.into_iter().enumerate() {
            for (colidx, column) in row.into_iter().enumerate() {
                if column != 0 {
                    let Coord { y, x } = self.topleft;
                    let next_step = Coord {
                        y: rowidx as i32 + y,
                        x: colidx as i32 + x,
                    };
                    if !(0..PLAYGROUND_WIDTH).contains(&next_step.x) {
                        return Err("Out of bounds.");
                    }
                    if next_step.y >= PLAYGROUND_HEIGHT {
                        return Err("Out of bounds.");
                    }
                    if self.grid[next_step.y as usize][next_step.x as usize].value != 0 {
                        return Err("Collision.");
                    }
                }
            }
        }
        self.current_rotation = potential_rotation;
        Ok(())
    }
}

#[derive(PartialEq)]
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
    fn get_color(&self) -> Color {
        match self {
            Shape::O => Color::Blue,
            Shape::I => Color::Yellow,
            Shape::S => Color::Cyan,
            Shape::Z => Color::White,
            Shape::J => Color::Magenta,
            Shape::L => Color::Red,
            Shape::T => Color::Green,
        }
    }

    fn get_possible_rotations(&self) -> Vec<Rotation> {
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

    pub fn to_vec(&self, rotation: Rotation) -> ShapeVec {
        (0..16)
            .map(|i| (rotation >> (15 - i)) & 1)
            .collect::<Vec<Rotation>>()
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

type Rotation = u16;
type ShapeVec = Vec<Vec<Rotation>>;

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
mod game_tests {
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

#[cfg(test)]
mod tetromino_tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn tetromino() -> Tetromino {
        let grid = Game::create_grid();
        let mut tetromino = Tetromino::new(grid);
        tetromino.topleft = Coord { y: 5, x: 5 };
        tetromino
    }

    #[template]
    #[rstest(
        shape,
        case(Shape::O),
        case(Shape::I),
        case(Shape::S),
        case(Shape::Z),
        case(Shape::J),
        case(Shape::L),
        case(Shape::T)
    )]
    fn all_shapes(shape: Shape) {}

    #[apply(all_shapes)]
    fn move_sideways_left_ok(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        assert_eq!(tetromino.move_sideways(Direction::Left), Ok(()));
    }

    #[apply(all_shapes)]
    fn move_sideways_right_ok(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        assert_eq!(tetromino.move_sideways(Direction::Right), Ok(()));
    }

    #[apply(all_shapes)]
    fn move_sideways_left_out_of_bounds(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        tetromino.topleft.x = -3;
        assert_eq!(
            tetromino.move_sideways(Direction::Left),
            Err("Out of bounds.")
        );
    }

    #[apply(all_shapes)]
    fn move_sideways_right_out_of_bounds(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        tetromino.topleft.x = PLAYGROUND_WIDTH;
        assert_eq!(
            tetromino.move_sideways(Direction::Right),
            Err("Out of bounds.")
        );
    }

    #[apply(all_shapes)]
    fn move_sideways_left_collision(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;

        for row in 0..PLAYGROUND_HEIGHT {
            for column in 0..PLAYGROUND_WIDTH {
                if column > PLAYGROUND_WIDTH - 5 {
                    tetromino.grid[row as usize][column as usize] = Block::new(1, None);
                }
            }
        }

        assert_eq!(
            tetromino.move_sideways(Direction::Right),
            Err("Collision.")
        );
    }


    #[apply(all_shapes)]
    fn move_sideways_right_collision(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;

        for row in 0..PLAYGROUND_HEIGHT {
            for column in 0..PLAYGROUND_WIDTH {
                if column <= 6 {
                    tetromino.grid[row as usize][column as usize] = Block::new(1, None);
                }
            }
        }

        assert_eq!(
            tetromino.move_sideways(Direction::Left),
            Err("Collision.")
        );
    }

    #[apply(all_shapes)]
    fn move_down_no_obstacles(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        tetromino.topleft.y = 0;
        for _ in 0..5 {
            assert_eq!(tetromino.move_down(), Ok(()));
        }
    }

    #[apply(all_shapes)]
    fn move_down_out_of_bounds(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        tetromino.topleft.y = PLAYGROUND_HEIGHT;
        assert_eq!(tetromino.move_down(), Err("Out of bounds."));
    }

    #[apply(all_shapes)]
    fn move_down_collision(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        for i in 6..9 {
            tetromino.grid[i] = [Block::new(1, None); PLAYGROUND_WIDTH as usize];
        }
        assert_eq!(tetromino.move_down(), Err("Collision."));
    }

    #[apply(all_shapes)]
    fn rotate_left_ok(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        let possible_rotations = tetromino.shape.get_possible_rotations();
        tetromino.current_rotation = *possible_rotations.last().unwrap();
        for rotation_number in (0..possible_rotations.len() - 1).rev() {
            assert_eq!(tetromino.rotate(Direction::Left), Ok(()));
            assert_eq!(
                tetromino.current_rotation,
                possible_rotations[rotation_number]
            )
        }
    }

    #[apply(all_shapes)]
    fn rotate_right_ok(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        let possible_rotations = tetromino.shape.get_possible_rotations();
        tetromino.current_rotation = possible_rotations[0];

        for rotation_index in 1..possible_rotations.len() {
            assert_eq!(tetromino.rotate(Direction::Right), Ok(()));
            assert_eq!(
                tetromino.current_rotation,
                possible_rotations[rotation_index]
            )
        }
    }

    #[apply(all_shapes)]
    fn rotate_left_out_of_bounds(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        tetromino.topleft.x = -3;
        let possible_rotations = tetromino.shape.get_possible_rotations();

        for rotation in possible_rotations {
            tetromino.current_rotation = rotation;
            assert_eq!(tetromino.rotate(Direction::Left), Err("Out of bounds."));
            assert_eq!(tetromino.current_rotation, tetromino.current_rotation);
        }
    }

    #[apply(all_shapes)]
    fn rotate_right_out_of_bounds(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        tetromino.topleft.x = PLAYGROUND_WIDTH;
        let possible_rotations = tetromino.shape.get_possible_rotations();

        for rotation in possible_rotations {
            tetromino.current_rotation = rotation;
            assert_eq!(tetromino.rotate(Direction::Right), Err("Out of bounds."));
            assert_eq!(tetromino.current_rotation, tetromino.current_rotation);
        }
    }

    #[apply(all_shapes)]
    fn rotate_collision_left(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        let possible_rotations = tetromino.shape.get_possible_rotations();

        for i in 6..9 {
            tetromino.grid[i] = [Block::new(1, None); PLAYGROUND_WIDTH as usize];
        }

        for rotation in possible_rotations {
            tetromino.current_rotation = rotation;
            assert_eq!(tetromino.rotate(Direction::Left), Err("Collision."));
            assert_eq!(tetromino.current_rotation, tetromino.current_rotation);
        }
    }

    #[apply(all_shapes)]
    fn rotate_collision_right(mut tetromino: Tetromino, shape: Shape) {
        tetromino.shape = shape;
        let possible_rotations = tetromino.shape.get_possible_rotations();

        for i in 6..9 {
            tetromino.grid[i] = [Block::new(1, None); PLAYGROUND_WIDTH as usize];
        }

        for rotation in possible_rotations {
            tetromino.current_rotation = rotation;
            assert_eq!(tetromino.rotate(Direction::Right), Err("Collision."));
            assert_eq!(tetromino.current_rotation, tetromino.current_rotation);
        }
    }
}

#[cfg(test)]
mod shape_tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        shape,
        color,
        case(Shape::O, Color::Blue),
        case(Shape::I, Color::Yellow),
        case(Shape::S, Color::Cyan),
        case(Shape::Z, Color::White),
        case(Shape::J, Color::Magenta),
        case(Shape::L, Color::Red),
        case(Shape::T, Color::Green)
    )]
    fn get_color(shape: Shape, color: Color) {
        assert_eq!(shape.get_color(), color);
    }

    #[rstest(
        shape, rotations,
        case(Shape::O, vec![51]),
        case(Shape::I, vec![8738, 240]),
        case(Shape::S, vec![54, 561]),
        case(Shape::Z, vec![99, 306]),
        case(Shape::J, vec![275, 71, 802, 113]),
        case(Shape::L, vec![547, 116, 785, 23]),
        case(Shape::T, vec![114, 305, 39, 562]),
    )]
    fn get_possible_rotations(shape: Shape, rotations: Vec<Rotation>) {
        assert_eq!(shape.get_possible_rotations(), rotations);
    }

    #[rstest(
        shape, expected,
        case(Shape::O, vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 1],
            ]
        ]),
        case(Shape::I, vec![
            vec![
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
            ],
        ]),
        case(Shape::S, vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 1, 1, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 0, 1],
            ],
        ]),
        case(Shape::Z, vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 1],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 0],
            ],
        ]),
        case(Shape::J, vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 1],
                vec![0, 0, 1, 1],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 1, 1],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 1],
                vec![0, 0, 0, 1],
            ],
        ]),
        case(Shape::L, vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 1],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 1],
                vec![0, 1, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 1],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 1, 1, 1],
            ],
        ]),
        case(Shape::T, vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 1],
                vec![0, 0, 1, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 1, 1],
                vec![0, 0, 0, 1],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 1, 1, 1],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 0],
            ],
        ]),
    )]
    fn to_vec(shape: Shape, expected: Vec<ShapeVec>) {
        let possible_rotations = shape.get_possible_rotations();
        for (exp, possible_rotation) in expected.iter().zip(possible_rotations) {
            assert_eq!(shape.to_vec(possible_rotation), *exp);
        }
    }
}
