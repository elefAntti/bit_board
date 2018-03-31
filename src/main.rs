extern crate bit_board;
use std::fmt;
use bit_board::Coord;
use bit_board::BitBoard;


enum Player
{
    Black,
    White
}

struct OthelloSituation
{
    black_board: BitBoard,
    white_board: BitBoard,
    turn: Player
}

impl OthelloSituation
{
    fn new() -> OthelloSituation
    {
        let mut black_board = BitBoard::empty();
        black_board.set_value_at(Coord::new(3,4),true);
        black_board.set_value_at(Coord::new(4,3),true);
        let mut white_board = BitBoard::empty();
        white_board.set_value_at(Coord::new(3,3),true);
        white_board.set_value_at(Coord::new(4,4),true);
        OthelloSituation{ black_board, white_board, turn: Player::Black }
    }
}

impl fmt::Display for OthelloSituation
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  abcdefgh")?;        
        for row in 0..8
        {
            write!(f, "\n {}",row)?;
            for col in 0..8
            {

                if self.black_board.get_value_at(Coord::new(row, col))
                {
                    write!(f, "●")?;
                }
                else if self.white_board.get_value_at(Coord::new(row, col))
                {
                    write!(f, "○")?;                    
                }
                else 
                {
                    write!(f, ".")?;                    
                }
            }
            write!(f, "{}",row)?;            
        }
        write!(f, "\n  abcdefgh")
    }
}


fn test_pattern( ) -> BitBoard
{
    let mut board = BitBoard::empty();
    board.set_value_at( Coord::new(0, 0), true );
    board.set_value_at( Coord::new(0, 7), true );
    board.set_value_at( Coord::new(7, 0), true );
    board.set_value_at( Coord::new(7, 7), true );
    board.set_value_at( Coord::new(2, 2), true );
    board
}

fn main() {
    let test_board = test_pattern();
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
    }
    println!("Board: {}", OthelloSituation::new());
    
}
