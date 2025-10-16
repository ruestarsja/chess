// crate::components::chess_board

use crate::components::chess_piece::ChessPiece;

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
            " {} | {} | {} | {} | {} | {} | {} | {} \n {} | {} | {} | {} | {} | {} | {} | {} \n {} | {} | {} | {} | {} | {} | {} | {} \n {} | {} | {} | {} | {} | {} | {} | {} \n {} | {} | {} | {} | {} | {} | {} | {} \n {} | {} | {} | {} | {} | {} | {} | {} \n {} | {} | {} | {} | {} | {} | {} | {} \n {} | {} | {} | {} | {} | {} | {} | {} ",
            self.borrow_space_contents(0, 0),
            self.borrow_space_contents(0, 1),
            self.borrow_space_contents(0, 2),
            self.borrow_space_contents(0, 3),
            self.borrow_space_contents(0, 4),
            self.borrow_space_contents(0, 5),
            self.borrow_space_contents(0, 6),
            self.borrow_space_contents(0, 7),
            self.borrow_space_contents(1, 0),
            self.borrow_space_contents(1, 1),
            self.borrow_space_contents(1, 2),
            self.borrow_space_contents(1, 3),
            self.borrow_space_contents(1, 4),
            self.borrow_space_contents(1, 5),
            self.borrow_space_contents(1, 6),
            self.borrow_space_contents(1, 7),
            self.borrow_space_contents(2, 0),
            self.borrow_space_contents(2, 1),
            self.borrow_space_contents(2, 2),
            self.borrow_space_contents(2, 3),
            self.borrow_space_contents(2, 4),
            self.borrow_space_contents(2, 5),
            self.borrow_space_contents(2, 6),
            self.borrow_space_contents(2, 7),
            self.borrow_space_contents(3, 0),
            self.borrow_space_contents(3, 1),
            self.borrow_space_contents(3, 2),
            self.borrow_space_contents(3, 3),
            self.borrow_space_contents(3, 4),
            self.borrow_space_contents(3, 5),
            self.borrow_space_contents(3, 6),
            self.borrow_space_contents(3, 7),
            self.borrow_space_contents(4, 0),
            self.borrow_space_contents(4, 1),
            self.borrow_space_contents(4, 2),
            self.borrow_space_contents(4, 3),
            self.borrow_space_contents(4, 4),
            self.borrow_space_contents(4, 5),
            self.borrow_space_contents(4, 6),
            self.borrow_space_contents(4, 7),
            self.borrow_space_contents(5, 0),
            self.borrow_space_contents(5, 1),
            self.borrow_space_contents(5, 2),
            self.borrow_space_contents(5, 3),
            self.borrow_space_contents(5, 4),
            self.borrow_space_contents(5, 5),
            self.borrow_space_contents(5, 6),
            self.borrow_space_contents(5, 7),
            self.borrow_space_contents(6, 0),
            self.borrow_space_contents(6, 1),
            self.borrow_space_contents(6, 2),
            self.borrow_space_contents(6, 3),
            self.borrow_space_contents(6, 4),
            self.borrow_space_contents(6, 5),
            self.borrow_space_contents(6, 6),
            self.borrow_space_contents(6, 7),
            self.borrow_space_contents(7, 0),
            self.borrow_space_contents(7, 1),
            self.borrow_space_contents(7, 2),
            self.borrow_space_contents(7, 3),
            self.borrow_space_contents(7, 4),
            self.borrow_space_contents(7, 5),
            self.borrow_space_contents(7, 6),
            self.borrow_space_contents(7, 7),
        )
    }

}


impl ChessBoard {

    // fn get_rank_label(rank: u8) -> String {
    //     if rank < 8 {
    //         (8 - rank).to_string()
    //     } else {
    //         panic!("Invalid rank number: {}", rank);
    //     }
    // }
    
    // fn get_rank(rank_label: String) -> u8 {
    //     let parsed_label = rank_label.parse::<u8>();
    //     match parsed_label {
    //         Ok(_) => {
    //             let rank = 8 - parsed_label.unwrap();
    //             if rank < 8 {
    //                 rank
    //             } else {
    //                 panic!("Invalid rank label: {}", rank_label);
    //             }
    //         },
    //         Err(_) => {
    //             panic!("Invalid rank label: {}", rank_label);
    //         }
    //     }
    // }

    // fn get_file_label(file: u8) -> String {
    //     String::from(
    //         match file {
    //             0 => 'a',
    //             1 => 'b',
    //             2 => 'c',
    //             3 => 'd',
    //             4 => 'e',
    //             5 => 'f',
    //             6 => 'g',
    //             7 => 'h',
    //             _ => {
    //                 panic!("Invalid file number: {}", file);
    //             }
    //         }
    //     )
    // }

    // fn get_file(file_label: String) -> u8 {
    //     if file_label == "a" { 0 }
    //     else if file_label == "b" { 1 }
    //     else if file_label == "c" { 2 }
    //     else if file_label == "d" { 3 }
    //     else if file_label == "e" { 4 }
    //     else if file_label == "f" { 5 }
    //     else if file_label == "g" { 6 }
    //     else if file_label == "h" { 7 }
    //     else {
    //         panic!("Invalid file label {}", file_label);
    //     }
    // }

}


impl ChessBoard {

    pub fn borrow_space_contents(&self, rank: u8, file: u8) -> &ChessPiece {
        &(self.contents[rank as usize][file as usize])
    }

    // fn borrow_mut_space_contents(&mut self, rank: u8, file: u8) -> &mut ChessPiece {
    //     &mut (self.contents[rank as usize][file as usize])
    // }

    // fn clone_space_contents(&self, rank: u8, file: u8) -> ChessPiece {
    //     self.contents[rank as usize][file as usize].clone()
    // }

    // fn space_is_empty(&self, rank: u8, file: u8) -> bool {
    //     self.contents[rank as usize][file as usize].is_empty()
    // }

    // fn set_space_contents(&mut self, rank: u8, file: u8, contents:ChessPiece) {
    //     self.contents[rank as usize][file as usize] = contents;
    // }

}
