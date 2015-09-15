extern crate rand;

use self::rand::Rng;

#[derive(PartialEq,Copy,Clone)]
pub enum Direction { Up, Down, Left, Right }

pub enum GameStatus { Win, Running, Dead }

#[derive(PartialEq,Copy,Clone)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

pub struct Game {
    bounds: Vector,
    invaders: Vec<Invader>,
    player: Player,
    bullets: Vec<Bullet>,
}

impl Game {

    pub fn new (bounds: Vector) -> Game {

        let mut invaders = vec!();
        //for i in range(0i32, bounds.x / 3) {
        for i in 0i32..(bounds.x / 3) {
            invaders.push(Invader::new( Vector { x: 2 * i, y: 0 }));
        }

        Game {
            bounds: bounds,
            invaders: invaders,
            player: Player::new(Vector { x: bounds.x / 2, y: bounds.y - 1 }),
            bullets: vec!(),
        }
    }

    pub fn tick (&mut self) -> GameStatus {

        for invader in self.invaders.iter_mut() {
            match invader.give_chance_to_fire() {
                Some(bullet) => self.bullets.push(bullet),
                None => (),
            };
            invader.tick(self.bounds);
        }

        for bullet in self.bullets.iter_mut() {
            bullet.tick(self.bounds);

            self.invaders.retain(|i| !bullet.check_collision(i.position));

            if bullet.check_collision(self.player.position) {
                return GameStatus::Dead;
            }
        }

        if self.invaders.is_empty() {
            GameStatus::Win
        } else {
            GameStatus::Running
        }

    }

    pub fn shift (&mut self, dir: Direction) {
        self.player.shift(dir, self.bounds);
    }

    pub fn fire (&mut self) {
        self.bullets.push(self.player.fire());
    }

    pub fn get_player_vector (&self) -> &Vector {
        &self.player.position
    }

    pub fn get_invader_vectors (&self) -> Vec<Vector> {
        self.invaders.iter().map(|i| i.position).collect()
    }

    pub fn get_bullet_vectors (&self) -> Vec<Vector> {
        self.bullets.iter().map(|b| b.position).collect()
    }

}

struct Invader {
    position: Vector,
    direction: Direction,
}

impl Invader {

    fn new (pos: Vector) -> Invader {
        Invader {
            position: pos,
            direction: Direction::Left,
        }
    }

    fn tick (&mut self, bounds: Vector) {
        let x = &mut self.position.x;
        self.direction = match self.direction {
            Direction::Left if *x < 0 => Direction::Right,
            Direction::Right if *x == bounds.x => Direction::Left,
            _ => self.direction.clone()
        };
        match self.direction {
            Direction::Left => *x = *x - 1,
            Direction::Right => *x = *x + 1,
            _ => (),
        };
    }

    fn give_chance_to_fire (&self) -> Option<Bullet> {
        let mut rng = rand::thread_rng();
        let temp: f32 = rng.gen_range(0.0, 1.0);
        if temp > 0.996 {
            Some(Bullet::new(Vector { x: self.position.x, y: self.position.y + 1 }, Direction::Down))
        } else {
            None
        }
    }

}

struct Player {
    position: Vector,
}

impl Player {

    fn new (pos: Vector) -> Player {
        Player { position: pos }
    }

    fn shift (&mut self, dir: Direction, bounds: Vector) {
        let x = &mut self.position.x;
        match dir {
            Direction::Left if *x > 0 => *x = *x - 1,
            Direction::Right if *x < bounds.x - 1 => *x = *x + 1,
            _ => (),
        }
    }

    fn fire (&self) -> Bullet {
        Bullet::new(Vector { x: self.position.x, y: self.position.y - 1 }, Direction::Up)
    }

}

struct Bullet {
    position: Vector,
    direction: Direction,
}

impl Bullet {

    fn new (pos: Vector, dir: Direction) -> Bullet {
        Bullet {
            position: pos,
            direction: dir,
        }
    }

    fn tick (&mut self, bounds: Vector) {
        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            _ => (),
        };
        if self.position.y < 0 || self.position.y == bounds.y {
            drop(self);
        }
    }

    fn check_collision (&self, other: Vector) -> bool {
        self.position == other
    }

}
