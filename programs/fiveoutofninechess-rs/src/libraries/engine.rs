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
}