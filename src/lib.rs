
use std::fmt;

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

#[derive(Debug)]
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
    
    pub fn first_one(&self) -> Coord
    {
        Coord( self.0.trailing_zeros() )
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
        if self.0.is_empty() {
            return None;
        }
        let pos = self.0.first_one();
        self.0.set_value_at(pos, false);
        Some(pos)
    }
}

impl IntoIterator for BitBoard {
    type Item = Coord;
    type IntoIter = BoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator(self)
    }
}

