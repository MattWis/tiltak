use board_game_traits::{Color, Position as EvalPosition};

use crate::evaluation::parameters::ValueParameters;
use crate::position::bitboard::BitBoard;
use crate::position::color_trait::{BlackTr, ColorTr, WhiteTr};
use crate::position::{
    num_square_symmetries, square_symmetries, squares_iterator, GroupData, Piece::*, Position,
    Role::*, Square,
};

pub(crate) fn static_eval_game_phase<const S: usize>(
    position: &Position<S>,
    group_data: &GroupData<S>,
    value_params: &mut ValueParameters,
) {
    let mut white_flat_count = 0;
    let mut black_flat_count = 0;

    for square in squares_iterator::<S>() {
        let stack = &position[square];
        if let Some(piece) = position[square].top_stone() {
            let i = square.0 as usize;
            match piece {
                WhiteFlat => {
                    value_params.flat_psqt[square_symmetries::<S>()[i]] += 1.0;
                    white_flat_count += 1;
                }
                BlackFlat => {
                    value_params.flat_psqt[square_symmetries::<S>()[i]] -= 1.0;
                    black_flat_count += 1;
                }
                WhiteWall => value_params.wall_psqt[square_symmetries::<S>()[i]] += 1.0,
                BlackWall => value_params.wall_psqt[square_symmetries::<S>()[i]] -= 1.0,
                WhiteCap => value_params.cap_psqt[square_symmetries::<S>()[i]] += 1.0,
                BlackCap => value_params.cap_psqt[square_symmetries::<S>()[i]] -= 1.0,
            }
            if stack.height > 1 {
                let controlling_player = piece.color();
                let color_factor = piece.color().multiplier() as f32;
                for piece in stack.into_iter().take(stack.height as usize - 1) {
                    if piece.color() == controlling_player {
                        value_params.our_stack_psqt[square_symmetries::<S>()[i]] += color_factor;
                    } else {
                        value_params.their_stack_psqt[square_symmetries::<S>()[i]] -= color_factor;
                    }
                }
            }
        }
    }

    // Give the side to move a bonus/malus depending on flatstone lead
    let white_flatstone_lead = white_flat_count - black_flat_count;

    // Bonus/malus depending on the number of groups each side has
    let mut seen_groups = vec![false; S * S + 1]; // TODO: Can be an array with full const-generics
    seen_groups[0] = true;

    let number_of_groups = squares_iterator::<S>()
        .map(|square| {
            let group_id = group_data.groups[square] as usize;
            if !seen_groups[group_id] {
                seen_groups[group_id] = true;
                position[square].top_stone().unwrap().color().multiplier()
            } else {
                0
            }
        })
        .sum::<isize>() as f32;

    let opening_scale_factor = f32::min(
        f32::max((24.0 - position.half_moves_played() as f32) / 12.0, 0.0),
        1.0,
    );
    let endgame_scale_factor = f32::min(
        f32::max((position.half_moves_played() as f32 - 24.0) / 24.0, 0.0),
        1.0,
    );
    let middlegame_scale_factor = 1.0 - opening_scale_factor - endgame_scale_factor;

    debug_assert!(middlegame_scale_factor <= 1.0);
    debug_assert!(opening_scale_factor == 0.0 || endgame_scale_factor == 0.0);

    value_params.side_to_move[0] =
        position.side_to_move().multiplier() as f32 * opening_scale_factor;
    value_params.flatstone_lead[0] = white_flatstone_lead as f32 * opening_scale_factor;
    value_params.i_number_of_groups[0] = number_of_groups * opening_scale_factor;

    value_params.side_to_move[1] =
        position.side_to_move().multiplier() as f32 * middlegame_scale_factor;
    value_params.flatstone_lead[1] = white_flatstone_lead as f32 * middlegame_scale_factor;
    value_params.i_number_of_groups[1] = number_of_groups * middlegame_scale_factor;

    value_params.side_to_move[2] =
        position.side_to_move().multiplier() as f32 * endgame_scale_factor;
    value_params.flatstone_lead[2] = white_flatstone_lead as f32 * endgame_scale_factor;
    value_params.i_number_of_groups[2] = number_of_groups * endgame_scale_factor;

    for critical_square in group_data.critical_squares(Color::White) {
        critical_squares_eval::<WhiteTr, BlackTr, S>(position, critical_square, value_params);
    }

    for critical_square in group_data.critical_squares(Color::Black) {
        critical_squares_eval::<BlackTr, WhiteTr, S>(position, critical_square, value_params);
    }

    squares_iterator::<S>()
        .map(|sq| (sq, &position[sq]))
        .filter(|(_, stack)| stack.len() > 1)
        .for_each(|(square, stack)| {
            let top_stone = stack.top_stone().unwrap();
            let controlling_player = top_stone.color();
            let color_factor = top_stone.color().multiplier() as f32;

            // Extra bonus for having your capstone over your own piece
            if top_stone.role() == Cap
                && stack.get(stack.len() - 2).unwrap().color() == controlling_player
            {
                value_params.capstone_over_own_piece[0] += color_factor;
            }

            match top_stone.role() {
                Cap => value_params.capstone_on_stack[0] += color_factor,
                Flat => (),
                Wall => value_params.standing_stone_on_stack[0] += color_factor,
            }

            // Malus for them having stones next to our stack with flat stones on top
            for neighbour in square.neighbours::<S>() {
                if let Some(neighbour_top_stone) = position[neighbour].top_stone() {
                    if top_stone.role() == Flat && neighbour_top_stone.color() != controlling_player
                    {
                        match neighbour_top_stone.role() {
                            Flat => {
                                value_params.flat_stone_next_to_our_stack[0] +=
                                    color_factor * stack.len() as f32
                            }
                            Wall => {
                                value_params.standing_stone_next_to_our_stack[0] +=
                                    color_factor * stack.len() as f32
                            }
                            Cap => {
                                value_params.capstone_next_to_our_stack[0] +=
                                    color_factor * stack.len() as f32
                            }
                        }
                    }
                }
            }
        });

    let mut num_ranks_occupied_white = 0;
    let mut num_files_occupied_white = 0;
    let mut num_ranks_occupied_black = 0;
    let mut num_files_occupied_black = 0;

    for line in BitBoard::all_lines::<S>().iter() {
        line_score::<WhiteTr, BlackTr, S>(group_data, *line, value_params);
        line_score::<BlackTr, WhiteTr, S>(group_data, *line, value_params);
    }

    for i in 0..S as u8 {
        if !WhiteTr::road_stones(group_data).rank::<S>(i).is_empty() {
            num_ranks_occupied_white += 1;
        }
        if !BlackTr::road_stones(group_data).rank::<S>(i).is_empty() {
            num_ranks_occupied_black += 1;
        }
    }

    for i in 0..S as u8 {
        if !WhiteTr::road_stones(group_data).file::<S>(i).is_empty() {
            num_files_occupied_white += 1;
        }
        if !BlackTr::road_stones(group_data).file::<S>(i).is_empty() {
            num_files_occupied_black += 1;
        }
    }

    value_params.num_lines_occupied[num_ranks_occupied_white] += 1.0;
    value_params.num_lines_occupied[num_files_occupied_white] += 1.0;
    value_params.num_lines_occupied[num_ranks_occupied_black] -= 1.0;
    value_params.num_lines_occupied[num_files_occupied_black] -= 1.0;
}

/// Give bonus for our critical squares
fn critical_squares_eval<Us: ColorTr, Them: ColorTr, const S: usize>(
    position: &Position<S>,
    critical_square: Square,
    value_params: &mut ValueParameters,
) {
    let top_stone = position[critical_square].top_stone;
    if top_stone.is_none() {
        value_params.critical_squares[0] += Us::color().multiplier() as f32;
    } else if top_stone == Some(Us::wall_piece()) {
        value_params.critical_squares[1] += Us::color().multiplier() as f32;
    } else if top_stone == Some(Them::flat_piece()) {
        value_params.critical_squares[2] += Us::color().multiplier() as f32;
    }
    // Their capstone or wall
    else {
        value_params.critical_squares[3] += Us::color().multiplier() as f32
    }

    // Bonus for having our cap next to our critical square
    for neighbour in critical_square.neighbours::<S>() {
        if position[neighbour].top_stone() == Some(Us::cap_piece()) {
            value_params.critical_squares[4] += Us::color().multiplier() as f32;
            // Further bonus for a capped stack next to our critical square
            for piece in position[neighbour].into_iter() {
                if piece == Us::flat_piece() {
                    value_params.critical_squares[5] += Us::color().multiplier() as f32;
                }
            }
        }
    }
}

fn line_score<Us: ColorTr, Them: ColorTr, const S: usize>(
    group_data: &GroupData<S>,
    line: BitBoard,
    value_params: &mut ValueParameters,
) {
    let road_pieces_in_line = (Us::road_stones(group_data) & line).count();

    value_params.line_control[road_pieces_in_line as usize] += Us::color().multiplier() as f32;

    // Bonus for blocking their lines
    if road_pieces_in_line >= 3 {
        value_params.block_their_line[road_pieces_in_line as usize - 3] +=
            ((Them::flats(group_data) & line).count() as isize * Them::color().multiplier()) as f32;
        value_params.block_their_line[2 + road_pieces_in_line as usize - 3] +=
            ((Them::walls(group_data) & line).count() as isize * Them::color().multiplier()) as f32;
        value_params.block_their_line[4 + road_pieces_in_line as usize - 3] +=
            ((Them::caps(group_data) & line).count() as isize * Them::color().multiplier()) as f32;
    }
}
