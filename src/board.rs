use std::rand::Rng;
use std::rand;

pub enum GameError { Wall, Suicide }

#[deriving(PartialEq)]
pub enum Direction { Up, Down, Left, Right }

#[deriving(PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn next (&self, dir: &Direction) -> Position {
        let mut dx = 0;
        let mut dy = 0;

        match *dir {
            Up => dy -= 1,
            Down => dy += 1,
            Left => dx -= 1,
            Right => dx += 1,
        }

        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

pub struct Board {
    width: i32,
    height: i32,
    snake: Snake,
    bullet: Position,
}

impl Board {

    pub fn new (width: i32, height: i32) -> Board {
        let snake_pos = Position { x: width / 2, y: height / 2 };
        Board {
            width: width,
            height: height,
            snake: Snake::new(snake_pos),
            bullet: random_position(&width, &height)
        }
    }

    pub fn set_direction (&mut self, dir: Direction) {
        self.snake.direction = dir;
    }

    pub fn tick (&mut self) -> Result<(), GameError> {

        self.snake.step();

        if self.snake.eats_bullet(self.bullet) {
            self.snake.grow();
            self.bullet = random_position(&self.width, &self.height);
        }

        if self.snake.hits_wall(&self.width, &self.height) {
            Err(Wall)
        } else if self.snake.hits_itself() {
            Err(Suicide)
        } else {
            Ok(())
        }

    }

    pub fn get_snake_positions (&self) -> &Vec<Position> {
        &self.snake.segments
    }

    pub fn get_bullet_position (&self) -> &Position {
        &self.bullet
    }

}

struct Snake {
    segments: Vec<Position>,
    direction: Direction,
    popped_segment: Position,
}

impl Snake {

    fn new (pos: Position) -> Snake {
        Snake {
            segments: vec!(pos),
            direction: Up,
            popped_segment: Position { x: 0, y: 0 }
        }
    }

    fn step (&mut self) {
        let new_head = self.segments[0].next(&self.direction);
        self.segments.insert(0, new_head);
        self.popped_segment = self.segments.pop().unwrap();
    }

    fn hits_wall (&self, max_x: &i32, max_y: &i32) -> bool {
        let head = self.segments[0];
        head.x < 0 || head.x == *max_x || head.y < 0 || head.y == *max_y
    }

    fn hits_itself (&self) -> bool {
        self.segments.iter().skip(1).any(|s| *s == self.segments[0] )
    }

    fn grow (&mut self) {
        self.segments.push(self.popped_segment);
    }

    fn eats_bullet (&self, bullet: Position) -> bool {
        self.segments[0] == bullet
    }

}

fn random_position (max_x: &i32, max_y: &i32) -> Position {
    let mut rng = rand::task_rng();
    Position {
        x: rng.gen_range::<i32>(0, *max_x),
        y: rng.gen_range::<i32>(0, *max_y),
    }
}
