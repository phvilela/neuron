use std::{
    cmp::{max, min}, f32, fmt::Display, thread::{sleep}, time::Duration,
};

use rand::random_range;

const SCREEN_WIDTH: u16 = 20;
const SCREEN_HEIGHT: u16 = 20;

#[derive(Clone, Copy)]
struct Vector {
    x: f32,
    y: f32,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Vector: x = {} , y = {}", self.x, self.y);
    }
}

impl Vector {
    pub fn size(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }

    pub fn add(&self, v: Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    pub fn mul(self, n: f32) -> Vector {
        Vector {
            x: self.x * n,
            y: self.y * n,
        }
    }

    pub fn dot(self, v: Vector) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn flip_x(&mut self) {
        self.x = -self.x;
    }
    pub fn flip_y(&mut self) {
        self.y = -self.y;
    }
    const ZERO: Vector = Vector { x: 0.0, y: 0.0 };
}

struct Game {
    ball: Vector,
    ball_direction: Vector,
    ball_speed: f32,
    player_a: i16,
    player_b: i16,
}

impl Game {
    const NEW: Game = Game {
        ball: Vector {x : SCREEN_WIDTH as f32 / 2.0 , y : SCREEN_HEIGHT as f32 / 2.0},
        ball_direction: Vector::ZERO,
        ball_speed: 0.0,
        player_a: 0,
        player_b: 0,
    };
    const DEFAULT_BALL_SPEED: f32 = 1.0;

    fn start(&mut self) {
        let rand_rad: f32 = random_range(0.0..=0.7853983);
        self.ball_direction = Vector {
            x: rand_rad.cos(),
            y: rand_rad.sin(),
        };
        self.ball_speed = Game::DEFAULT_BALL_SPEED;

        loop {
            self.update();
        }
    }

    fn update(&mut self) {
        self.ball = self.ball.add(self.ball_direction.mul(self.ball_speed));
        self.check_colision();
        sleep(Duration::from_millis(10));
        self.render();
    }

    fn render(&self) {
        let x = self.ball.x;
        let y = self.ball.y;
        let ball = y as usize * SCREEN_WIDTH as usize + x as usize;
        let mut i: usize = 0;
        println!("frame");
        loop {
            if i % SCREEN_WIDTH as usize == 0 {
                println!("")
            }
            if i == ball {
                print!("1")
            } else {
                print!(" ")
            }
            i = i + 1;
            if i > SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize {
                break;
            }
        }
    }

    fn check_colision(&mut self) {
        if self.ball.y < 0.0 || self.ball.y > SCREEN_HEIGHT as f32 {
            self.ball_direction.flip_y();
            self.ball.y =  min(max(SCREEN_HEIGHT, self.ball.y as u16),0) as f32;
        }
        if self.ball.x < 0.0 || self.ball.x > SCREEN_WIDTH as f32 {
            self.ball_direction.flip_x();
            self.ball.x = min(max(SCREEN_HEIGHT, self.ball.x as u16),0) as f32
        }
    }
}

fn main() {
    let mut game: Game = Game::NEW;
    game.start()
}
