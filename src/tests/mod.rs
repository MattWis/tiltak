#[cfg(test)]
mod board_tests;
#[cfg(test)]
mod mcts_tests;
#[cfg(test)]
mod move_gen_5s_tests;
#[cfg(test)]
mod move_gen_generic_tests;
#[cfg(test)]
mod tactics_test;

#[cfg(test)]
use crate::board::Board;
#[cfg(test)]
use board_game_traits::board::Board as BoardTrait;
#[cfg(test)]
use pgn_traits::pgn::PgnBoard;

#[cfg(test)]
fn do_moves_and_check_validity<const S: usize>(board: &mut Board<S>, move_strings: &[&str]) {
    let mut moves = vec![];
    for mv_san in move_strings.iter() {
        let mv = board.move_from_san(&mv_san).unwrap();
        board.generate_moves(&mut moves);
        assert!(
            moves.contains(&mv),
            "Move {} was not among legal moves: {:?}\n{:?}",
            board.move_to_san(&mv),
            moves,
            board
        );
        board.do_move(mv);
        moves.clear();
    }
}
