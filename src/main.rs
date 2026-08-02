use std::{
    cmp::{max, min},
    f32,
    fmt::Display,
    io::{self, Write},
    sync::OnceLock,
    thread::sleep,
    time::Duration,
};

use rand::random_range;
use terminal_size::terminal_size;

static SCREEN_WIDTH: OnceLock<u16> = OnceLock::new();
static SCREEN_HEIGHT: OnceLock<u16> = OnceLock::new();

fn set_size() {
    if let Some((w, h)) = terminal_size() {
        let _ = SCREEN_WIDTH.set(w.0);
        let _ = SCREEN_HEIGHT.set(h.0 - 3);
    };
}

// Funções auxiliares opcionais para facilitar o acesso de qualquer lugar
fn screen_width() -> u16 {
    // Se não foi setado por algum motivo, define um padrão ou dá panic
    *SCREEN_WIDTH
        .get()
        .expect("SCREEN_WIDTH não foi inicializado!")
}

fn screen_height() -> u16 {
    *SCREEN_HEIGHT
        .get()
        .expect("SCREEN_HEIGHT não foi inicializado!")
}

fn clear_screen() {
    // \x1B[2J limpa a tela inteira
    // \x1B[1;1H move o cursor de volta para o canto superior esquerdo (linha 1, coluna 1)
    print!("\x1B[2J\x1B[1;1H");

    // É importante dar o flush para garantir que o comando seja executado imediatamente
    io::stdout().flush().unwrap();
}

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
    fn new() -> Game {
        Game {
            ball: Vector {
                x: screen_width() as f32 / 2.0,
                y: screen_width() as f32 / 2.0,
            },
            ball_direction: Vector::ZERO,
            ball_speed: 0.0,
            player_a: 0,
            player_b: 0,
        }
    }
    const DEFAULT_BALL_SPEED: f32 = 0.8;

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
        sleep(Duration::from_millis(30));
        self.render();
    }

    fn render(&self) {
        clear_screen();
        let x = self.ball.x;
        let y = self.ball.y;
        let ball = y as usize * screen_width() as usize + x as usize;
        let mut i: usize = 0;
        println!("Ball: {}\t Direction {}", self.ball, self.ball_direction);
        for _ in 0..screen_width() {
            print!("-")
        }

        loop {
            if i % screen_width() as usize == 0 {
                println!("")
            }
            if i == ball {
                print!("o")
            } else {
                print!(" ")
            }
            i = i + 1;
            if i > screen_width() as usize * screen_height() as usize {
                break;
            }
        }
    }

    fn check_colision(&mut self) {
        if self.ball.y < 0.0 || self.ball.y > screen_height() as f32 {
            self.ball_direction.flip_y();
            self.ball.y = max(min(screen_height(), self.ball.y as u16), 0) as f32;
        }
        if self.ball.x < 0.0 || self.ball.x > screen_width() as f32 {
            self.ball_direction.flip_x();
            self.ball.x = max(min(screen_width(), self.ball.x as u16), 0) as f32
        }
    }
}

fn main() {
    set_size();
    let mut game: Game = Game::new();
    game.start()
}
