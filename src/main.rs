// A simple Battleship game in Rust Language
// for learning use

// Imports of needed libraries
use std::io::{self, write};
use rand::rng;

// Defines size of the game board
const BOARD_SIZE: usize = 10;

struct Board{
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}

enum CellState{
    Empty,
    Ship,
    Hit,
    Miss
}

impl Board{
    fn new() -> Self{
        Board{
            grid: [[CellState::Empty;BOARD_SIZE];BOARD_SIZE],
            ships: vec::new(),
        }
    }
}

// This function places randomly ships on the game board.
fn place_ship(&mut self, size: usize){
    let mut rng = rand::thread_rng();
}


fn main() {
    loop {

    }
}

fn get_player_input() {

}

fn generate_opponent_move() {

}