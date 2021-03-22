use crate::core::{Coord, Direction, Grid, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH};
use crate::shape::{Rotation, Shape};
use crate::ui::Color;
use rand::prelude::SliceRandom;

#[cfg(test)]
use rstest_reuse::{self, *};

#[derive(Clone)]
pub struct Tetromino {
    pub grid: Grid,
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
        let tetrovec = self.shape.to_4x4(self.current_rotation);
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
        let tetrovec = self.shape.to_4x4(self.current_rotation);
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
        let tetrovec = self.shape.to_4x4(potential_rotation);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Block, Game};
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

        assert_eq!(tetromino.move_sideways(Direction::Right), Err("Collision."));
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

        assert_eq!(tetromino.move_sideways(Direction::Left), Err("Collision."));
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
