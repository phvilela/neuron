use std::sync::OnceLock;
use terminal_size::terminal_size;
mod game;
mod vector;

static SCREEN_WIDTH: OnceLock<u16> = OnceLock::new();
static SCREEN_HEIGHT: OnceLock<u16> = OnceLock::new();

fn set_size() {
    if let Some((w, h)) = terminal_size() {
        let _ = SCREEN_WIDTH.set(w.0);
        let _ = SCREEN_HEIGHT.set(h.0);
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

fn main() {
    set_size();
    let mut game: game::Game = game::Game::new(screen_width().into() , screen_height().into());
    game.start()
}
