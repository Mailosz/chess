
#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Empty = 0b0000,
    SpecialX = 0b0001,

    PawnW   = 0b0011,
    QueenW  = 0b1101,
    RookW   = 0b0101,
    BishopW = 0b1001,
    KnightW = 0b1011,
    KingW   = 0b1111,
    SpecialW= 0b0111,

    PawnB   = 0b0010,
    KingB   = 0b1100,
    QueenB  = 0b0100,
    RookB   = 0b1000,
    BishopB = 0b1010,
    KnightB = 0b0110,
    SpecialB= 0b1110,
}

#[derive(Clone, Copy)]
pub enum PlayerColor {
    White = 1,
    Black = 0
}

#[derive(Clone, Copy)]
pub struct Chessboard {
    pub squares : [Piece; 64]
}


#[derive(Clone, Copy)]
pub struct Game {
    pub turn : u32,
    pub player : PlayerColor,
    pub board : Chessboard
}

impl Game {
    pub fn new() -> Game {
        Game { turn: 0, player: PlayerColor::White, board: Chessboard::standard() }
    }


}

impl Chessboard {

    pub fn standard() -> Chessboard {
        Chessboard{
            squares : [
                Piece::RookW, Piece::KnightW, Piece::BishopW, Piece::QueenW, Piece::KingW, Piece::BishopW, Piece::KnightW, Piece::RookW,
                Piece::PawnW, Piece::PawnW, Piece::PawnW, Piece::PawnW, Piece::PawnW, Piece::PawnW, Piece::PawnW, Piece::PawnW,
                Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, 
                Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, 
                Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, 
                Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, 
                Piece::PawnB, Piece::PawnB, Piece::PawnB, Piece::PawnB, Piece::PawnB, Piece::PawnB, Piece::PawnB, Piece::PawnB, 
                Piece::RookB, Piece::KnightB, Piece::BishopB, Piece::QueenB, Piece::KingB, Piece::BishopB, Piece::KnightB, Piece::RookB,
            ]
        }
    }
}