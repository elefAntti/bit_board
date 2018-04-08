//#![feature(test)]
pub mod bitboard;
pub mod othello;
pub mod game;




mod ai
{
    use std::marker::PhantomData;
    use game::GameSituation;

    pub trait SituationEvaluator
    {
        type Situation; 
        const MAX_SCORE: i32;
        //Returns an evaluation of situation in the range [-MAX_SCORE, MAX_SCORE]
        //From the perspective of the current player
        fn evaluate_situation( situation: &Self::Situation ) -> i32;
    }

    pub trait MinMaxTraits
    {
        type Move: Clone;        
        type Situation: GameSituation<Move = Self::Move>;
        type Evaluator: SituationEvaluator<Situation = Self::Situation>;
    }

    pub struct MinMax< Traits: MinMaxTraits >
    {
        phantom: PhantomData< Traits >
    }

    impl<Traits: MinMaxTraits>  MinMax<Traits> 
    {
        pub fn new() -> Self
        {
            MinMax{phantom: PhantomData}
        }

        fn search( &self, situation: Traits::Situation, depth: u32 ) -> i32
        {
            if depth == 0 
            {
                return Traits::Evaluator::evaluate_situation( &situation );
            }
            situation.get_moves()
                    .map( |a_move| situation.copy_apply( a_move ) )
                    .map( |child_situation| -Self::search(self, child_situation.unwrap(), depth - 1) )
                    .max().unwrap_or( Traits::Evaluator::evaluate_situation( &situation ) )
        }

        pub fn search_root( &self, situation: &Traits::Situation, depth: u32 ) -> ( Option<Traits::Move>, i32 )
        {
            if situation.is_finished()
            {
                return ( None, Traits::Evaluator::evaluate_situation(&situation) );
            }
            let mut best_move:Option<Traits::Move> = None;
            let mut best_score = -Traits::Evaluator::MAX_SCORE;
            for a_move in situation.get_moves()
            {
                let child_score = -Self::search( self, situation.copy_apply( a_move.clone() ).unwrap(), depth - 1 ); 
                if  child_score >= best_score
                {
                    best_score = child_score;
                    best_move = Some( a_move );
                }
            }
            ( best_move, best_score )
        }
    }
}

use std::io;
use othello::OthelloSituation;
use game::GameSituation;

fn read_coord() -> Option<bitboard::Coord>
{
    let mut input_coord: Option<bitboard::Coord> = None;
	while input_coord.is_none()
	{
	    println!("Please enter coord (eg. a1) or 'pass'");       
		let mut input = String::new();
	    io::stdin()
	        .read_line(&mut input)
	        .expect("failed to read input.");

        if input.trim() == "pass"
        {
            return None;
        }

		let chars:Vec<char> = input.trim().chars().collect();
        if chars.len() == 2
        {
            let col = "abcdefgh".find(chars[0]);
            let row = chars[1].to_digit(10).unwrap_or(0) as u32;

            if let Some(col) = col 
            {
                if row > 0 && row <= 8
                {
                    input_coord = Some(bitboard::Coord::new( row - 1, col as u32 ));
                } 
            }
        }
	}
	input_coord
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
        let mut my_move = read_coord()?;
        while situation.copy_apply(my_move).is_none() 
        {
            println!("Invalid move");
            my_move = read_coord()?;
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
    const MAX_SCORE: i32 = 16;
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
}

impl OthelloMinMaxPlayer
{
    pub fn new() -> OthelloMinMaxPlayer
    {
        OthelloMinMaxPlayer{ algorithm: ai::MinMax::new() }
    }
}

impl game::Player for OthelloMinMaxPlayer
{
    type Move = <OthelloSituation as GameSituation>::Move;
    type Situation = OthelloSituation;
    fn make_move( &mut self, situation: &Self::Situation, _previous_move: Option<Self::Move> ) -> Option<Self::Move>
    {
        self.algorithm.search_root( situation, 3 ).0
    }
}