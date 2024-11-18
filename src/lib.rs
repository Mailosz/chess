

use game::Game;
use wasm_bindgen::{convert::{IntoWasmAbi, OptionIntoWasmAbi, ReturnWasmAbi, WasmAbi}, prelude::*};

mod game;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_available_moves(position: u32) -> Vec<u32> {

    let mut moves = Vec::new();

    moves.push(1);
    moves.push(3);
    moves.push(5);
    moves.push(7);
    moves.push(9);
    moves.push(11);
    moves.push(13);

    return moves;

}


#[wasm_bindgen]
pub fn get_board() -> JsValue {

    let game = Game::new();

    let mut squares: Vec<&str> = Vec::with_capacity(64);

    for piece in game.board.squares.into_iter() {

        squares.push(match piece {
            game::Piece::Empty => "",
            game::Piece::SpecialX => "X",
            game::Piece::PawnW => "PW",
            game::Piece::QueenW => "QW",
            game::Piece::RookW => "RW",
            game::Piece::BishopW => "BW",
            game::Piece::KnightW => "NW",
            game::Piece::SpecialW => "SW",
            game::Piece::KingW => "KW",
            game::Piece::PawnB => "PB",
            game::Piece::KingB => "KB",
            game::Piece::QueenB => "QB",
            game::Piece::RookB => "RB",
            game::Piece::BishopB => "BB",
            game::Piece::KnightB => "NB",
            game::Piece::SpecialB => "SB",
        });
    } 

    return JsValue::from_str(squares.join(",").as_str());

}

