extern crate bit_board;
use bit_board::game;
use bit_board::game::{GameSituation};
//use bit_board::bitboard::Coord;
use bit_board::othello;
use bit_board::othello::OthelloSituation;
use bit_board::{HumanOthelloPlayer, DummyOthelloPlayer};



fn play_a_bit() -> Option<OthelloSituation>
{
    let mut situation = OthelloSituation::new();
    for _ in 1..10
    {
        let coord = situation.get_moves().next()?;
        println!("Situation {} playing {}", situation, coord);
        situation = situation.copy_apply(coord)?;
    }
    Some(situation)
}

type OthelloMove = <OthelloSituation as GameSituation>::Move;  
type OthelloPlayer = game::Player< Situation = OthelloSituation, Move = OthelloMove >;


struct OthelloGame
{
    black_player: Box<OthelloPlayer>,
    white_player: Box<OthelloPlayer>,
    situation: OthelloSituation
}

impl OthelloGame
{
    fn new( black_player: Box<OthelloPlayer>, white_player: Box<OthelloPlayer> ) -> OthelloGame
    {
        OthelloGame{ black_player, white_player, situation: OthelloSituation::new() }
    }

    fn play(&mut self) -> Option<othello::Player> 
    {
        let previous_move: Option<OthelloMove> = None;
        while !self.situation.is_finished() 
        {
            let ref mut player_to_move = match self.situation.get_turn()
            {
                othello::Player::Black => &mut self.black_player,
                othello::Player::White => &mut self.white_player
            };
            let new_move = player_to_move.make_move( &self.situation, previous_move );
            if let Some(new_move) = new_move 
            {
                if let Some(new_situation) = self.situation.copy_apply(new_move) 
                {
                    self.situation = new_situation;
                }
                else 
                {
                    println!("Player {} returned an illegal move {}", self.situation.get_turn(), new_move );
                    return Some(self.situation.get_turn().opposite());
                }
            }
            else 
            {
                println!("Player {} returned a pass (giving up)", self.situation.get_turn() );
                return Some(self.situation.get_turn().opposite());
            }
        }
        self.situation.get_winner()
    }
}


fn main() 
{
    //play_a_bit();    
    let mut game = OthelloGame::new( Box::new( HumanOthelloPlayer::new() ), Box::new( DummyOthelloPlayer::new() ) );
    game.play();
}
