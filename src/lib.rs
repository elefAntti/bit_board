pub mod bitboard;

pub mod Othello
{
    use std::fmt;
    use super::bitboard::{Coord, Direction, BitBoard};
    enum Player
    {
        Black,
        White
    }

    pub struct OthelloSituation
    {
        black_board: BitBoard,
        white_board: BitBoard,
        moves: BitBoard,
        turn: Player
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