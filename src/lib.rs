//#![feature(test)]
pub mod bitboard;
pub mod othello;

pub mod game
{
    pub trait GameSituation: Sized
    {
        type Move;
        type MoveIterator: Iterator<Item = Self::Move>; 
        type Role: PartialEq + Sized;

        fn copy_apply( &self, the_move: Self::Move ) -> Option<Self>;
        fn get_moves( &self ) -> Self::MoveIterator;

        fn get_turn( &self ) -> Self::Role;
        fn is_finished( &self ) -> bool;
        fn get_winner( &self ) -> Option<Self::Role>; 
    }

    pub trait Player 
    {
        type Move;       
        type Situation: GameSituation<Move = Self::Move>;
        fn make_move( &mut self, situation: &Self::Situation, previous_move: Option<Self::Move> ) -> Option<Self::Move>;
    }
}

use othello::OthelloSituation;
use game::GameSituation;

struct HumanOthelloPlayer{}

impl game::Player for HumanOthelloPlayer
{
    type Move = <OthelloSituation as GameSituation>::Move;
    type Situation = OthelloSituation;
    fn make_move( &mut self, situation: &Self::Situation, previous_move: Option<Self::Move> ) -> Option<Self::Move>
    {
        println!("{}", situation);
        situation.get_moves().next()
    }
}