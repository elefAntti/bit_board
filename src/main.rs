extern crate bit_board;
use bit_board::OthelloGame;
use bit_board::othello::players::{HumanOthelloPlayer, OthelloMinMaxPlayer};


fn main() 
{
    let mut game = OthelloGame::new( Box::new( HumanOthelloPlayer::new() ), Box::new( OthelloMinMaxPlayer::new(7) ) );
    match game.play()
    {
        None => println!("It's a tie"),
        Some(winner) => println!("{} won" ,winner)
    }
}
