use crate::ui::Color;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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
    pub fn get_color(&self) -> Color {
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

    pub fn get_possible_rotations(&self) -> Vec<Rotation> {
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

pub type Rotation = u16;
type ShapeVec = Vec<Vec<Rotation>>;

#[cfg(test)]
mod tests {
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
