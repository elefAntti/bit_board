#![feature(test)]
extern crate test;

pub mod bitboard;
pub mod othello;
pub mod game;
pub mod ai;


use game::GameSituation;
type OthelloMove = <othello::OthelloSituation as GameSituation>::Move;  
type OthelloPlayer = game::Player< Situation = othello::OthelloSituation, Move = OthelloMove >;

pub struct OthelloGame
{
    black_player: Box<OthelloPlayer>,
    white_player: Box<OthelloPlayer>,
    situation: othello::OthelloSituation
}

impl OthelloGame
{
    pub fn new( black_player: Box<OthelloPlayer>, white_player: Box<OthelloPlayer> ) -> OthelloGame
    {
        OthelloGame{ black_player, white_player, situation: othello::OthelloSituation::new() }
    }

    pub fn play(&mut self) -> Option<othello::Player> 
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

//Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn two_random_players()
    {
        let mut game = OthelloGame::new( Box::new(othello::players::DummyOthelloPlayer::new()), Box::new(othello::players::DummyOthelloPlayer::new()) );
        game.play();
    }

    #[test]
    fn minmax_beats_dummy()
    {
        let mut game = OthelloGame::new( Box::new(othello::players::DummyOthelloPlayer::new()), Box::new(othello::players::OthelloMinMaxPlayer::new(3)) );
        assert_eq!(game.play(), Some(othello::Player::White));
    }


    #[test]
    fn minmax_beats_dummy2()
    {
        let mut game = OthelloGame::new( Box::new(othello::players::OthelloMinMaxPlayer::new(3)),Box::new(othello::players::DummyOthelloPlayer::new()) );
        assert_eq!(game.play(), Some(othello::Player::Black));
    }
}