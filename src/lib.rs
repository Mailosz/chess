

use std::{borrow::Borrow, cell::RefCell};

use game::{Chessboard, Game, Piece};
use wasm_bindgen::{convert::{IntoWasmAbi, OptionIntoWasmAbi, ReturnWasmAbi, WasmAbi}, prelude::*};
use once_cell::sync::Lazy;

mod game;


static mut CURRENT_GAME : Option<Game> = Option::None;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

fn no_moves() -> Vec<u32> {
    return Vec::new();
}

fn white_pawn_moves(board: Chessboard, position: u32) -> Vec<u32> {

    let mut vec = Vec::new();

    if (position < 56) {
        
        if board.squares[(position + 8) as usize] == Piece::Empty {
            vec.push(position + 8);

            if (position < 16 && position >= 8 && board.squares[(position + 16) as usize] == Piece::Empty) {
                vec.push(position + 16);
            }
        }
    }

    return vec;
}

fn black_pawn_moves(board: Chessboard, position: u32) -> Vec<u32> {

    let mut vec = Vec::new();

    if (position >= 8) {
        
        if board.squares[(position - 8) as usize] == Piece::Empty {
            vec.push(position - 8);

            if (position >= 48 && position < 56 && board.squares[(position - 16) as usize] == Piece::Empty) {
                vec.push(position - 16);
            }
        }
    }

    return vec;
}

#[wasm_bindgen]
pub fn get_available_moves(position: u32) -> Vec<u32> {

    let game = get_current_game();

    let piece = game.board.squares[position as usize];

    let moves = match piece {
        game::Piece::Empty => no_moves(),
        game::Piece::SpecialX => no_moves(),
        game::Piece::PawnW => white_pawn_moves(game.board, position),
        game::Piece::QueenW => no_moves(),
        game::Piece::RookW => no_moves(),
        game::Piece::BishopW => no_moves(),
        game::Piece::KnightW => no_moves(),
        game::Piece::SpecialW => no_moves(),
        game::Piece::KingW => no_moves(),
        game::Piece::PawnB => black_pawn_moves(game.board, position),
        game::Piece::KingB => no_moves(),
        game::Piece::QueenB => no_moves(),
        game::Piece::RookB => no_moves(),
        game::Piece::BishopB => no_moves(),
        game::Piece::KnightB => no_moves(),
        game::Piece::SpecialB => no_moves(),
    };

    return moves;

}

#[wasm_bindgen]
pub fn new_game() -> () {
    unsafe{
        CURRENT_GAME.replace(Game::new());
    }
}

fn get_current_game() -> Game {
    let game;
    unsafe {
        game = CURRENT_GAME.unwrap();
    }
    return game;
}


#[wasm_bindgen]
pub fn get_board() -> JsValue {

    let game = get_current_game();

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

