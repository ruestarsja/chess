// crate::rules::knight

const POSSIBLE_MOVES: [[i8; 2]; 8] = [
    [2, 1],
    [1, 2],
    [-1, 2],
    [-2, 1],
    [-2, -1],
    [-1, -2],
    [1, -2],
    [2, -1]
];

pub fn is_valid_move(start_rank: u8, start_file: u8, target_rank: u8, target_file: u8) -> bool {
    if start_rank >= 8 || start_file >= 8 {
        panic!("Invalid starting rank or file: rank {}, file {}", start_rank, start_file);
    }
    if target_rank >= 8 || target_file >= 8 {
        return false;
    }
    let possible_move: [i8; 2] = [(target_rank as i8) - (start_rank as i8), (target_file as i8) - (start_file as i8)];
    if POSSIBLE_MOVES.contains(&possible_move) {
        return true;
    } else {
        return false;
    }
}

// fn get_valid_moves(start_rank: u8, start_file: u8) -> Vec<[u8; 2]> {
//     if start_rank >= 8 || start_file >= 8 {
//         panic!("Invalid starting rank or file: rank {}, file {}", start_rank, start_file);
//     }
//     let mut moves: Vec<[u8; 2]> = vec![];
//     for possible_move in POSSIBLE_MOVES {
//         let end_rank: i8 = (start_rank as i8) + possible_move[0];
//         let end_file: i8 = (start_file as i8) + possible_move[0];
//         if (0..8).contains(&end_rank) && (0..8).contains(&end_file) {
//             moves.push([end_rank as u8, end_file as u8]);
//         }
//     }
//     return moves;
// }
