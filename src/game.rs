use std::{
    cmp::{max, min}, io::{Error, ErrorKind::Other, Result, stdout}, thread::sleep, time::{Duration, Instant},
};

use crossterm::{cursor::MoveTo, event::{self, Event, KeyCode}, execute, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}};
use rand::random_range;

use crate::vector::Vector;

const SLEEP_TIME: u64 = 10;

const STDIO_MSG : &str = "Error handling stdio";

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
        enable_raw_mode().expect(STDIO_MSG);
        clear_screen().expect(STDIO_MSG);
        loop {
            if let Err(_) = self.update() { break;};
        }
        disable_raw_mode().expect("Error restoring stdio mode");
    }

    
    fn read_input(&mut self) -> Result<u8> {
        let mut moves : u8 = 0;
        let before = Instant::now();
        while event::poll(Duration::ZERO)? {
            if let Event::Key(key_pressed) = event::read()? {
                match key_pressed.code {
                    KeyCode::Esc => { return  Err(Error::new(Other,"Jogo Encerrado")); },
                    KeyCode::Char('w') => { moves |= 0b11 ;},
                    KeyCode::Char('s') => {moves |= 0b10;},
                    KeyCode::Up => {moves |= 0b1100;},
                    KeyCode::Down => {moves |= 0b1000;},

                    _ => {}
                }
            }
        }
        sleep(Duration::from_millis(20) - (before - Instant::now()));
        Ok(moves)
    }

    fn update_players(& mut self,moves: u8) {
        if moves & 0b10 != 0 {
            if moves & 0b01 == 0 {
                self.player_a -= 1;
            } else {
                self.player_a += 1
            }
        } else if moves & 0b1000 != 0 {
            if moves & 0b0100 == 0 {
                self.player_b -= 1;
            } else {
                self.player_b += 1
            }
        }
        let height = self.screen_height;
        self.player_a = self.player_a.clamp(0, height.try_into().unwrap());
        self.player_b = self.player_b.clamp(0, height.try_into().unwrap());
    }
    
    fn update(&mut self) -> Result<()>{
        let moves = self.read_input()?;
        
        self.update_players(moves);
        self.ball = self.ball.add(self.ball_direction.mul(self.ball_speed));
        self.check_colision();
        sleep(Duration::from_millis(SLEEP_TIME));
        self.render();
        Ok(())
    }

    fn render(&self) {
        clear_screen().expect("");
        let x = self.ball.x;
        let y = self.ball.y;
        let ball = y as usize * self.screen_width as usize + x as usize;
        let mut line: usize = 0;
        println!("Ball: {}\t A : {} B : {}", self.ball, self.player_a,self.player_b);
        println!("\r{}", "-".repeat(self.screen_width as usize));

        loop {
            if beetween(
                ball as isize,
                line as isize * self.screen_width,
                (line as isize + 1) * self.screen_width as isize,
            ) {
                let mut print_string = " ".repeat(self.screen_width as usize);
                let bytes_string = unsafe { print_string.as_bytes_mut() };
                let ball_line_index = ball % self.screen_width as usize;
                if ball_line_index < self.screen_width as usize {
                    bytes_string[ball_line_index] = 'o' as u8;
                }
                print!("\r{}", print_string);
            }

            line = line + 1;
            if line >= self.screen_height as usize {
                break;
            }
            println!("\r");
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

fn clear_screen() -> Result<()> {
    execute!(
            stdout(),
            Clear(ClearType::All), // Limpa toda a tela visível
            MoveTo(0, 0)           // Move o cursor de volta para o canto superior esquerdo (1,1)
        )?;
    Ok(())
}

fn beetween(x: isize, a: isize, b: isize) -> bool {
    x > a && x < b
}
