// crate::components::chess_board

use crate::components::chess_piece::ChessPiece;
use crate::rules::bishop;
use crate::rules::king;
use crate::rules::knight;
use crate::rules::pawn;
use crate::rules::queen;
use crate::rules::rook;
use crate::utils::logs::log;

use std::fmt::Display;


pub struct ChessBoard {
    contents: [[ChessPiece; 8]; 8],
}


impl Default for ChessBoard {

    fn default() -> Self {
        let no = ChessPiece::default();
        let bp = ChessPiece::construct(String::from("black"), String::from("pawn"));
        let br = ChessPiece::construct(String::from("black"), String::from("rook"));
        let bn = ChessPiece::construct(String::from("black"), String::from("knight"));
        let bb = ChessPiece::construct(String::from("black"), String::from("bishop"));
        let bq = ChessPiece::construct(String::from("black"), String::from("queen"));
        let bk = ChessPiece::construct(String::from("black"), String::from("king"));
        let wp = ChessPiece::construct(String::from("white"), String::from("pawn"));
        let wr = ChessPiece::construct(String::from("white"), String::from("rook"));
        let wn = ChessPiece::construct(String::from("white"), String::from("knight"));
        let wb = ChessPiece::construct(String::from("white"), String::from("bishop"));
        let wq = ChessPiece::construct(String::from("white"), String::from("queen"));
        let wk = ChessPiece::construct(String::from("white"), String::from("king"));
        Self {
            contents: [
                [ br.clone(), bn.clone(), bb.clone(), bq.clone(), bk.clone(), bb.clone(), bn.clone(), br.clone() ],
                [ bp.clone(), bp.clone(), bp.clone(), bp.clone(), bp.clone(), bp.clone(), bp.clone(), bp.clone() ],
                [ no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone() ],
                [ no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone() ],
                [ no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone() ],
                [ no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone(), no.clone() ],
                [ wp.clone(), wp.clone(), wp.clone(), wp.clone(), wp.clone(), wp.clone(), wp.clone(), wp.clone() ],
                [ wr.clone(), wn.clone(), wb.clone(), wq.clone(), wk.clone(), wb.clone(), wn.clone(), wr.clone() ]
            ]
        }
    }

}


impl Display for ChessBoard {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
"     {}   {}   {}   {}   {}   {}   {}   {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
 {} | {} | {} | {} | {} | {} | {} | {} | {} | {}
     {}   {}   {}   {}   {}   {}   {}   {}    ",
            Self::get_file_label(0),
            Self::get_file_label(1),
            Self::get_file_label(2),
            Self::get_file_label(3),
            Self::get_file_label(4),
            Self::get_file_label(5),
            Self::get_file_label(6),
            Self::get_file_label(7),

            Self::get_rank_label(0),
            self.borrow_space_contents(0, 0),
            self.borrow_space_contents(0, 1),
            self.borrow_space_contents(0, 2),
            self.borrow_space_contents(0, 3),
            self.borrow_space_contents(0, 4),
            self.borrow_space_contents(0, 5),
            self.borrow_space_contents(0, 6),
            self.borrow_space_contents(0, 7),
            Self::get_rank_label(0),

            Self::get_rank_label(1),
            self.borrow_space_contents(1, 0),
            self.borrow_space_contents(1, 1),
            self.borrow_space_contents(1, 2),
            self.borrow_space_contents(1, 3),
            self.borrow_space_contents(1, 4),
            self.borrow_space_contents(1, 5),
            self.borrow_space_contents(1, 6),
            self.borrow_space_contents(1, 7),
            Self::get_rank_label(1),

            Self::get_rank_label(2),
            self.borrow_space_contents(2, 0),
            self.borrow_space_contents(2, 1),
            self.borrow_space_contents(2, 2),
            self.borrow_space_contents(2, 3),
            self.borrow_space_contents(2, 4),
            self.borrow_space_contents(2, 5),
            self.borrow_space_contents(2, 6),
            self.borrow_space_contents(2, 7),
            Self::get_rank_label(2),

            Self::get_rank_label(3),
            self.borrow_space_contents(3, 0),
            self.borrow_space_contents(3, 1),
            self.borrow_space_contents(3, 2),
            self.borrow_space_contents(3, 3),
            self.borrow_space_contents(3, 4),
            self.borrow_space_contents(3, 5),
            self.borrow_space_contents(3, 6),
            self.borrow_space_contents(3, 7),
            Self::get_rank_label(3),

            Self::get_rank_label(4),
            self.borrow_space_contents(4, 0),
            self.borrow_space_contents(4, 1),
            self.borrow_space_contents(4, 2),
            self.borrow_space_contents(4, 3),
            self.borrow_space_contents(4, 4),
            self.borrow_space_contents(4, 5),
            self.borrow_space_contents(4, 6),
            self.borrow_space_contents(4, 7),
            Self::get_rank_label(4),

            Self::get_rank_label(5),
            self.borrow_space_contents(5, 0),
            self.borrow_space_contents(5, 1),
            self.borrow_space_contents(5, 2),
            self.borrow_space_contents(5, 3),
            self.borrow_space_contents(5, 4),
            self.borrow_space_contents(5, 5),
            self.borrow_space_contents(5, 6),
            self.borrow_space_contents(5, 7),
            Self::get_rank_label(5),

            Self::get_rank_label(6),
            self.borrow_space_contents(6, 0),
            self.borrow_space_contents(6, 1),
            self.borrow_space_contents(6, 2),
            self.borrow_space_contents(6, 3),
            self.borrow_space_contents(6, 4),
            self.borrow_space_contents(6, 5),
            self.borrow_space_contents(6, 6),
            self.borrow_space_contents(6, 7),
            Self::get_rank_label(6),

            Self::get_rank_label(7),
            self.borrow_space_contents(7, 0),
            self.borrow_space_contents(7, 1),
            self.borrow_space_contents(7, 2),
            self.borrow_space_contents(7, 3),
            self.borrow_space_contents(7, 4),
            self.borrow_space_contents(7, 5),
            self.borrow_space_contents(7, 6),
            self.borrow_space_contents(7, 7),
            Self::get_rank_label(7),

            Self::get_file_label(0),
            Self::get_file_label(1),
            Self::get_file_label(2),
            Self::get_file_label(3),
            Self::get_file_label(4),
            Self::get_file_label(5),
            Self::get_file_label(6),
            Self::get_file_label(7)
        )
    }

}


impl ChessBoard {

    pub fn get_rank_label(rank: u8) -> String {
        if rank < 8 {
            (8 - rank).to_string()
        } else {
            log(
                "ERROR",
                format!(
                    "crate::components::chess_board::ChessBoard::get_rank_label received the invalid rank number {}",
                    rank
                )
            );
            panic!("Invalid rank number: {}", rank);
        }
    }
    
    pub fn get_rank(rank_label: String) -> u8 {
        let parsed_label = rank_label.parse::<u8>();
        match parsed_label {
            Ok(_) => {
                let rank = 8 - parsed_label.unwrap();
                if rank < 8 {
                    rank
                } else {
                    log(
                        "ERROR",
                        format!(
                            "crate::components::chess_board::ChessBoard::get_rank received the invalid rank label \"{}\"",
                            rank_label
                        )
                    );
                    panic!("Invalid rank label: {}", rank_label);
                }
            },
            Err(_) => {
                log(
                    "ERROR",
                    format!(
                        "crate::components::chess_board::ChessBoard::get_rank received the invalid rank label \"{}\"",
                        rank_label
                    )
                );
                panic!("Invalid rank label: {}", rank_label);
            }
        }
    }

    pub fn get_file_label(file: u8) -> String {
        String::from(
            match file {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => {
                    log(
                        "ERROR",
                        format!(
                            "crate::components::chess_board::ChessBoard::get_file_label received the invalid file number {}",
                            file
                        )
                    );
                    panic!("Invalid file number: {}", file);
                }
            }
        )
    }

    pub fn get_file(file_label: String) -> u8 {
        if      file_label == "a" { 0 }
        else if file_label == "b" { 1 }
        else if file_label == "c" { 2 }
        else if file_label == "d" { 3 }
        else if file_label == "e" { 4 }
        else if file_label == "f" { 5 }
        else if file_label == "g" { 6 }
        else if file_label == "h" { 7 }
        else {
            log(
                "ERROR",
                format!(
                    "crate::components::chess_board::ChessBoard::get_file_label received the invalid file label {}",
                    file_label
                )
            );
            panic!("Invalid file label {}", file_label);
        }
    }

}


impl ChessBoard {

    pub fn borrow_space_contents(&self, rank: u8, file: u8) -> &ChessPiece {
        &(self.contents[rank as usize][file as usize])
    }

    fn borrow_mut_space_contents(&mut self, rank: u8, file: u8) -> &mut ChessPiece {
        &mut (self.contents[rank as usize][file as usize])
    }

    fn clone_space_contents(&self, rank: u8, file: u8) -> ChessPiece {
        self.contents[rank as usize][file as usize].clone()
    }

    // fn space_is_empty(&self, rank: u8, file: u8) -> bool {
    //     self.contents[rank as usize][file as usize].is_empty()
    // }

    fn set_space_contents(&mut self, rank: u8, file: u8, contents:ChessPiece) {
        self.contents[rank as usize][file as usize] = contents;
    }

    fn is_valid_move(&self, start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, piece: &ChessPiece, last_move: &Option<((u8, u8), (u8, u8))>) -> bool {
        let _type = piece.get_type();
        if      _type == "none"   { false }
        else if _type == "pawn"   {   pawn::is_valid_move(start_rank, start_file, target_rank, target_file, &self, last_move) }
        else if _type == "rook"   {   rook::is_valid_move(start_rank, start_file, target_rank, target_file, &self) }
        else if _type == "knight" { knight::is_valid_move(start_rank, start_file, target_rank, target_file, &self) }
        else if _type == "bishop" { bishop::is_valid_move(start_rank, start_file, target_rank, target_file, &self) }
        else if _type == "queen"  {  queen::is_valid_move(start_rank, start_file, target_rank, target_file, &self) }
        else if _type == "king"   {   king::is_valid_move(start_rank, start_file, target_rank, target_file, &self) }
        else {
            log(
                "ERROR",
                format!(
                    "crate::components::chess_board::ChessBoard::is_valid_move received a ChessPiece with the invalid type \"{}\".",
                    _type
                )
            );
            panic!("Invalid piece type: {}", _type);
        }
    }

    pub fn move_piece(&mut self, is_black_turn: bool, start_rank: u8, start_file: u8, target_rank: u8, target_file: u8, last_move: &mut Option<((u8, u8), (u8, u8))>) -> bool {
        if self.borrow_space_contents(start_rank, start_file).is_black() != is_black_turn {
            return false;
        }
        if self.is_valid_move(
            start_rank, start_file,
            target_rank, target_file,
            self.borrow_space_contents(start_rank, start_file),
            last_move,
        ) {
            log(
                "INFO",
                format!(
                    "Performing move {}{} -> {}{}",
                    Self::get_file_label(start_file),
                    Self::get_rank_label(start_rank),
                    Self::get_file_label(target_file),
                    Self::get_rank_label(target_rank)
                )
            );

            // Check for en passant and remove the attacked pawn
            if self.borrow_space_contents(start_rank, start_file).is_pawn()
            && self.borrow_space_contents(target_rank, target_file).is_empty()
            && start_file != target_file {
                log("INFO", "Detected en passant.");
                if self.borrow_space_contents(start_rank, start_file).is_white() {
                    self.set_space_contents(target_rank + 1, target_file, ChessPiece::default());
                } else {
                    self.set_space_contents(target_rank - 1, target_file, ChessPiece::default());
                }
            }

            // Check for castle and move the associated rook
            if self.borrow_space_contents(start_rank, start_file).is_king()
            && (start_file as i8 - target_file as i8) == 2 {
                log("INFO", "Detected castle.");
                if target_file < start_file {
                    log("INFO", "Detected queenside castle.");
                    let rook_file = ChessBoard::get_file(String::from("a"));
                    self.set_space_contents(start_rank, target_file + 1, self.clone_space_contents(start_rank, rook_file));
                    self.set_space_contents(start_rank, rook_file, ChessPiece::default());
                } else {
                    log("INFO", "Detected kingside castle.");
                    let rook_file = ChessBoard::get_file(String::from("h"));
                    self.set_space_contents(start_rank, target_file - 1, self.clone_space_contents(start_rank, rook_file));
                    self.set_space_contents(start_rank, rook_file, ChessPiece::default());
                }
            }

            // Move piece (typical updates)
            self.set_space_contents(target_rank, target_file, self.clone_space_contents(start_rank, start_file));
            self.set_space_contents(start_rank, start_file, ChessPiece::default());
            self.borrow_mut_space_contents(target_rank, target_file).mark_moved();
            *last_move = Some(((start_file, start_rank), (target_file, target_rank)));

            // TODO: check for promotion here (once implemented)

            return true;
        }
        log(
            "INFO",
            format!(
                "Skipped move {}{} -> {}{} due to it being invalid.",
                Self::get_file_label(start_file),
                Self::get_rank_label(start_rank),
                Self::get_file_label(target_file),
                Self::get_rank_label(target_rank)
            )
        );
        return false;
    }

}
