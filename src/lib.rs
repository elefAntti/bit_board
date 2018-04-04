pub mod bitboard;

pub mod Othello
{
    use std::fmt;
    use super::bitboard::{Coord, Direction, BitBoard};

    #[derive(PartialEq)]
    enum Player
    {
        Black,
        White
    }

    impl Player
    {
        fn opposite( &self ) -> Player
        {
            if self == &Player::Black { Player::White } else { Player::Black }
        }
    }

    pub struct OthelloSituation
    {
        black_board: BitBoard,
        white_board: BitBoard,
        moves: BitBoard, //Moves are generated for all the situations
        //Because even terminal ones need to know if there are any legal moves
        //But would it be cheaper to return an iterator of child states. ie. apply the moves as you generate?
        //TODO: test
        turn: Player
    }

    // move is one-hot bitboard indicating where the player wants to play
    // Return value is a bitboard containing the pieces to be flipped
    fn apply_move_towards( own_board: BitBoard, other_board: BitBoard, move_board: BitBoard, direction: Direction ) -> BitBoard
    {
        let mut move_tmp = move_board.shift( direction ) & other_board;
        let mut delta = move_tmp;
        while !(move_tmp.is_empty())
        {
            move_tmp = move_tmp.shift( direction );
            if !((move_tmp & own_board).is_empty())
            {
                return delta;
            }
            delta |= move_tmp;
            move_tmp &= other_board;                        
        }
        BitBoard::empty()
    }

    fn delta_for_move( own_board: BitBoard, other_board: BitBoard, move_board: BitBoard ) -> BitBoard
    {
        let empty_spaces = !own_board & !other_board;
        if (empty_spaces & move_board).is_empty()
        {
            return BitBoard::empty();
        }
        apply_move_towards( own_board, other_board, move_board, Direction::Right ) |
        apply_move_towards( own_board, other_board, move_board, Direction::UpRight ) |
        apply_move_towards( own_board, other_board, move_board, Direction::Up ) |
        apply_move_towards( own_board, other_board, move_board, Direction::UpLeft ) |
        apply_move_towards( own_board, other_board, move_board, Direction::Left ) |
        apply_move_towards( own_board, other_board, move_board, Direction::DownLeft ) |
        apply_move_towards( own_board, other_board, move_board, Direction::Down ) |
        apply_move_towards( own_board, other_board, move_board, Direction::DownRight )
    }

    fn generate_moves_towards( own_board: BitBoard, other_board: BitBoard, direction: Direction ) -> BitBoard
    {
        let empty_spaces = !own_board & !other_board;
        let mut moves = BitBoard::empty();
        let mut move_tmp = own_board.shift( direction ) & other_board;
        while !(move_tmp.is_empty())
        {
            move_tmp = move_tmp.shift( direction );
            let moves_generated_this_round = move_tmp & empty_spaces;
            move_tmp &= other_board;
            moves |= moves_generated_this_round;
        }
        moves
    }

    fn generate_moves( own_board: BitBoard, other_board: BitBoard ) -> BitBoard
    {
        generate_moves_towards( own_board, other_board, Direction::Right ) |
        generate_moves_towards( own_board, other_board, Direction::UpRight ) |
        generate_moves_towards( own_board, other_board, Direction::Up ) |
        generate_moves_towards( own_board, other_board, Direction::UpLeft ) |
        generate_moves_towards( own_board, other_board, Direction::Left ) |
        generate_moves_towards( own_board, other_board, Direction::DownLeft ) |
        generate_moves_towards( own_board, other_board, Direction::Down ) |
        generate_moves_towards( own_board, other_board, Direction::DownRight )
    }

    impl OthelloSituation
    {
        pub fn new() -> OthelloSituation
        {
            let mut black_board = BitBoard::empty();
            black_board.set_value_at(Coord::new(3,4),true);
            black_board.set_value_at(Coord::new(4,3),true);
            let mut white_board = BitBoard::empty();
            white_board.set_value_at(Coord::new(3,3),true);
            white_board.set_value_at(Coord::new(4,4),true);
            let moves = generate_moves( black_board, white_board );
            OthelloSituation{ black_board, white_board, moves, turn: Player::Black }
        }

        fn get_own_board( &self ) -> BitBoard
        {
            if self.turn == Player::Black
            {
                self.black_board
            } 
            else
            {
                self.white_board
            }
        }

        fn get_opponent_board( &self ) -> BitBoard
        {
            if self.turn == Player::Black
            {
                self.white_board
            } 
            else
            {
                self.black_board
            }
        }

        fn generate_moves( &mut self )
        {
            self.moves = generate_moves( self.get_own_board(), self.get_opponent_board() );            
        }

        pub fn apply_move( &self, move_coord: Coord ) -> Option<OthelloSituation>
        {
            let move_as_board = BitBoard::empty().with_one_at( move_coord );
            let own_board = self.get_own_board(); 
            let opponent_board = self.get_opponent_board(); 

            let delta = delta_for_move( own_board, opponent_board, move_as_board ); 

            if delta.is_empty()
            {
                return None;
            }

            let mut black_board = self.black_board ^ delta;
            let mut white_board = self.white_board ^ delta;

            if self.turn == Player::Black
            {
                black_board |= move_as_board;
            } 
            else
            {
                white_board |= move_as_board;
            }

            let mut new_situation = OthelloSituation{ black_board, white_board, turn: self.turn.opposite(), moves: BitBoard::empty() };

            new_situation.generate_moves();

            Some( new_situation )
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
                    else if self.moves.get_value_at(Coord::new(row, col))
                    {
                        write!(f, ",")?;                    
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

}