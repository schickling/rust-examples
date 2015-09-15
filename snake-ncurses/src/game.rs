extern crate rand;

use self::rand::Rng;

pub enum GameError { Wall, Suicide }

#[derive(PartialEq,Clone,Copy)]
pub enum Direction { Up, Down, Left, Right }

#[derive(PartialEq,Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    fn next (&self, dir: Direction) -> Vector {
        let (dx, dy) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        Vector {
            x: self.x + dx,
            y: self.y + dy,
        }
    }



    fn random (bounds: Vector) -> Vector {
        let mut rng = rand::thread_rng();
        Vector {
            x: rng.gen_range::<>(0, bounds.x),
            y: rng.gen_range::<>(0, bounds.y),
        }
    }
}


impl Clone for Vector {

    fn clone(&self) -> Self {
        Vector { x : self.x, y : self.y}
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
    }

}

pub struct Board {
    bounds: Vector,
    snake: Snake,
    bullet: Vector,
}

impl Board {

    pub fn new (bounds: Vector) -> Board {
        Board {
            bounds: bounds,
            snake: Snake::new(Vector { x: bounds.x / 2, y: bounds.y / 2 }),
            bullet: Vector::random(bounds),
        }
    }

    pub fn set_direction (&mut self, dir: Direction) {
        self.snake.direction = dir;
    }

    pub fn tick (&mut self) -> Result<(), GameError> {

        self.snake.step();

        if self.snake.eats_bullet(self.bullet) {
            self.snake.grow();
            self.bullet = Vector::random(self.bounds);
        }

        if self.snake.hits_wall(self.bounds) {
            Err(GameError::Wall)
        } else if self.snake.hits_itself() {
            Err(GameError::Suicide)
        } else {
            Ok(())
        }

    }

    pub fn get_snake_vectors (&self) -> &[Vector] {
        let ref v = self.snake.segments;
        &v[..]
    }

    pub fn get_bullet_vector (&self) -> &Vector {
        &self.bullet
    }

}

struct Snake {
    segments: Vec<Vector>,
    direction: Direction,
    popped_segment: Vector,
}

impl Snake {

    fn new (pos: Vector) -> Snake {
        Snake {
            segments: vec!(pos),
            direction: Direction::Up,
            popped_segment: Vector { x: 0, y: 0 }
        }
    }

    fn step (&mut self) {
        let new_head = self.segments[0].next(self.direction);
        self.segments.insert(0, new_head);
        self.popped_segment = self.segments.pop().unwrap();
    }

    fn hits_wall (&self, bounds: Vector) -> bool {
        let head = self.segments[0];
        head.x < 0 || head.x == bounds.x || head.y < 0 || head.y == bounds.y
    }

    fn hits_itself (&self) -> bool {
        self.segments.iter().skip(1).any(|s| *s == self.segments[0] )
    }

    fn grow (&mut self) {
        self.segments.push(self.popped_segment);
    }

    fn eats_bullet (&self, bullet: Vector) -> bool {
        self.segments[0] == bullet
    }

}
