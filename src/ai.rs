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