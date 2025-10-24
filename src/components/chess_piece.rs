// crate::components::chess_piece

use std::fmt::Display;

use crate::utils::logs::log;


#[derive(Clone)]


pub struct ChessPiece {

    is_black: bool,
    piece_type: u8,
    has_moved: bool,

}


impl Default for ChessPiece {

    fn default() -> Self {
        Self {
            is_black: false,
            piece_type: Self::get_type_num(String::from("none")),
            has_moved: false,
        }
    }

}


impl Display for ChessPiece {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() { write!(f, "-") }
        else {
            let color = self.get_color();
            let _type = self.get_type();
            if color == "black" {
                if      _type == "pawn"   { write!(f, "p") }
                else if _type == "rook"   { write!(f, "r") }
                else if _type == "knight" { write!(f, "n") }
                else if _type == "bishop" { write!(f, "b") }
                else if _type == "queen"  { write!(f, "q") }
                else if _type == "king"   { write!(f, "k") }
                else {
                    log(
                        "ERROR",
                        format!(
                            "crate::chess_piece::ChessPiece::fmt used Self::get_type and got the invalid piece type \"{}\"",
                            _type
                        )
                    );
                    panic!("Invalid piece type: {}", _type)
                }
            } else if color == "white" {
                if      _type == "pawn"   { write!(f, "P") }
                else if _type == "rook"   { write!(f, "R") }
                else if _type == "knight" { write!(f, "N") }
                else if _type == "bishop" { write!(f, "B") }
                else if _type == "queen"  { write!(f, "Q") }
                else if _type == "king"   { write!(f, "K") }
                else {
                    log(
                        "ERROR",
                        format!(
                            "crate::chess_piece::ChessPiece::fmt used Self::get_type and got the invalid piece type \"{}\"",
                            _type
                        )
                    );
                    panic!("Invalid piece type: {}", _type)
                }
            } else {
                log(
                    "ERROR",
                    format!(
                        "crate::chess_piece::ChessPiece::fmt used Self::get_color and got the invalid piece color \"{}\"",
                        color
                    )
                );
                panic!("Invalid color: {}", color);
            }
        }
    }

}


impl ChessPiece {

    pub fn construct(color: String, piece_type: String) -> Self {
        if !( color == "black" || color == "white" ) {
            log(
                "ERROR",
                format!(
                    "crate::chess_piece::ChessPiece::construct received the invalid piece color \"{}\"",
                    color
                )
            );
            panic!("Invalid color: {} (expected \"white\" or \"black\")", color);
        }
        Self {
            is_black: color == "black",
            piece_type: Self::get_type_num(piece_type),
            has_moved: false,
        }
    }

    fn get_type_num(type_label: String) -> u8 {
        if      type_label == "none" { 0 }
        else if type_label == "pawn" { 1 }
        else if type_label == "rook" { 2 }
        else if type_label == "knight" { 3 }
        else if type_label == "bishop" { 4 }
        else if type_label == "queen" { 5 }
        else if type_label == "king" { 6 }
        else {
            log(
                "ERROR",
                format!(
                    "crate::chess_piece::ChessPiece::get_type_num received the invalid type label \"{}\"",
                    type_label
                )
            );
            panic!("Invalid type label: {}", type_label);
        }
    }

    fn get_type_label(type_num: u8) -> String {
        String::from(
            match type_num {
                0 => "none",
                1 => "pawn",
                2 => "rook",
                3 => "knight",
                4 => "bishop",
                5 => "queen",
                6 => "king",
                _ => {
                    log(
                        "ERROR",
                        format!(
                            "crate::chess_piece::ChessPiece::get_type_label received the invalid type number {}",
                            type_num
                        )
                    );
                    panic!("Invalid type num: {}", type_num);
                }
            }
        )
    }

}


impl ChessPiece {

    pub fn get_color(&self) -> String {
        if self.is_empty() {
            String::from("none")
        } else if self.is_black {
            String::from("black")
        } else {
            String::from("white")
        }
    }

    pub fn is_black(&self) -> bool {
        self.is_black
    }

    pub fn is_white(&self) -> bool {
        !(self.is_black) && !(self.is_empty())
    }

    pub fn get_type(&self) -> String {
        Self::get_type_label(self.piece_type)
    }

    pub fn is_empty(&self) -> bool {
        self.get_type() == "none"
    }

    pub fn is_pawn(&self) -> bool {
        self.get_type() == "pawn"
    }

    pub fn is_rook(&self) -> bool {
        self.get_type() == "rook"
    }

    // fn is_knight(&self) -> bool {
    //     self.get_type() == "knight"
    // }

    // fn is_bishop(&self) -> bool {
    //     self.get_type() == "bishop"
    // }

    // fn is_queen(&self) -> bool {
    //     self.get_type() == "queen"
    // }

    pub fn is_king(&self) -> bool {
        self.get_type() == "king"
    }

    pub fn mark_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

}
