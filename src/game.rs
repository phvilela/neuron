use std::{
    cmp::max,
    io::{Error, ErrorKind::Other, Result, Write, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use rand::{random, random_range};

use crate::vector::Vector;

const SLEEP_TIME: u64 = 10;

const STDIO_MSG: &str = "Error handling stdio";

const BOUNCE_BOOST: f32 = 1.1;

const DEFAULT_FINAL_SCORE: u8 = 5;

pub struct Game {
    ball: Vector,
    ball_direction: Vector,
    ball_speed: f32,
    player_a: i16,
    player_b: i16,
    score_a: u8,
    score_b: u8,
    screen_width: usize,
    screen_height: usize,
}

impl Game {
    const DEFAULT_BALL_SPEED: f32 = 8.0 / 1000.0;

    pub fn new(w: usize, h: usize) -> Game {
        Game {
            ball: Vector {
                x: w as f32 / 2.0,
                y: h as f32 / 2.0,
            },
            ball_direction: Vector::ZERO,
            ball_speed: Self::DEFAULT_BALL_SPEED * w as f32,
            player_a: 0,
            player_b: 0,
            score_a: 0,
            score_b: 0,
            screen_width: w,
            screen_height: h - 2,
        }
    }

    pub fn start(&mut self) {
        self.ball_direction = gen_direction();
        enable_raw_mode().expect(STDIO_MSG);
        clear_screen().expect(STDIO_MSG);
        loop {
            match self.update() {
                Ok(0) => {}
                Ok(n) => {
                    if n == 1 {
                        println!("Player 1 wins {} x {}\r", self.score_a, self.score_b)
                    } else if n == 2 {
                        println!("Player 2 wins {} x {}\r", self.score_b, self.score_a)
                    }
                    break;
                }
                Err(_) => break,
            };
        }
        disable_raw_mode().expect("Error restoring stdio mode");
    }

    fn read_input(&mut self) -> Result<u8> {
        let mut moves: u8 = 0;
        let before = Instant::now();
        while event::poll(Duration::ZERO)? {
            if let Event::Key(key_pressed) = event::read()? {
                match key_pressed.code {
                    KeyCode::Esc => {
                        return Err(Error::new(Other, "Jogo Encerrado"));
                    }
                    KeyCode::Char('w') => {
                        moves |= 0b11;
                    }
                    KeyCode::Char('s') => {
                        moves |= 0b10;
                    }
                    KeyCode::Up => {
                        moves |= 0b1100;
                    }
                    KeyCode::Down => {
                        moves |= 0b1000;
                    }

                    _ => {}
                }
            }
        }
        sleep(Duration::from_millis(20) - (before - Instant::now()));
        Ok(moves)
    }

    fn update_players(&mut self, moves: u8) {
        let move_size: i16 = max(self.screen_height / 16, 2) as i16;
        if moves & 0b10 != 0 {
            if moves & 0b01 == 0 {
                self.player_a += move_size;
            } else {
                self.player_a -= move_size;
            }
        } else if moves & 0b1000 != 0 {
            if moves & 0b0100 == 0 {
                self.player_b += move_size;
            } else {
                self.player_b -= move_size;
            }
        }
        let height = self.screen_height;
        self.player_a = self.player_a.clamp(1, (height - 1).try_into().unwrap());
        self.player_b = self.player_b.clamp(1, (height - 1).try_into().unwrap());
    }

    fn reset_ball(&mut self) {
        self.ball_direction = gen_direction();
        self.ball = Vector {
            x: (self.screen_width / 2) as f32,
            y: (self.screen_height / 2) as f32,
        };
        self.ball_speed = Self::DEFAULT_BALL_SPEED * self.screen_width as f32;
    }

    fn check_colision(&mut self) -> Option<u8> {
        let height: f32 = (self.screen_height) as f32;
        let width: f32 = (self.screen_width) as f32;
        if self.ball.y < 0.0 || self.ball.y > height {
            self.ball_direction.flip_y();
            self.ball.y = self.ball.y.clamp(0.0, (height - f32::MIN) as f32);
        }
        if self.ball.x < 0.0 || self.ball.x > width as f32 {
            self.ball.x = self.ball.x.clamp(0.0, (width - f32::MIN) as f32);
            let goal_treshold = max(self.screen_height / 16, 1);
            if self.ball.x == 0.0 {
                if self.player_a.abs_diff(self.ball.y as i16) >= goal_treshold as u16 {
                    return Some(0);
                }
            } else if self.player_b.abs_diff(self.ball.y as i16) >= goal_treshold as u16 {
                return Some(1);
            }

            self.ball_direction.flip_x();

            self.ball_speed *= BOUNCE_BOOST;
            self.ball_speed = self.ball_speed.min(width as f32 / 16.0);
        }
        None
    }

    fn update(&mut self) -> Result<i8> {
        let moves = self.read_input()?;

        self.update_players(moves);
        self.ball = self.ball.add(self.ball_direction.mul(self.ball_speed));
        if let Some(g) = self.check_colision() {
            if g == 0 {
                self.score_b += 1;
                if self.score_b == DEFAULT_FINAL_SCORE {
                    return Ok(1);
                }
            } else {
                self.score_a += 1;
                if self.score_a == DEFAULT_FINAL_SCORE {
                    return Ok(2);
                }
            }
            self.reset_ball();
        }
        sleep(Duration::from_millis(SLEEP_TIME));
        self.render();
        Ok(0)
    }

    fn render(&self) {
        clear_screen().expect("");
        let x = self.ball.x;
        let y = self.ball.y;
        let ball = y as usize * self.screen_width as usize + x as usize;
        let mut line: usize = 0;
        println!(
            "Ball: {}\t A : {} B : {}\r",
            self.ball, self.score_a, self.score_b
        );
        println!("{}\r", "-".repeat(self.screen_width as usize));
        let player_size = max(self.screen_height / 16, 2);
        loop {
            let mut player_in_row: u8 = 0;
            if (self.player_a as usize).abs_diff(line) <= player_size {
                player_in_row |= 0x1;
            }
            if (self.player_b as usize).abs_diff(line) <= player_size {
                player_in_row |= 0x2;
            }
            if beetween(
                ball as isize,
                (line * self.screen_width) as isize,
                (line as isize + 1) * self.screen_width as isize,
            ) {
                let mut print_string = " ".repeat(self.screen_width as usize);
                let bytes_string = unsafe { print_string.as_bytes_mut() };
                let ball_line_index = ball % self.screen_width as usize;
                if ball_line_index < self.screen_width as usize {
                    bytes_string[ball_line_index] = 'o' as u8;
                }
                if player_in_row & 0b1 != 0 {
                    bytes_string[0] = '|' as u8
                };
                if player_in_row & 0b10 != 0 {
                    bytes_string[self.screen_width as usize - 1 as usize] = '|' as u8
                };
                print!("{}\r", print_string);
                player_in_row = 0;
            }
            if player_in_row != 0 {
                if player_in_row == 1 {
                    print!("|")
                } else if player_in_row == 2 {
                    print!("{}|", " ".repeat(self.screen_width as usize - 1));
                } else if player_in_row == 3 {
                    print!("|{}|", " ".repeat(self.screen_width as usize - 2));
                }
            }

            line = line + 1;
            if line >= self.screen_height as usize {
                print!("\r");
                stdout().flush().unwrap();
                break;
            }
            print!("\r\n");
        }
    }
}

fn gen_direction() -> Vector {
    let rand_rad: f32 = random_range(0.0..=0.7853983);
    let side: bool = random();
    let x = if side {
        rand_rad.cos()
    } else {
        -rand_rad.cos()
    };
    Vector {
        x: x,
        y: rand_rad.sin(),
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
