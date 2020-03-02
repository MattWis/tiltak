use crate::board;
use crate::board::Role::*;
use crate::board::{
    Board, ColorTr, Direction, Move, Movement, Piece, Square, StackMovement, BOARD_SIZE,
};
use arrayvec::ArrayVec;

impl Board {
    pub fn generate_moves_colortr<Us: ColorTr, Them: ColorTr>(
        &self,
        moves: &mut Vec<<Board as board_game_traits::board::Board>::Move>,
    ) {
        for square in board::board_iterator() {
            match self[square].top_stone() {
                None => {
                    if Us::stones_left(&self) > 0 {
                        moves.push(Move::Place(Us::flat_piece(), square));
                        moves.push(Move::Place(Us::standing_piece(), square));
                    }
                    if Us::capstones_left(&self) > 0 {
                        moves.push(Move::Place(Us::cap_piece(), square));
                    }
                }
                Some(piece) if Us::piece_is_ours(piece) => {
                    for direction in square.directions() {
                        let mut movements = vec![];
                        if piece == Us::cap_piece() {
                            self.generate_moving_moves_cap::<Us>(
                                direction,
                                square,
                                square,
                                self[square].len() as u8,
                                &ArrayVec::new(),
                                &mut movements,
                            );
                        } else if Us::piece_is_ours(piece) {
                            self.generate_moving_moves_non_cap::<Us>(
                                direction,
                                square,
                                square,
                                self[square].len() as u8,
                                &ArrayVec::new(),
                                &mut movements,
                            );
                        }
                        for movements in movements.into_iter().filter(|mv| !mv.is_empty()) {
                            let stack_movement = StackMovement { movements };
                            let mv = Move::Move(square, direction, stack_movement.clone());
                            if !self.move_is_suicide::<Us, Them>(&mv) {
                                moves.push(mv);
                            }
                        }
                    }
                }
                Some(_) => (),
            }
        }
    }

    pub fn generate_moving_moves_cap<Colorr: ColorTr>(
        &self,
        direction: Direction,
        origin_square: Square,
        square: Square,
        pieces_carried: u8,
        partial_movement: &ArrayVec<[Movement; BOARD_SIZE - 1]>,
        movements: &mut Vec<ArrayVec<[Movement; BOARD_SIZE - 1]>>,
    ) {
        if let Some(neighbour) = square.go_direction(direction) {
            let max_pieces_to_take = if square == origin_square {
                pieces_carried.min(BOARD_SIZE as u8)
            } else {
                (pieces_carried - 1).min(BOARD_SIZE as u8)
            };
            let neighbour_piece = self[neighbour].top_stone();
            if neighbour_piece.map(Piece::role) == Some(Cap) {
                return;
            }
            if neighbour_piece.map(Piece::role) == Some(Standing) && max_pieces_to_take > 0 {
                let mut new_movement = partial_movement.clone();
                new_movement.push(Movement { pieces_to_take: 1 });
                movements.push(new_movement);
            } else {
                for pieces_to_take in 1..=max_pieces_to_take {
                    let mut new_movement = partial_movement.clone();
                    new_movement.push(Movement { pieces_to_take });

                    self.generate_moving_moves_cap::<Colorr>(
                        direction,
                        origin_square,
                        neighbour,
                        pieces_to_take,
                        &new_movement,
                        movements,
                    );
                    movements.push(new_movement);
                }
            }
        }
    }

    pub fn generate_moving_moves_non_cap<Colorr: ColorTr>(
        &self,
        direction: Direction,
        origin_square: Square,
        square: Square,
        pieces_carried: u8,
        partial_movement: &ArrayVec<[Movement; BOARD_SIZE - 1]>,
        movements: &mut Vec<ArrayVec<[Movement; BOARD_SIZE - 1]>>,
    ) {
        if let Some(neighbour) = square.go_direction(direction) {
            let neighbour_piece = self[neighbour].top_stone();
            if neighbour_piece.is_some() && neighbour_piece.unwrap().role() != Flat {
                return;
            }

            let neighbour = square.go_direction(direction).unwrap();
            let max_pieces_to_take = if square == origin_square {
                pieces_carried.min(BOARD_SIZE as u8)
            } else {
                (pieces_carried - 1).min(BOARD_SIZE as u8)
            };
            for pieces_to_take in 1..=max_pieces_to_take {
                let mut new_movement = partial_movement.clone();
                new_movement.push(Movement { pieces_to_take });

                self.generate_moving_moves_non_cap::<Colorr>(
                    direction,
                    origin_square,
                    neighbour,
                    pieces_to_take,
                    &new_movement,
                    movements,
                );
                movements.push(new_movement);
            }
        }
    }

    // Never inline, for profiling purposes
    #[inline(never)]
    pub fn move_is_suicide<Us: ColorTr, Them: ColorTr>(&self, mv: &Move) -> bool {
        if let Move::Move(square, direction, stack_movement) = mv {
            // Stack moves that don't give the opponent a new road stone,
            // can trivially be ruled out
            if self
                .top_stones_left_behind_by_move(*square, &stack_movement)
                .any(|piece| piece.is_some() && !Us::piece_is_ours(piece.unwrap()))
            {
                let mut our_road_pieces = Us::road_stones(self);
                let mut their_road_pieces = Them::road_stones(self);
                let mut sq = *square;

                for new_top_piece in self.top_stones_left_behind_by_move(*square, &stack_movement) {
                    our_road_pieces = our_road_pieces.clear(sq.0);
                    their_road_pieces = their_road_pieces.clear(sq.0);
                    if let Some(piece) = new_top_piece {
                        if Us::is_road_stone(piece) {
                            our_road_pieces = our_road_pieces.set(sq.0);
                        }
                        if Them::is_road_stone(piece) {
                            their_road_pieces = their_road_pieces.set(sq.0);
                        }
                    }
                    sq = sq.go_direction(*direction).unwrap_or(sq);
                }

                let (our_components, our_highest_component_id) =
                    board::connected_components_graph(our_road_pieces);

                if board::is_win_by_road(&our_components, our_highest_component_id).is_some() {
                    return false;
                }

                let (their_components, their_highest_component_id) =
                    board::connected_components_graph(their_road_pieces);

                return board::is_win_by_road(&their_components, their_highest_component_id)
                    .is_some();
            /*
            if let Some(winning_square) = board::is_win_by_road(&their_components, their_highest_component_id) {
                println!("Found suicide move {:?}", mv);
                let mut sq = *square;
                // First check if the winning square is among those used by the move
                // We cannot use self, since the move hasn't been played on self
                for top_piece in self.top_stones_left_behind_by_move(*square, &stack_movement) {
                    if sq == winning_square {
                        return Them::piece_is_ours(top_piece.unwrap());
                    }
                    sq = sq.go_direction(*direction).unwrap_or(sq);
                }
                // The winning square is not among the squares touched by the move
                // Now we can safely use self to check
                for sq in board::board_iterator() {
                    if sq == winning_square {
                        let top_piece = self[sq].top_stone().unwrap();
                        return Them::piece_is_ours(top_piece);
                    }
                }
                unreachable!(
                    "Couldn't find the winning square {} for move {:?} on board\n{:?}",
                    winning_square, mv, self
                );
            } else {
                false
            }
            */
            } else {
                false
            }
        } else {
            false
        }
    }
}
