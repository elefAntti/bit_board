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