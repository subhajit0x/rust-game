use ggez::event::KeyCode;
use ggez::{graphics};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::config::{GRID_CELL_SIZE, GRID_SIZE};
use ggez::graphics::mint;
use ggez::mint::Point2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    x: i16,
    y: i16,
}

trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
    where
        T: std::ops::Add<Output=T> + std::ops::Rem<Output=T> + Clone,
{
    fn modulo(&self, n: T) -> T {
        // Because of our trait bounds, we can now apply these operators.
        (self.clone() % n.clone() + n.clone()) % n
    }
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    pub fn random(max_x: i16, max_y: i16) -> Self {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..max_x), rng.gen_range(0..max_y)).into()
    }

    pub fn new_from_move(pos: GridPosition, dir: Direction, speed: i16) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, (pos.y - speed).modulo(GRID_SIZE.1)),
            Direction::Down => GridPosition::new(pos.x, (pos.y + speed).modulo(GRID_SIZE.1)),
            Direction::Left => GridPosition::new((pos.x - speed).modulo(GRID_SIZE.0), pos.y),
            Direction::Right => GridPosition::new((pos.x + speed).modulo(GRID_SIZE.0), pos.y),
        }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<GridPosition> for Point2<f32> {
    fn from(pos: GridPosition) -> Self {
        let coords: [f32; 2] = [(pos.x * GRID_CELL_SIZE.0) as f32, (pos.y * GRID_CELL_SIZE.1) as f32];
        let point: Point2<f32> = coords.into();
        point
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

impl From<GridPosition> for (i16, i16) {
    fn from(pos: GridPosition) -> Self {
        (pos.x as i16, pos.y as i16)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) { // rand 0.8
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn rand() -> Direction {
        return rand::random();
    }
}

pub struct RectangleBorder {
    top_left_corner: GridPosition,
    bot_right_corner: GridPosition,
}

impl RectangleBorder {
    pub fn new(top_left_corner: GridPosition, bot_right_corner: GridPosition) -> Self {
        RectangleBorder { top_left_corner, bot_right_corner }
    }

    pub fn is_it_in(&self, it: GridPosition) -> bool { // :)
        return !(it.x < self.top_left_corner.x
            || it.x > self.bot_right_corner.x
            || it.y < self.top_left_corner.y
            || it.y > self.bot_right_corner.y);
    }
}
