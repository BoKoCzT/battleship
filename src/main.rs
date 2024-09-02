// Imports of needed libraries
use std::io::{self, write};
use rand::rng;

// We define size of the game board
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

fn main() {
    loop {

    }
}

fn get_player_input() {

}

fn generate_opponent_move() {

}