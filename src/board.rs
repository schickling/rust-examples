use std::rand::Rng;
use std::rand;

#[deriving(PartialEq)]
pub enum Direction { Up, Down, Left, Right }

pub enum Status { Running, Error }

#[deriving(PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn step (&mut self, dir: &Direction) -> Position {
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

    pub fn tick (&mut self) -> Status {

        //for p in self.snake.segments.iter_mut() {
            //p.step(&self.snake.direction);
        //}

        let new_head = self.snake.segments[0].step(&self.snake.direction);
        self.snake.segments.insert(0, new_head);


        {
            let snake_head = self.snake.head();
            if (snake_head.x < 0 || snake_head.x == self.width ||
                snake_head.y < 0 || snake_head.y == self.height) {
                return Error;
            }
        }

        if self.snake.segments[0] == self.bullet {
            self.bullet = random_position(&self.width, &self.height);
        } else {
            self.snake.segments.pop();
        }

        Running

    }

    pub fn get_snake_positions (&self) -> &Vec<Position> {
        &self.snake.segments
    }

    pub fn get_bullet_position (&self) -> &Position {
        &self.bullet
    }

}

struct Snake {
    length: i32,
    segments: Vec<Position>,
    direction: Direction,
}

impl Snake {
    fn new (pos: Position) -> Snake {
        Snake {
            length: 1,
            segments: vec!(pos),
            direction: Up,
        }
    }

    fn head (&self) -> &Position {
        &self.segments[0]
    }
}

fn random_position (max_x: &i32, max_y: &i32) -> Position {
    let mut rng = rand::task_rng();
    Position {
        x: rng.gen_range::<i32>(0, *max_x),
        y: rng.gen_range::<i32>(0, *max_y),
    }
}
