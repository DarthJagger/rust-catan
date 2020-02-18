pub mod display;
mod terminal_player;
mod action_parser;

pub use action_parser::parse_action;

use catan::board;
use catan::state::TricellState;
use catan::game::{play, Phase};

use terminal_player::TerminalPlayer;

fn main() {
    println!("[START]");
    let mut state = board::setup::random_default::<TricellState>(1);

    let mut player = TerminalPlayer::new();
    play::play_until_finished(&mut Phase::START_GAME, &mut *state, &mut player);

    println!("[END]");
}
