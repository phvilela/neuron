use std::{
    cmp::{max, min},
    io::{Write, stdout},
    thread::sleep,
    time::Duration,
};

use rand::random_range;

use crate::vector::Vector;

const SLEEP_TIME: u64 = 10;

pub struct Game {
    ball: Vector,
    ball_direction: Vector,
    ball_speed: f32,
    player_a: i16,
    player_b: i16,
    screen_width: isize,
    screen_height: isize,
}

impl Game {
    pub fn new(w: isize, h: isize) -> Game {
        Game {
            ball: Vector {
                x: w as f32 / 2.0,
                y: h as f32 / 2.0,
            },
            ball_direction: Vector::ZERO,
            ball_speed: 0.0,
            player_a: 0,
            player_b: 0,
            screen_width: w,
            screen_height: h - 2,
        }
    }
    const DEFAULT_BALL_SPEED: f32 = 2.0;

    pub fn start(&mut self) {
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
        sleep(Duration::from_millis(SLEEP_TIME));
        self.render();
    }

    fn render(&self) {
        clear_screen();
        let x = self.ball.x;
        let y = self.ball.y;
        let ball = y as usize * self.screen_width as usize + x as usize;
        let mut line: usize = 0;
        println!("Ball: {}\t Direction {}", self.ball, self.ball_direction);
        println!("{}","-".repeat(self.screen_width as usize));

        loop {
            if beetween( ball as isize, line as isize * self.screen_width, (line as isize + 1) * self.screen_width as isize ) {
                let mut print_string = " ".repeat(self.screen_width as usize);
                let bytes_string = unsafe { print_string.as_bytes_mut() };
                let ball_line_index = ball % self.screen_width as usize; 
                if ball_line_index < self.screen_width as usize{
                    bytes_string[ball_line_index] = 'o' as u8;
                }
                print!("{}",print_string);
            }
            
            line = line + 1;
            if line >= self.screen_height as usize {
                break;
            }
            println!()
        }
    }

    fn check_colision(&mut self) {
        if self.ball.y < 0.0 || self.ball.y > self.screen_height as f32 {
            self.ball_direction.flip_y();
            self.ball.y = max(min(self.screen_height as u16, self.ball.y as u16), 0) as f32;
        }
        if self.ball.x < 0.0 || self.ball.x > self.screen_width as f32 {
            self.ball_direction.flip_x();
            self.ball.x = max(min(self.screen_width as u16, self.ball.x as u16), 0) as f32
        }
    }
}

fn clear_screen() {
    // \x1B[2J limpa a tela inteira
    // \x1B[1;1H move o cursor de volta para o canto superior esquerdo (linha 1, coluna 1)
    print!("\x1B[2J\x1B[1;1H");

    // É importante dar o flush para garantir que o comando seja executado imediatamente
    stdout().flush().unwrap();
}

fn beetween(x: isize, a: isize, b: isize) -> bool {
    x > a && x < b
}
