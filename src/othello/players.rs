use std::io;
use super::super::othello::OthelloSituation;
use super::super::othello::OthelloMove;
use super::super::game::GameSituation;
use super::super::{ai, game, bitboard, othello};

fn read_move() -> Option<OthelloMove>
{
    let mut input_coord: Option<bitboard::Coord> = None;
	while input_coord.is_none()
	{
	    println!("Please enter coord (eg. a1) or 'pass' or 'quit'");       
		let mut input = String::new();
	    io::stdin()
	        .read_line(&mut input)
	        .expect("failed to read input.");

        if input.trim() == "pass"
        {
            return Some(OthelloMove::Pass);
        }

        if input.trim() == "quit"
        {
            return None;
        }

        input_coord = bitboard::Coord::from_str(&input); 
	}
	Some(OthelloMove::Coord(input_coord?))
}

pub struct HumanOthelloPlayer{}

impl HumanOthelloPlayer
{
    pub fn new() -> HumanOthelloPlayer
    {
        HumanOthelloPlayer{}
    }
}

impl game::Player for HumanOthelloPlayer
{
    type Move = <OthelloSituation as GameSituation>::Move;
    type Situation = OthelloSituation;
    fn make_move( &mut self, situation: &Self::Situation, _previous_move: Option<Self::Move> ) -> Option<Self::Move>
    {
        println!("{}", situation);
        let mut my_move = read_move()?;
        while situation.copy_apply(my_move.clone()).is_none() 
        {
            println!("Invalid move");
            my_move = read_move()?;
        }
        Some(my_move)
    }
}


//This othello player always plays the first legal move
pub struct DummyOthelloPlayer{}

impl DummyOthelloPlayer
{
    pub fn new() -> DummyOthelloPlayer
    {
        DummyOthelloPlayer{}
    }
}

impl game::Player for DummyOthelloPlayer
{
    type Move = <OthelloSituation as GameSituation>::Move;
    type Situation = OthelloSituation;
    fn make_move( &mut self, situation: &Self::Situation, _previous_move: Option<Self::Move> ) -> Option<Self::Move>
    {
        situation.get_moves().next()
    }
}

struct SimpleOthelloEvaluator{}

impl ai::SituationEvaluator for SimpleOthelloEvaluator
{
    type Situation = othello::OthelloSituation; 
    const MAX_SCORE: i32 = 64;
    fn evaluate_situation( situation: &Self::Situation ) -> i32
    {
        let turn = situation.get_turn();
        if situation.is_finished()
        {
            return match situation.get_winner()
            {
                None => 0,
                Some( ref winner ) if winner == &turn => Self::MAX_SCORE, 
                Some( _ ) => -Self::MAX_SCORE, 
            }
        }
        situation.get_score(turn)
    }
}

struct OthelloMinMaxTraits
{
}

impl ai::MinMaxTraits for OthelloMinMaxTraits
{
    type Move = < OthelloSituation as GameSituation >::Move; 
    type Situation = OthelloSituation;
    type Evaluator = SimpleOthelloEvaluator;
}

pub struct OthelloMinMaxPlayer
{
    algorithm: ai::MinMax< OthelloMinMaxTraits >,
    max_depth: u32
}

impl OthelloMinMaxPlayer
{
    pub fn new( max_depth: u32 ) -> OthelloMinMaxPlayer
    {
        OthelloMinMaxPlayer{ algorithm: ai::MinMax::new(), max_depth }
    }
}

impl game::Player for OthelloMinMaxPlayer
{
    type Move = <OthelloSituation as GameSituation>::Move;
    type Situation = OthelloSituation;
    fn make_move( &mut self, situation: &Self::Situation, _previous_move: Option<Self::Move> ) -> Option<Self::Move>
    {
        self.algorithm.search_root( situation, self.max_depth ).0
    }
}

#[cfg(test)]
mod tests
{
    use test::Bencher;
    use game::Player;
    use super::*;

    #[bench]
    fn bench_min_max_player_d3(b: &mut Bencher) {
        let situation = OthelloSituation::new();
        let mut player = OthelloMinMaxPlayer::new( 3 );
        b.iter(|| player.make_move(&situation, None));
    }
}