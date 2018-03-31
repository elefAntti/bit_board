
use std::fmt;
use std::ops::BitOr;

#[derive(Clone,Copy)]
pub struct Coord(u32);

impl Coord
{
    pub fn new( row: u32, col: u32 ) -> Coord
    {
        if col > 7 || row > 7
        {
            panic!("Overindexing the board row {}, col {}", row, col);
        }
        Coord( col + row * 8 )
    }
    pub fn get_row( &self ) -> u32
    {
        self.0 / 8
    }
    pub fn get_col( &self ) -> u32
    {
        self.0 % 8
    }
    pub fn get_idx( &self ) -> u32
    {
        self.0
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.get_row(), self.get_col())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BitBoard(u64);

impl BitBoard
{
    pub fn empty() -> BitBoard
    {
        BitBoard(0)
    }
    pub fn is_empty(&self) -> bool
    {
        self.0 == 0
    }
    pub fn get_value_at( &self, coord: Coord ) -> bool
    {
        ( self.0 >> coord.get_idx() ) & 1 == 1
    }

    pub fn set_value_at( &mut self, coord: Coord, value: bool )
    {
        let idx = coord.get_idx();
        if value
        {
            self.0 |= 1 << idx;
        }
        else
        {
            self.0 &= !(1 << idx);       
        }
    }

    pub fn shift_up( &self ) -> BitBoard
    {
        BitBoard( self.0 >> 8 )
    }

    pub fn shift_down( &self ) -> BitBoard
    {
        BitBoard( self.0 << 8 )
    }

    pub fn shift_left( &self ) -> BitBoard
    {
        let mask = BitBoard::leftmost_column().0;
        BitBoard( ( self.0 & !mask ) >> 1 )
    }

    pub fn shift_right( &self ) -> BitBoard
    {
        let mask = BitBoard::rightmost_column().0;
        BitBoard( ( self.0 & !mask ) << 1 )
    }


    pub fn rightmost_column() -> BitBoard
    {
        BitBoard( 0b1000000010000000100000001000000010000000100000001000000010000000u64 )
    }

    pub fn leftmost_column() -> BitBoard
    {
        BitBoard( 0b0000000100000001000000010000000100000001000000010000000100000001u64 )
    }
    
    pub fn first_one(&self) -> Option<Coord>
    {
        if self.is_empty()
        {
            None
        }
        else 
        {
            Some(Coord( self.0.trailing_zeros() ))
        }
    }

}

impl PartialEq for BitBoard {
    fn eq(&self, other: &BitBoard) -> bool {
        self.0 == other.0
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard( self.0 | other.0 )
    }
}

impl fmt::Display for BitBoard
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8
        {
            write!(f, "\n")?;
            for col in 0..8
            {

                if self.get_value_at(Coord::new(row, col))
                {
                    write!(f, "1")?;
                }
                else 
                {
                    write!(f, "0")?;
                }
            }
        }
        write!(f,"")
    }
}


#[derive(Debug)]
pub struct BoardIterator(BitBoard);

impl Iterator for BoardIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord>
    {
        let pos = self.0.first_one()?;
        self.0.set_value_at(pos, false);
        Some(pos)
    }
}

impl IntoIterator for BitBoard {
    type Item = Coord;
    type IntoIter = BoardIterator;

    fn into_iter(self) -> Self::IntoIter 
    {
        BoardIterator(self)
    }
}

//Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod test
{
    use super::*;

    fn test_pattern( ) -> BitBoard
    {
        let mut board = BitBoard::empty();
        board.set_value_at( Coord::new(0, 0), true );
        board.set_value_at( Coord::new(0, 7), true );
        board.set_value_at( Coord::new(7, 0), true );
        board.set_value_at( Coord::new(7, 7), true );
        board.set_value_at( Coord::new(2, 2), true );
        board
    }

    #[test]
    fn test_board_creation()
    {
        //When bord is printed, rows are 'reversed' because msb is to the right
        //So the coordinate system is reverse to what you would expect
        assert_eq!(test_pattern(), BitBoard(0b10000001_00000000_00000000_00000000_00000000_00000100_00000000_10000001u64))
    }

    #[test]
    fn test_print_board()
    {
        let out = format!("{}", test_pattern());
        assert_eq!(out, "\n10000001\n00000000\n00100000\n00000000\n00000000\n00000000\n00000000\n10000001");
    } 
    #[test]
    fn test_shift_right()
    {
        let out = format!("{}", test_pattern().shift_right());
        assert_eq!(out, "\n01000000\n00000000\n00010000\n00000000\n00000000\n00000000\n00000000\n01000000");
    } 
    #[test]
    fn test_shift_up()
    {
        let out = format!("{}", test_pattern().shift_up());
        assert_eq!(out, "\n00000000\n00100000\n00000000\n00000000\n00000000\n00000000\n10000001\n00000000");
    } 
    #[test]
    fn test_shift_left()
    {
        let out = format!("{}", test_pattern().shift_left());
        assert_eq!(out, "\n00000010\n00000000\n01000000\n00000000\n00000000\n00000000\n00000000\n00000010");
    } 
    #[test]
    fn test_shift_down()
    {
        let out = format!("{}", test_pattern().shift_down());
        assert_eq!(out, "\n00000000\n10000001\n00000000\n00100000\n00000000\n00000000\n00000000\n00000000");
    } 
}