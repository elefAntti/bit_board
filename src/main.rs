struct Coord(u32);

impl Coord
{
    fn new( row: u32, col: u32 ) -> Coord
    {
        if col > 7 || row > 7
        {
            panic!("Overindexing the board row {}, col {}", row, col);
        }
        Coord( col + row * 8 )
    }
    fn get_row( &self ) -> u32
    {
        self.0 / 8
    }
    fn get_col( &self ) -> u32
    {
        self.0 % 8
    }
    fn get_idx( &self ) -> u32
    {
        self.0
    }
}

struct BitBoard(u64);

impl BitBoard
{
    fn get_value_at( &self, coord: Coord ) -> bool
    {
        ( self.0 >> coord.get_idx() ) & 1 == 1
    }

    fn set_value_at( &mut self, coord: Coord, value: bool )
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

    fn shift_up( &self ) -> BitBoard
    {
        BitBoard( self.0 >> 8 )
    }

    fn shift_down( &self ) -> BitBoard
    {
        BitBoard( self.0 << 8 )
    }

    fn shift_left( &self ) -> BitBoard
    {
        let mask = BitBoard::leftmost_column().0;
        BitBoard( ( self.0 & !mask ) >> 1 )
    }

    fn shift_right( &self ) -> BitBoard
    {
        let mask = BitBoard::rightmost_column().0;
        BitBoard( ( self.0 & !mask ) << 1 )
    }


    fn rightmost_column() -> BitBoard
    {
        BitBoard( 0b1000000010000000100000001000000010000000100000001000000010000000u64 )
    }

    fn leftmost_column() -> BitBoard
    {
        BitBoard( 0b0000000100000001000000010000000100000001000000010000000100000001u64 )
    }
    
    fn first_one(&self) -> Coord
    {
        Coord( self.0.trailing_zeros() )
    }

    fn print( &self ) -> ()
    {
        let mut board_as_characters: String = String::from("");
        for row in 0..8
        {
            for col in 0..8
            {
                board_as_characters += if self.get_value_at(Coord::new(row, col)) {"1"} else {"0"};
            }
            board_as_characters += "\n";
        }
        println!("{}", board_as_characters);
    }
}

fn test_pattern( ) -> BitBoard
{
    let mut board = BitBoard(0);
    board.set_value_at( Coord::new(0, 0), true );
    board.set_value_at( Coord::new(0, 7), true );
    board.set_value_at( Coord::new(7, 0), true );
    board.set_value_at( Coord::new(7, 7), true );
    board.set_value_at( Coord::new(2, 2), true );
    board
}

fn main() {
    let test_board = test_pattern();
    test_board.print();
    println!("Shift right:");
    test_board.shift_right().print();
    println!("Shift up:");
    test_board.shift_up().print();
    let test_coord = test_board.shift_up().first_one();
    println!("first one at: ({}, {})",test_coord.get_row(), test_coord.get_col());
    println!("Shift left:");
    test_board.shift_left().print();
    println!("Shift down:");
    test_board.shift_down().print();


}
