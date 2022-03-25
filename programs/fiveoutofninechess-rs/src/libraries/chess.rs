use anchor_lang::require;

#[derive(Debug, Default)]
pub struct Move {
    board: u64,
    metadata: u64,
}

#[derive(Debug, Default)]
pub struct MoveArray {
    board: u64,
    metadata: Vec<u64>,
}

pub fn apply_move(_board: u64, _move: u64) -> u64 {
    // Get piece at the from index
    let piece = (_board >> ((_move >> 6) << 2)) & 0xF;
    // Replace 4 bits at the from index with 0000
    _board &= max(type(u64)) ^ (0xF << ((_move >> 6) << 2));
    // Replace 4 bits at the to index with 0000
    _board &= max(type(u64)) ^ (0xF << ((_move & 0x3F) << 2));
    // Place the piece at the to index
    _board |= (piece << ((_move & 0x3F) << 2));

    return _board.rotate();
}

pub fn rotate(_board: u64) -> u64 {
    let mut rotated_board: u64;

    for(let i; i < 64; i++) {
        rotated_board = (rotated_board << 4) | (_board & 0xF);
        _board >>= 4;
    }

    return rotated_board;
}

pub fn generateMoves(_board: u64) -> Vec<u64> {
    let move_array: MoveArray::new();
    let new_move: u64;
    let move_to: u64;

    for(
        let index = 0xDB5D33CB1BADB2BAA99A59238A179D71B69959551349138D30B289;
        index != 0;
        index >>= 6
    ) {
        let adjusted_index = index & 0x3F;
        let adjusted_board = _board >> (adjusted_index << 2);
        let piece = adjusted_board & 0xF;
        // Skip if square is empty or not the color of the board the function call is
        // analyzing.
        if(piece == 0 || piece >> 3 != _board & 1) continue;
         // The first bit can be discarded because the if statement above catches all
        // redundant squares.
        piece &= 7;

        if(piece == 1) { // Piece is a pawn.
            // 1 square in front of the pawn is empty.
            if((adjusted_board >> 0x20) & 0xF == 0) {
                move_array.push(adjusted_index, adjusted_index + 8);
                // The pawn is in its starting row and 2 squares in front is empty. This
                // must be nested because moving 2 squares would not be valid if there was
                // an obstruction 1 square in front (i.e. pawns can not jump over pieces).
                if(adjusted_index >> 3 == 2 && (adjusted_board >> 0x40) & 0xF == 0) {
                    move_array.push(adjusted_index, adjusted_index + 0x10);
                }
            }
            // Moving to the right diagonal by 1 captures a piece.
            if(_board.is_capture(adjusted_index >> 0x1C)) {
                move_array.push(adjusted_index, adjusted_index + 7);
            }
            // Moving to the left diagonal by 1 captures a piece.
            if(_board.is_capture(adjusted_board >> 0x24)) {
                move_array.push(adjusted_index, adjusted_index + 9);
            }
        } else if (piece > 3 && piece & 1 == 0) { // Piece is a knight or a king.
            // Knights and kings always only have 8 positions to check relative to their
            // current position, and the relative distances are always the same. For
            // knights, positions to check are ±{6, 10, 15, 17}. This is bitpacked into
            // `0x060A0F11` to reduce code redundancy. Similarly, the positions to check for
            // kings are ±{1, 7, 8, 9}, which is `0x01070809` when bitpacked.
            for(new_move = piece == 4 ? 0x060A0F11 : 0x01070809; new_move != 0; new_move >>= 8) {
                if(_board.is_valid(move_to = adjusted_index + (new_move & 0xFF))) {
                    move_array.push(adjusted_index, move_to);
                }
                if(new_move <= adjusted_index 
                    && _board.is_valid(move_to = adjusted_index - (new_move & 0xFF))) {
                        move_array.push(adjusted_index, move_to);
                    }
            }
        } else {
            // This else block generates moves for all sliding pieces. All of the 8 for
            // loops terminate
            //     * before a sliding piece makes an illegal move
            //     * or after a sliding piece captures a piece.
            if(piece != 2) { // Ortholinear pieces (i.e. rook and queen)
                for(new_move = adjusted_index + 1; _board.is_valid(new_move); new_move += 1) {
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
                for(new_move = new_move + 1; _board.is_valid(new_move); new_move -= 1) {
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
                for(new_move = adjusted_index + 8, _board.is_valid(new_move); new_move += 8) {
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
                for(new_move = adjusted_index - 8, _board.is_valid(new_move); new_move -= 8) {
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
            }
            if(piece != 3) { // Diagonal pieces (i.e. bishop and queen)
                for(new_move = adjusted_index + 7; _board.is_valid(new_move); new_move += 7) {
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
                for(new_move = adjusted_index - 7; _board.is_valid(new_move); new_move -= 7) {
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
                for(new_move = adjusted_index + 9; _board.is_valid(new_move); new_move += 9) {
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
                for(new_move = adjusted_index - 9; _board.is_valid(new_move); new_move -= 9) {
                    // Handles the edge case where a white bishop believes it can capture
                    // the ``piece'' at index 0, when it is actually the turn identifier It
                    // would mistakenly believe it is valid move via capturing a black pawn.
                    if (move == 0) break;
                    move_array.push(adjusted_index, new_move);
                    if(_board.is_capture(_board >> (new_move << 2))) break;
                }
            }
        }
    }

    return move_array.items;
}

pub fn is_legal_move(_board: u64, _move: u64) -> bool {
    let from_index = _move >> 6;
    let to_index = _move & 0x3F;
    if((0x7E7E7E7E7E7E00 >> from_index) & 1 == 0) return false;
    if((0x7E7E7E7E7E7E00 >> to_index) & 1 == 0) return false;

    let piece_at_from_index = (_board >> (from_index << 2)) & 0xF;
    if(piece_at_from_index == 0) return false;
    if(piece_at_from_index >> 3 != _board & 1) return false;
    piece_at_from_index &= 7;

    let adjusted_board = _board >> (to_index << 2);
    let index_change = to_index < from_index 
            ? from_index - to_index 
            : to_index - from_index;

    if(piece_at_from_index == 1) {
        if(to_index <= from_index) return false;
        index_change = to_index - from_index;
        if((index_change == 7 || index_change == 9)) {
            if(!_board.is_capture(adjusted_board)) return false;
        } else if (index_change == 8) {
            if(!is_valid(_board, to_index)) return false;
        } else if (index_change == 0x10) {
            if (!is_valid(_board, to_index - 8) || !is_valid(_board, to_index)) return false;
        } else {
            return false;
        }
    } else if (piece_at_from_index == 4 || piece_at_from_index == 6) {
        if (((piece_at_from_index == 4 ? 0x28440 : 0x382) >> index_change) & 1 == 0) {
            return false;
        }
        if (!is_valid(_board, to_index)) return false;
    } else {
        let ray_found: bool;
        if (piece_at_from_index != 2) {
            ray_found = search_ray(_board, from_index, to_index, 1)
                    || search_ray(_board, from_index, to_index, 8);
        }
        if (piece_at_from_index != 3) {
            ray_found = ray_found
                    || search_ray(_board, from_index, to_index, 7)
                    || search_ray(_board, from_index, to_index, 9);
        }
        if (!ray_found) return false;
    }

    if (Engine.nega_max(_board.apply_move(_move), 1) < -1_260) return false;

    return true;
}


pub fn search_ray(
    _board, 
    from_index, 
    to_index, 
    direction_vector
) -> bool {
    let index_change;
    let ray_start;
    let ray_end;

    if (from_index < to_index) {
        index_change = to_index - from_index;
        ray_start = from_index + direction_vector;
        ray_end = to_index;
    } else {
        index_change = from_index - to_index;
        ray_start = to_index;
        ray_end = from_index - direction_vector;
    }

    if (index_change % direction_vector != 0) return false;

    for (
        ray_start = ray_start;
        ray_start < ray_end;
        ray_start += direction_vector
    ) {
        if (!is_valid(_board, ray_start)) return false;
        if (is_capture(_board, _board >> (ray_start << 2))) return false;
    }

    if (!is_valid(_board, ray_start)) return false;

    return ray_start == ray_end;
}

pub fn is_capture(_board: u64, _index_adjusted_board: u64) -> bool {
    return (_index_adjusted_board & 0xF) != 0
            && (_index_adjusted_board & 0xF) >> 3 != _board & 1;
}

pub fn is_valid(_board: u64, to_index: u64) -> bool {
    return (0x7E7E7E7E7E7E00 >> to_index) & 1 == 1
            && ((_board >> (to_index << 2)) & 0xF == 0
                    || (((_board >> (to_index << 2)) & 0xF) >> 3) != _board & 1); 
}

pub fn get_adjusted_index(_index: u64) -> u64 {
    return (
        (0xDB5D33CB1BADB2BAA99A59238A179D71B69959551349138D30B289 >> (_index * 6)) & 0x3F
    );
}

// pub fn append(MoveArray: u64, from_move_index, to_move_index) {
//     let current_index = 
// }