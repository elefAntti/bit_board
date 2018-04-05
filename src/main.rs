extern crate bit_board;
use std::fmt;
use bit_board::bitboard::{Coord, Direction, BitBoard};
use bit_board::Othello::OthelloSituation;




/*fn test_pattern( ) -> BitBoard
{
    let mut board = BitBoard::empty();
    board.set_value_at( Coord::new(0, 0), true );
    board.set_value_at( Coord::new(0, 7), true );
    board.set_value_at( Coord::new(7, 0), true );
    board.set_value_at( Coord::new(7, 7), true );
    board.set_value_at( Coord::new(2, 2), true );
    board
}*/

fn play_a_bit() -> Option<OthelloSituation>
{
    let mut situation = OthelloSituation::new();
    for i in 1..10
    {
        let coord = situation.get_moves().next()?;
        println!("Situation {} playing {}", situation, coord);
        situation = situation.apply_move(coord)?;
    }
    Some(situation)
}

fn main() {
    /*let test_board = test_pattern();
    println!("Test pattern: {}", test_board);
    println!("Shift right: {}", test_board.shift_right());
    println!("Shift up: {}", test_board.shift_up());
    println!("first one at: {}",test_board.shift_up().first_one().expect("There should be a one on board"));
    println!("Shift left: {}", test_board.shift_left());
    println!("Shift down: {}", test_board.shift_down());
    println!("Original or shift down: {}", test_board.shift_down() | test_board );
    for coord in test_board
    {
        println!("{}", coord );
    }*/
    println!("Board: {}", OthelloSituation::new());
    println!("Board: {}", OthelloSituation::new().apply_move(Coord::new(2,3)).unwrap());
    play_a_bit();    
}
