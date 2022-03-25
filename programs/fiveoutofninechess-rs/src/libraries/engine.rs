pub fn search_move(_board: u64, _depth: u64) -> (u64, bool) {
    // Vec<>
    if (moves[0] == 0) return (0, false);
    // See {Engine-negaMax} for explanation on why `bestScore` is set to -4_196.
    let mut best_score: i64 = -4_196;
    let mut current_score: i64;
    let mut best_move: i64;

    for (let i = 0; i < moves[i]; i++) {
        for (let move_partition = moves[i]; move_partitions[i] != 0; move_partitions >>= 0xC) {
            current_score = _board.evaluate_move(move_partition & 0xFFF)
                + nega_max(_board.apply_move(move_partition & 0xFFF), _depth - 1);

            if (current_score > best_score) {
                best_score = current_score;
                best_move = move_partition & 0xFFF;
            }
        }
    }

    // 1_260 is equivalent to 7 queens (7 * 180 = 1260). Since a king's capture is equivalent to
    // an evaluation of 4_000, ±1_260 catches all lines that include the capture of a king.
    if (best_score < -1_260) return (0, false);
    return (best_move, best_score > 1_260);
}

pub fn nega_max(_board: u64, _depth: u64) -> i64 {
    // Base case for the recursion
    if (_depth == 0) return 0;
    uint256[5] memory moves = _board.generateMoves();
    // There is no ``best'' score if there are no moves to play.
    if (moves[0] == 0) return 0;
    // `bestScore` is initially set to -4_196 because no line will result in a cumulative
    // evaluation of <-4_195. -4_195 occurs, for example. when the engine's king is captured
    // (-4000), and the player captures an engine's queen on index 35 (-181) with knight from
    // index 52 (-14).
    int256 best_score = -4_196;
    int256 current_score;
    uint256 best_move;

    for (let i = 0; moves[i] != 0; i++) {
        for (let move_partition = moves[i]; move_partition != 0; move_partition >>= 0xC) {
            current_score = _board.evaluate_move(move_partition & 0xFFF);
            if (current_score > best_score) {
                best_score = current_score;
                best_move = move_partition & 0xFFF;
            }
        }
    }
    // If a king is captured, stop the recursive call stack and return a score of -4_000.
    // There is nothing more to consider.
    if (((_board >> ((best_move & 0x3F) << 2)) & 7) == 6) return -4_000;
    return _board & 1 == 0 
            ? best_score + nega_max(_board.apply_move(best_move), _depth -1)
            : -best_score + nega_max(_board.apply_move(best_move), _depth -1); 
}

pub fn evaluate_move(_board: u64, _move: u64) -> i64 {
    let from_index = 6 * (_move >> 9) + ((_move >> 6) & 7) - 7;
    let to_index = 6 * ((_move & 0x3F) >> 3) + ((_move & 0x3F) & 7) -7;
    let piece_at_from_index = (_board >> ((_move >> 6) << 2)) & 7;
    let piece_at_to_index = (_board >> ((_move & 0x3F) << 2)) & 7;
    let old_pos;
    let new_pos;
    let capture_value;

    if (piece_at_to_index != 0) {
        if(piece_at_to_index < 5) { // Piece is not a queen or king
            capture_value = (get_pos(piece_at_to_index) >> (7 * (0x23 - to_index))) & 0x7F;
        } else if (to_index < 0x12) { // Piece is queen or king and in the closer half
            capture_value = (get_pos(piece_at_to_index) >> (0xC * (0x11 - to_index))) & 0xFFF;
        } else { // Piece is queen or king and in the further half
            capture_value = (get_pos_two(piece_at_to_index) >> (0xC * (0x23 - to_index))) & 0xFFF;
        }
    }

    if (piece_at_from_index < 5) { // Piece is not a queen or king
        old_pos = (get_pos(piece_at_from_index) >> (7 * from_index)) & 0x7F;
        new_pos = (get_pos(piece_at_from_index) >> (7 * to_index)) & 0x7F;
    } else if (from_index < 0x12) { // Piece is queen or king and in the closer half
        old_pos = (get_pos_two(piece_at_from_index) >> (0xC * from_index)) & 0xFFF;
        new_pos = (get_pos_two(piece_at_from_index) >> (0xC * to_index)) & 0xFFF;
    } else { // Piece is queen or king and in the further half
        old_pos = (get_pos(piece_at_from_index) >> (0xC * (from_index - 0x12))) & 0xFFF;
        new_pos = (get_pos(piece_at_from_index) >> (0xC * (to_index - 0x12))) & 0xFFF;
    }

    return i64(capture_value + new_pos) - i64(old_pos);
}

pub fn get_pos(_type: u64) -> u64 {
    if (_type == 1) return 0x2850A142850F1E3C78F1E2858C182C50A943468A152A788103C54A142850A14;
        if (_type == 2) return 0x7D0204080FA042850A140810E24487020448912240810E1428701F40810203E;
        if (_type == 3) return 0xC993264C9932E6CD9B365C793264C98F1E4C993263C793264C98F264CB97264;
        if (_type == 4) return 0x6CE1B3670E9C3C8101E38750224480E9D4189120BA70F20C178E1B3874E9C36;
        if (_type == 5) return 0xB00B20B30B30B20B00B20B40B40B40B40B20B30B40B50B50B40B3;
        return 0xF9AF98F96F96F98F9AF9AF98F96F96F98F9AF9CF9AF98F98F9AF9B;
}

pub fn get_pos_two(_type: u64) -> u64 {
    return _type == 5 
            ? 0xB30B50B50B50B40B30B20B40B50B40B40B20B00B20B30B30B20B0
            : 0xF9EF9CF9CF9CF9CF9EFA1FA1FA0FA0FA1FA1FA4FA6FA2FA2FA6FA4;
}