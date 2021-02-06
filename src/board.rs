//! Tak move generation, along with all required data types.

use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref ZOBRIST_KEYS_4S: Box<ZobristKeys<4>> = ZobristKeys::new();
    pub(crate) static ref ZOBRIST_KEYS_5S: Box<ZobristKeys<5>> = ZobristKeys::new();
    pub(crate) static ref ZOBRIST_KEYS_6S: Box<ZobristKeys<6>> = ZobristKeys::new();
    pub(crate) static ref ZOBRIST_KEYS_7S: Box<ZobristKeys<7>> = ZobristKeys::new();
    pub(crate) static ref ZOBRIST_KEYS_8S: Box<ZobristKeys<8>> = ZobristKeys::new();
}

pub const MAX_BOARD_SIZE: usize = 8;

pub const fn starting_stones<const S: usize>() -> u8 {
    match S {
        3 => 10,
        4 => 16,
        5 => 21,
        6 => 30,
        7 => 40,
        8 => 50,
        _ => 0,
    }
}

pub const fn starting_capstones<const S: usize>() -> u8 {
    match S {
        3 => 0,
        4 => 0,
        5 => 1,
        6 => 1,
        7 => 2,
        8 => 2,
        _ => 0,
    }
}

pub(crate) const fn num_square_symmetries<const S: usize>() -> usize {
    match S {
        4 => 3,
        5 => 6,
        6 => 6,
        _ => 0,
    }
}

pub(crate) const fn square_symmetries<const S: usize>() -> &'static [usize] {
    match S {
        4 => &[0, 1, 1, 0, 1, 2, 2, 1, 1, 2, 2, 1, 0, 1, 1, 0],
        5 => &[
            0, 1, 2, 1, 0, 1, 3, 4, 3, 1, 2, 4, 5, 4, 2, 1, 3, 4, 3, 1, 0, 1, 2, 1, 0,
        ],
        6 => &[
            0, 1, 2, 2, 1, 0, 1, 3, 4, 4, 3, 1, 2, 4, 5, 5, 4, 2, 2, 4, 5, 5, 4, 2, 1, 3, 4, 4, 3,
            1, 0, 1, 2, 2, 1, 0,
        ],
        _ => &[],
    }
}

pub const NUM_VALUE_PARAMS_4S: usize = 51;
pub const NUM_POLICY_PARAMS_4S: usize = 78;

pub const NUM_VALUE_PARAMS_5S: usize = 69;
pub const NUM_POLICY_PARAMS_5S: usize = 93;

pub const NUM_VALUE_PARAMS_6S: usize = 72;
pub const NUM_POLICY_PARAMS_6S: usize = 99;

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_4S: [f32; NUM_VALUE_PARAMS_4S] = [
    0.40710354,
    0.54004306,
    0.7314182,
    1.0928891,
    1.0292282,
    1.3468512,
    0.0060105007,
    0.0055509824,
    -0.00028526317,
    1.3862512,
    1.7173628,
    1.6419909,
    0.9690684,
    1.0156716,
    1.0867381,
    1.3008595,
    1.4003952,
    1.7866403,
    0.48352754,
    0.10772651,
    1.0677989,
    -0.2995102,
    -0.2784462,
    0.4824724,
    0.3578643,
    0.30005231,
    0.112963594,
    0.03649128,
    -0.0068652583,
    -0.0045327637,
    -0.007574806,
    -0.001304483,
    0.7344016,
    -0.012913749,
    -0.27687806,
    -0.00081042293,
    0.8332573,
    -1.0331386,
    -0.5144214,
    -0.005624555,
    0.71696895,
    -0.81962454,
    -0.5723752,
    0.25761572,
    1.1193179,
    0.005045411,
    -0.03437448,
    0.008928416,
    0.048655495,
    -0.0016141674,
    0.003347816,
];
#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_4S: [f32; NUM_POLICY_PARAMS_4S] = [
    0.96480286,
    0.112528,
    0.25500473,
    0.4112829,
    -0.08060045,
    -0.23519231,
    -0.13165799,
    -0.008488559,
    -0.0050255177,
    0.00999761,
    -0.113257684,
    -0.16278866,
    0.6787402,
    1.1777495,
    -0.3540673,
    -0.3450718,
    -0.14545316,
    -0.03423411,
    -0.006642866,
    0.0071283225,
    -0.004408128,
    -0.008781511,
    0.3106411,
    -0.41825965,
    0.3085478,
    1.3927934,
    -0.8061303,
    -0.48430258,
    -0.39221516,
    0.7936677,
    -0.004510097,
    -0.009524884,
    -0.008346889,
    0.0068213847,
    0.060292657,
    0.056763157,
    0.0071164146,
    0.17309238,
    0.2843739,
    0.0025831861,
    -0.029959261,
    0.81146693,
    -0.0073796343,
    1.8442278,
    0.46516204,
    0.9331577,
    -0.009010632,
    0.8256556,
    -3.113514,
    -1.4565177,
    0.6001051,
    1.3691607,
    0.36907476,
    0.22568786,
    0.23353463,
    0.2539174,
    -0.98199755,
    -1.413433,
    0.0051357076,
    -0.38820496,
    0.93453836,
    -1.2818239,
    -1.3273182,
    -0.7235824,
    -0.86146796,
    0.609189,
    -0.14510402,
    0.009848367,
    -0.009406421,
    -0.008845272,
    0.001895139,
    0.007292602,
    -0.0025014114,
    0.00015416648,
    -0.045751374,
    -0.008639442,
    1.651265,
    0.66081977,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_5S: [f32; NUM_VALUE_PARAMS_5S] = [
    -0.00044795033,
    0.15347332,
    0.14927012,
    0.25764394,
    0.2447137,
    0.27844432,
    0.7183903,
    0.79589164,
    0.69361377,
    0.93700093,
    0.77688575,
    1.0438795,
    -0.47725853,
    0.023881366,
    0.10956399,
    0.6041755,
    0.7021375,
    0.9956894,
    1.1578636,
    1.1255516,
    1.2779299,
    1.2831495,
    1.311057,
    1.2934446,
    0.7101744,
    0.73263896,
    0.77619076,
    0.8653954,
    0.8186914,
    0.8584326,
    0.98251414,
    0.7959507,
    1.0613332,
    0.61214393,
    0.04162296,
    0.47685462,
    -0.18535407,
    -0.175548,
    0.025191614,
    0.31633365,
    0.044689283,
    0.08818814,
    -0.04582565,
    0.036502212,
    0.11076386,
    0.12404986,
    0.60829574,
    0.35141426,
    -0.032268483,
    -0.15010805,
    -0.15450484,
    0.7011735,
    -0.77606714,
    -0.432654,
    -0.1280988,
    0.12062097,
    0.5066281,
    -1.0205822,
    -0.7606904,
    -0.18055946,
    0.6164267,
    1.3433626,
    0.0029393125,
    0.012231762,
    -0.07691176,
    0.14723985,
    0.103527844,
    0.08759902,
    -0.0380222,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_5S: [f32; NUM_POLICY_PARAMS_5S] = [
    0.9295695,
    -0.083421424,
    0.08532888,
    0.2387281,
    0.5274521,
    0.24493831,
    0.09562564,
    0.15470326,
    -0.24693699,
    -0.30108738,
    -0.21431528,
    -0.2704717,
    -0.022588426,
    -1.7911652,
    -1.2216772,
    -0.94670635,
    -0.13166411,
    0.7949566,
    3.6376195,
    -0.19435506,
    -0.25095072,
    0.26169717,
    0.8698118,
    1.5360512,
    -0.43167987,
    -0.46735555,
    -0.46749815,
    -0.4370854,
    -0.02836023,
    -0.10167722,
    -0.40005624,
    0.26396355,
    1.1372943,
    -0.24493426,
    0.38994327,
    -0.08613078,
    0.20141272,
    0.7585348,
    0.9396367,
    -0.44191897,
    -0.654275,
    -0.7014535,
    -0.19884999,
    0.19501987,
    -0.1429219,
    -0.89655656,
    -0.26302937,
    0.91213554,
    1.0827745,
    0.034203924,
    0.018315857,
    -0.047500875,
    0.5382421,
    0.14895687,
    0.6110193,
    0.228183,
    0.75111663,
    0.77497125,
    1.953449,
    0.41483137,
    2.9484003,
    2.1978526,
    0.7461167,
    -3.6696496,
    -1.7434666,
    0.8811208,
    1.1639132,
    0.27322596,
    0.2283973,
    0.10525754,
    0.25576675,
    -0.8693829,
    -1.370112,
    -0.8019496,
    -0.062427573,
    0.4139188,
    -1.3210533,
    -1.8491417,
    -1.6519781,
    -1.9573829,
    0.5835941,
    0.021586694,
    -0.15781526,
    -0.05396689,
    0.05479999,
    0.109241985,
    0.9827647,
    0.024347505,
    -0.3630683,
    0.0024180522,
    0.0050033038,
    2.3537161,
    1.012874,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_6S: [f32; NUM_VALUE_PARAMS_6S] = [
    0.14389691,
    0.14861599,
    0.21016096,
    0.26289055,
    0.29426488,
    0.2697997,
    0.41935468,
    0.5154811,
    0.5189208,
    0.6312286,
    0.5163997,
    0.7095309,
    -0.40372136,
    -0.17229083,
    -0.08402492,
    0.22461775,
    0.32553476,
    0.5386108,
    0.45318204,
    0.63134146,
    0.763742,
    0.7841039,
    0.82111675,
    0.7907069,
    0.37137604,
    0.4108737,
    0.48396173,
    0.5127213,
    0.5265846,
    0.50946397,
    0.522094,
    0.46919465,
    0.63071007,
    0.5474945,
    0.29245657,
    0.48472342,
    -0.23528779,
    -0.04572703,
    -0.00088525214,
    0.15706137,
    0.14138941,
    0.08143665,
    -0.012585025,
    0.12482873,
    0.016435517,
    0.16839688,
    0.5649024,
    0.3192357,
    -0.025385296,
    -0.076988645,
    -0.115113735,
    0.43385288,
    -0.32607457,
    -0.24166256,
    -0.15374532,
    -0.06544677,
    0.090924904,
    0.2354019,
    -0.39573103,
    -0.33551803,
    -0.1949905,
    0.024735041,
    0.31291002,
    0.58010185,
    -0.0008957982,
    0.008336878,
    0.036577567,
    0.010293869,
    0.061720997,
    0.028714254,
    0.112023935,
    0.45978305,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_6S: [f32; NUM_POLICY_PARAMS_6S] = [
    0.89010215,
    -0.44227162,
    -0.040291704,
    -0.2304978,
    1.0121428,
    0.4682882,
    -0.02764575,
    -0.041628633,
    -0.19531427,
    -0.16233057,
    -0.13584544,
    -0.23512484,
    -0.16102716,
    -0.41830775,
    0.002314275,
    -0.922825,
    -0.05262945,
    0.044212952,
    0.8872694,
    -0.26905358,
    -0.24215038,
    0.020847214,
    0.44418764,
    0.9954152,
    0.48969522,
    -0.3372674,
    -0.50664485,
    -0.5019418,
    -0.31593677,
    -0.19631371,
    -0.006926868,
    -0.3731796,
    -0.53606695,
    -0.43022972,
    0.07106458,
    0.29349113,
    0.010416347,
    0.4458158,
    -0.10321867,
    -0.044042256,
    0.2408937,
    0.6781876,
    0.25086695,
    -0.2637086,
    -0.38575223,
    -0.5381356,
    -0.3633851,
    -0.15752447,
    -0.12405224,
    -0.50303805,
    -1.0917884,
    -0.96145505,
    -0.65610796,
    0.52460515,
    1.750216,
    0.019733787,
    0.011104343,
    0.0029525892,
    0.6340498,
    -0.010717551,
    0.3188459,
    0.28675926,
    0.49561694,
    0.84726703,
    1.9913195,
    0.3848487,
    3.3264358,
    1.6433451,
    0.5454407,
    -3.1533794,
    -1.4226677,
    1.5443202,
    1.3655221,
    0.42389217,
    0.5014939,
    0.09088122,
    0.31120455,
    -0.53451127,
    -0.9207075,
    -0.78396416,
    -0.16850425,
    0.2872031,
    -0.9876141,
    -1.1136076,
    -1.1014684,
    -1.1989646,
    0.47031713,
    0.0057362453,
    0.023288574,
    -0.055236913,
    0.09043382,
    -0.11676836,
    0.030210404,
    -0.06826632,
    0.010061843,
    0.016899843,
    0.020130021,
    0.5230228,
    0.9176952,
];

use crate::bitboard::BitBoard;
use crate::board::Direction::*;
use crate::board::Piece::*;
use crate::board::Role::Flat;
use crate::board::Role::*;
use crate::{policy_eval, search, value_eval};
use arrayvec::ArrayVec;
use board_game_traits::board;
use board_game_traits::board::GameResult::{BlackWin, Draw, WhiteWin};
use board_game_traits::board::{Board as BoardTrait, EvalBoard as EvalBoardTrait};
use board_game_traits::board::{Color, GameResult};
use pgn_traits::pgn;
use rand::{Rng, SeedableRng};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{Index, IndexMut};
use std::{fmt, iter, ops};

/// Extra items for tuning evaluation constants.
pub trait TunableBoard: BoardTrait {
    fn value_params() -> &'static [f32];
    fn policy_params() -> &'static [f32];
    type ExtraData;

    fn static_eval_coefficients(&self, coefficients: &mut [f32]);

    fn static_eval_with_params(&self, params: &[f32]) -> f32 {
        let mut coefficients: Vec<f32> = vec![0.0; Self::value_params().len()];
        self.static_eval_coefficients(&mut coefficients);
        coefficients.iter().zip(params).map(|(a, b)| a * b).sum()
    }

    fn generate_moves_with_params(
        &self,
        params: &[f32],
        data: &Self::ExtraData,
        simple_moves: &mut Vec<<Self as BoardTrait>::Move>,
        moves: &mut Vec<(<Self as BoardTrait>::Move, search::Score)>,
    );

    fn coefficients_for_move(
        &self,
        coefficients: &mut [f32],
        mv: &Move,
        data: &Self::ExtraData,
        num_legal_moves: usize,
    );

    /// Move generation that includes a heuristic probability of each move being played.
    ///
    /// # Arguments
    ///
    /// * `simple_moves` - An empty vector to temporarily store moves without probabilities. The vector will be emptied before the function returns, and only serves to re-use allocated memory.
    /// * `moves` A vector to place the moves and associated probabilities.
    fn generate_moves_with_probabilities(
        &self,
        group_data: &Self::ExtraData,
        simple_moves: &mut Vec<Move>,
        moves: &mut Vec<(Move, search::Score)>,
    );
}

pub(crate) trait ColorTr {
    fn color() -> Color;

    fn stones_left<const S: usize>(board: &Board<S>) -> u8;

    fn caps_left<const S: usize>(board: &Board<S>) -> u8;

    fn road_stones<const S: usize>(group_data: &GroupData<S>) -> BitBoard;

    fn blocking_stones<const S: usize>(group_data: &GroupData<S>) -> BitBoard;

    fn flats<const S: usize>(group_data: &GroupData<S>) -> BitBoard;

    fn walls<const S: usize>(group_data: &GroupData<S>) -> BitBoard;

    fn caps<const S: usize>(group_data: &GroupData<S>) -> BitBoard;

    fn flat_piece() -> Piece;

    fn wall_piece() -> Piece;

    fn cap_piece() -> Piece;

    fn is_road_stone(piece: Piece) -> bool;

    fn piece_is_ours(piece: Piece) -> bool;

    fn is_critical_square<const S: usize>(group_data: &GroupData<S>, square: Square) -> bool;

    fn critical_squares<const S: usize>(group_data: &GroupData<S>) -> BitBoard;
}

pub(crate) struct WhiteTr {}

impl ColorTr for WhiteTr {
    fn color() -> Color {
        Color::White
    }

    fn stones_left<const S: usize>(board: &Board<S>) -> u8 {
        board.white_stones_left
    }

    fn caps_left<const S: usize>(board: &Board<S>) -> u8 {
        board.white_caps_left
    }

    fn road_stones<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.white_road_pieces()
    }

    fn blocking_stones<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.white_blocking_pieces()
    }

    fn flats<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.white_flat_stones
    }

    fn walls<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.white_walls
    }

    fn caps<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.white_caps
    }

    fn flat_piece() -> Piece {
        Piece::WhiteFlat
    }

    fn wall_piece() -> Piece {
        Piece::WhiteWall
    }

    fn cap_piece() -> Piece {
        Piece::WhiteCap
    }

    fn is_road_stone(piece: Piece) -> bool {
        piece == WhiteFlat || piece == WhiteCap
    }

    fn piece_is_ours(piece: Piece) -> bool {
        piece == WhiteFlat || piece == WhiteWall || piece == WhiteCap
    }

    fn is_critical_square<const S: usize>(group_data: &GroupData<S>, square: Square) -> bool {
        group_data.white_critical_squares.get(square.0)
    }

    fn critical_squares<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.white_critical_squares
    }
}

pub(crate) struct BlackTr {}

impl ColorTr for BlackTr {
    fn color() -> Color {
        Color::Black
    }

    fn stones_left<const S: usize>(board: &Board<S>) -> u8 {
        board.black_stones_left
    }

    fn caps_left<const S: usize>(board: &Board<S>) -> u8 {
        board.black_caps_left
    }

    fn road_stones<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.black_road_pieces()
    }

    fn blocking_stones<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.black_blocking_pieces()
    }

    fn flats<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.black_flat_stones
    }

    fn walls<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.black_walls
    }

    fn caps<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.black_caps
    }

    fn flat_piece() -> Piece {
        Piece::BlackFlat
    }

    fn wall_piece() -> Piece {
        Piece::BlackWall
    }

    fn cap_piece() -> Piece {
        Piece::BlackCap
    }

    fn is_road_stone(piece: Piece) -> bool {
        piece == BlackFlat || piece == BlackCap
    }

    fn piece_is_ours(piece: Piece) -> bool {
        piece == BlackFlat || piece == BlackCap || piece == BlackWall
    }

    fn is_critical_square<const S: usize>(group_data: &GroupData<S>, square: Square) -> bool {
        group_data.black_critical_squares.get(square.0)
    }

    fn critical_squares<const S: usize>(group_data: &GroupData<S>) -> BitBoard {
        group_data.black_critical_squares
    }
}

/// A location on the board. Can be used to index a `Board`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Square(pub u8);

impl Square {
    pub fn from_rank_file<const S: usize>(rank: u8, file: u8) -> Self {
        debug_assert!(rank < S as u8 && file < S as u8);
        Square(rank * S as u8 + file as u8)
    }

    pub fn rank<const S: usize>(self) -> u8 {
        self.0 / S as u8
    }

    pub fn file<const S: usize>(self) -> u8 {
        self.0 % S as u8
    }

    pub fn neighbours<const S: usize>(self) -> impl Iterator<Item = Square> {
        (if self.0 as usize == 0 {
            [1, S as i8].iter()
        } else if self.0 as usize == S - 1 {
            [-1, S as i8].iter()
        } else if self.0 as usize == S * S - S {
            [1, -(S as i8)].iter()
        } else if self.0 as usize == S * S - 1 {
            [-1, -(S as i8)].iter()
        } else if self.rank::<S>() == 0 {
            [-1, 1, S as i8].iter()
        } else if self.rank::<S>() == S as u8 - 1 {
            [-(S as i8), -1, 1].iter()
        } else if self.file::<S>() == 0 {
            [-(S as i8), 1, S as i8].iter()
        } else if self.file::<S>() == S as u8 - 1 {
            [-(S as i8), -1, S as i8].iter()
        } else {
            [-(S as i8), -1, 1, S as i8].iter()
        })
        .cloned()
        .map(move |sq| sq + self.0 as i8)
        .map(|sq| Square(sq as u8))
    }

    pub fn directions<const S: usize>(self) -> impl Iterator<Item = Direction> {
        (if self.0 as usize == 0 {
            [East, South].iter()
        } else if self.0 as usize == S - 1 {
            [West, South].iter()
        } else if self.0 as usize == S * S - S {
            [East, North].iter()
        } else if self.0 as usize == S * S - 1 {
            [West, North].iter()
        } else if self.rank::<S>() == 0 {
            [West, East, South].iter()
        } else if self.rank::<S>() == S as u8 - 1 {
            [North, West, East].iter()
        } else if self.file::<S>() == 0 {
            [North, East, South].iter()
        } else if self.file::<S>() == S as u8 - 1 {
            [North, West, South].iter()
        } else {
            [North, West, East, South].iter()
        })
        .cloned()
    }

    pub fn go_direction<const S: usize>(self, direction: Direction) -> Option<Self> {
        match direction {
            North => self.0.checked_sub(S as u8).map(Square),
            West => {
                if self.file::<S>() == 0 {
                    None
                } else {
                    Some(Square(self.0 - 1))
                }
            }
            East => {
                if self.file::<S>() == S as u8 - 1 {
                    None
                } else {
                    Some(Square(self.0 + 1))
                }
            }
            South => {
                if self.0 as usize + S >= S * S {
                    None
                } else {
                    Some(Square(self.0 + S as u8))
                }
            }
        }
    }

    pub fn parse_square<const S: usize>(input: &str) -> Result<Square, pgn::Error> {
        if input.len() != 2 {
            return Err(pgn::Error::new_parse_error(format!(
                "Couldn't parse square \"{}\"",
                input
            )));
        }
        let mut chars = input.chars();
        let file = (chars.next().unwrap() as u8).overflowing_sub(b'a').0;
        let rank = (S as u8 + b'0')
            .overflowing_sub(chars.next().unwrap() as u8)
            .0;
        if file >= S as u8 || rank >= S as u8 {
            Err(pgn::Error::new_parse_error(format!(
                "Couldn't parse square \"{}\" at size {}",
                input, S
            )))
        } else {
            Ok(Square(file + rank * S as u8))
        }
    }

    pub fn to_string<const S: usize>(&self) -> String {
        let mut string = String::new();
        write!(string, "{}", (self.file::<S>() + b'a') as char).unwrap();
        write!(string, "{}", S as u8 - self.rank::<S>()).unwrap();
        string
    }
}

/// Iterates over all board squares.
pub fn squares_iterator<const S: usize>() -> impl Iterator<Item = Square> {
    (0..(S * S)).map(|i| Square(i as u8))
}

/// One of the 3 piece roles in Tak. The same as piece, but without different variants for each color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Role {
    Flat,
    Wall,
    Cap,
}

/// One of the 6 game pieces in Tak. Each piece has one variant for each color.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Piece {
    WhiteFlat = 0,
    BlackFlat = 1,
    WhiteWall = 2,
    BlackWall = 3,
    WhiteCap = 4,
    BlackCap = 5,
}

impl Piece {
    pub fn from_role_color(role: Role, color: Color) -> Self {
        match (role, color) {
            (Flat, Color::White) => WhiteFlat,
            (Wall, Color::White) => WhiteWall,
            (Cap, Color::White) => WhiteCap,
            (Flat, Color::Black) => BlackFlat,
            (Wall, Color::Black) => BlackWall,
            (Cap, Color::Black) => BlackCap,
        }
    }

    pub fn role(self) -> Role {
        match self {
            WhiteFlat | BlackFlat => Flat,
            WhiteWall | BlackWall => Wall,
            WhiteCap | BlackCap => Cap,
        }
    }

    pub fn color(self) -> Color {
        match self {
            WhiteFlat | WhiteWall | WhiteCap => Color::White,
            BlackFlat | BlackWall | BlackCap => Color::Black,
        }
    }

    pub fn is_road_piece(self) -> bool {
        WhiteTr::is_road_stone(self) || BlackTr::is_road_stone(self)
    }

    pub fn flip_color(self) -> Self {
        match self {
            WhiteFlat => BlackFlat,
            BlackFlat => WhiteFlat,
            WhiteWall => BlackWall,
            BlackWall => WhiteWall,
            WhiteCap => BlackCap,
            BlackCap => WhiteCap,
        }
    }
}

impl ops::Not for Piece {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            WhiteFlat => BlackFlat,
            BlackFlat => WhiteFlat,
            WhiteWall => BlackWall,
            BlackWall => WhiteWall,
            WhiteCap => BlackCap,
            BlackCap => WhiteCap,
        }
    }
}

/// The contents of a square on the board, consisting of zero or more pieces
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stack {
    pub(crate) top_stone: Option<Piece>,
    pub(crate) bitboard: BitBoard,
    pub(crate) height: u8,
}

impl Stack {
    /// Get a piece by index. 0 is the bottom of the stack
    pub fn get(&self, i: u8) -> Option<Piece> {
        if i >= self.height {
            None
        } else if i == self.height - 1 {
            self.top_stone
        } else if self.bitboard.get(i) {
            Some(WhiteFlat)
        } else {
            Some(BlackFlat)
        }
    }

    pub fn top_stone(&self) -> Option<Piece> {
        self.top_stone
    }

    /// Push a new piece to the top of the stack
    ///
    /// Any piece already on the stack will be flattened, including capstones
    pub fn push(&mut self, piece: Piece) {
        if self.height > 0 && self.top_stone.unwrap().color() == Color::White {
            self.bitboard = self.bitboard.set(self.height - 1);
        }
        self.top_stone = Some(piece);
        self.height += 1;
    }

    /// Remove the top piece from the stack, a
    ///
    /// Will not un-flatten a previously flattened stone
    pub fn pop(&mut self) -> Option<Piece> {
        debug_assert_ne!(self.height, 0);
        let old_piece = self.top_stone;
        if self.height > 1 {
            let piece = if self.bitboard.get(self.height - 2) {
                Piece::WhiteFlat
            } else {
                Piece::BlackFlat
            };
            self.bitboard = self.bitboard.clear(self.height - 2);
            self.top_stone = Some(piece);
        } else {
            self.top_stone = None;
        }
        self.height -= 1;
        old_piece
    }

    pub fn replace_top(&mut self, piece: Piece) -> Option<Piece> {
        self.top_stone.replace(piece)
    }

    pub fn remove(&mut self, i: u8) -> Piece {
        if i == self.height - 1 {
            self.pop().expect("Tried to remove from empty stack")
        } else {
            let piece = if self.bitboard.get(i) {
                Piece::WhiteFlat
            } else {
                Piece::BlackFlat
            };
            let pieces_below = self.bitboard & BitBoard::lower_n_bits(i);
            let pieces_above = self.bitboard & !BitBoard::lower_n_bits(i + 1);
            self.bitboard = pieces_below
                | BitBoard {
                    board: pieces_above.board >> 1,
                };
            self.height -= 1;
            piece
        }
    }

    pub fn is_empty(&self) -> bool {
        self.height == 0
    }

    pub fn len(&self) -> u8 {
        self.height
    }
}

/// An iterator over the pieces in a stack, from the bottom up
pub struct StackIterator {
    stack: Stack,
}

impl Iterator for StackIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }
}

impl IntoIterator for Stack {
    type Item = Piece;
    type IntoIter = StackIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator { stack: self }
    }
}

/// A legal move for a position.
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Move {
    Place(Role, Square),
    Move(Square, Direction, StackMovement), // Number of stones to take
}

impl Move {
    pub fn to_string<const S: usize>(&self) -> String {
        let mut string = String::new();
        match self {
            Move::Place(role, square) => match role {
                Cap => write!(string, "C{}", square.to_string::<S>()).unwrap(),
                Flat => write!(string, "{}", square.to_string::<S>()).unwrap(),
                Wall => write!(string, "S{}", square.to_string::<S>()).unwrap(),
            },
            Move::Move(square, direction, stack_movements) => {
                let mut pieces_held = stack_movements.movements[0].pieces_to_take;
                if pieces_held == 1 {
                    write!(string, "{}", square.to_string::<S>()).unwrap();
                } else {
                    write!(string, "{}{}", pieces_held, square.to_string::<S>()).unwrap();
                }
                match direction {
                    North => string.push('+'),
                    West => string.push('<'),
                    East => string.push('>'),
                    South => string.push('-'),
                }
                // Omit number of pieces dropped, if all stones are dropped immediately
                if stack_movements.movements.len() > 1 {
                    for movement in stack_movements.movements.iter().skip(1) {
                        let pieces_to_drop = pieces_held - movement.pieces_to_take;
                        write!(string, "{}", pieces_to_drop).unwrap();
                        pieces_held -= pieces_to_drop;
                    }
                    write!(string, "{}", pieces_held).unwrap();
                }
            }
        }
        string
    }

    fn from_string<const S: usize>(input: &str) -> Result<Self, pgn::Error> {
        if input.len() < 2 {
            return Err(pgn::Error::new(
                pgn::ErrorKind::ParseError,
                "Input move too short.",
            ));
        }
        if !input.is_ascii() {
            return Err(pgn::Error::new(
                pgn::ErrorKind::ParseError,
                "Input move contained non-ascii characters.",
            ));
        }
        let first_char = input.chars().next().unwrap();
        match first_char {
            'a'..='h' if input.len() == 2 => {
                let square = Square::parse_square::<S>(input)?;
                Ok(Move::Place(Flat, square))
            }
            'a'..='h' if input.len() == 3 => {
                let square = Square::parse_square::<S>(&input[0..2])?;
                let direction = Direction::parse(input.chars().nth(2).unwrap());
                // Moves in the simplified move notation always move one piece
                let movements = iter::once(Movement { pieces_to_take: 1 }).collect();
                Ok(Move::Move(square, direction, StackMovement { movements }))
            }
            'C' if input.len() == 3 => {
                Ok(Move::Place(Cap, Square::parse_square::<S>(&input[1..])?))
            }
            'S' if input.len() == 3 => {
                Ok(Move::Place(Wall, Square::parse_square::<S>(&input[1..])?))
            }
            '1'..='8' if input.len() > 3 => {
                let square = Square::parse_square::<S>(&input[1..3])?;
                let direction = Direction::parse(input.chars().nth(3).unwrap());
                let pieces_taken = first_char.to_digit(10).unwrap() as u8;
                let mut pieces_held = pieces_taken;

                let mut amounts_to_drop = input
                    .chars()
                    .skip(4)
                    .map(|ch| ch.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>();
                amounts_to_drop.pop(); //

                let mut movements = ArrayVec::new();
                movements.push(Movement {
                    pieces_to_take: pieces_taken,
                });

                for amount_to_drop in amounts_to_drop {
                    movements.push(Movement {
                        pieces_to_take: pieces_held - amount_to_drop,
                    });
                    pieces_held -= amount_to_drop;
                }
                Ok(Move::Move(square, direction, StackMovement { movements }))
            }
            _ => Err(pgn::Error::new(
                pgn::ErrorKind::ParseError,
                format!(
                    "Couldn't parse move \"{}\". Moves cannot start with {} and have length {}.",
                    input,
                    first_char,
                    input.len()
                ),
            )),
        }
    }
}

/// The counterpart of `Move`. When applied to a `Board`, it fully reverses the accompanying `Move`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReverseMove {
    Place(Square),
    Move(Square, Direction, StackMovement, bool),
}

/// One of the four cardinal directions on the board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    fn reverse(self) -> Direction {
        match self {
            North => South,
            West => East,
            East => West,
            South => North,
        }
    }

    fn parse(ch: char) -> Self {
        match ch {
            '+' => North,
            '<' => West,
            '>' => East,
            '-' => South,
            _ => panic!("Couldn't parse \"{}\" as direction.", ch),
        }
    }
}

/// One or more `Movement`s, storing how many pieces are dropped off at each step
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StackMovement {
    pub movements: ArrayVec<[Movement; MAX_BOARD_SIZE - 1]>,
}

/// Moving a stack of pieces consists of one or more `Movement`s
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Movement {
    pub pieces_to_take: u8,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(remote = "Color"))]
pub enum ColorDef {
    White,
    Black,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroupEdgeConnection {
    data: u8,
}

impl GroupEdgeConnection {
    pub fn connect_square<const S: usize>(self, square: Square) -> Self {
        let mut edge_connection = self;
        if square.rank::<S>() == S as u8 - 1 {
            edge_connection = edge_connection.connect_north();
        }
        if square.rank::<S>() == 0 {
            edge_connection = edge_connection.connect_south();
        }
        if square.file::<S>() == 0 {
            edge_connection = edge_connection.connect_west();
        }
        if square.file::<S>() == S as u8 - 1 {
            edge_connection = edge_connection.connect_east();
        }
        edge_connection
    }

    pub fn is_winning(self) -> bool {
        self.is_connected_north() && self.is_connected_south()
            || self.is_connected_east() && self.is_connected_west()
    }

    pub fn is_connected_north(self) -> bool {
        self.data & 0b1000 != 0
    }

    pub fn connect_north(self) -> Self {
        GroupEdgeConnection {
            data: self.data | 0b1000,
        }
    }

    pub fn is_connected_west(self) -> bool {
        self.data & 0b100 != 0
    }

    pub fn connect_west(self) -> Self {
        GroupEdgeConnection {
            data: self.data | 0b100,
        }
    }

    pub fn is_connected_east(self) -> bool {
        self.data & 0b10 != 0
    }

    pub fn connect_east(self) -> Self {
        GroupEdgeConnection {
            data: self.data | 0b10,
        }
    }

    pub fn is_connected_south(self) -> bool {
        self.data & 1 != 0
    }

    pub fn connect_south(self) -> Self {
        GroupEdgeConnection {
            data: self.data | 1,
        }
    }
}

impl ops::BitOr for GroupEdgeConnection {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        GroupEdgeConnection {
            data: self.data | rhs.data,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupData<const S: usize> {
    pub(crate) groups: AbstractBoard<u8, S>,
    pub(crate) amount_in_group: Box<[(u8, GroupEdgeConnection)]>,
    pub(crate) white_critical_squares: BitBoard,
    pub(crate) black_critical_squares: BitBoard,
    white_flat_stones: BitBoard,
    black_flat_stones: BitBoard,
    white_caps: BitBoard,
    black_caps: BitBoard,
    white_walls: BitBoard,
    black_walls: BitBoard,
}

impl<const S: usize> Default for GroupData<S> {
    fn default() -> Self {
        GroupData {
            groups: Default::default(),
            amount_in_group: vec![(0, GroupEdgeConnection::default()); S * S + 1]
                .into_boxed_slice(),
            white_critical_squares: Default::default(),
            black_critical_squares: Default::default(),
            white_flat_stones: Default::default(),
            black_flat_stones: Default::default(),
            white_caps: Default::default(),
            black_caps: Default::default(),
            white_walls: Default::default(),
            black_walls: Default::default(),
        }
    }
}

impl<const S: usize> GroupData<S> {
    pub(crate) fn white_road_pieces(&self) -> BitBoard {
        self.white_flat_stones | self.white_caps
    }

    pub(crate) fn black_road_pieces(&self) -> BitBoard {
        self.black_flat_stones | self.black_caps
    }

    pub(crate) fn white_blocking_pieces(&self) -> BitBoard {
        self.white_walls | self.white_caps
    }

    pub(crate) fn black_blocking_pieces(&self) -> BitBoard {
        self.black_walls | self.black_caps
    }

    pub(crate) fn all_pieces(&self) -> BitBoard {
        self.white_flat_stones
            | self.white_blocking_pieces()
            | self.black_flat_stones
            | self.black_blocking_pieces()
    }

    pub fn is_critical_square(&self, square: Square, color: Color) -> bool {
        match color {
            Color::White => WhiteTr::is_critical_square(self, square),
            Color::Black => BlackTr::is_critical_square(self, square),
        }
    }

    pub fn critical_squares<'a>(&'a self, color: Color) -> impl Iterator<Item = Square> + 'a {
        match color {
            Color::White => self.white_critical_squares.into_iter(),
            Color::Black => self.black_critical_squares.into_iter(),
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct ZobristKeys<const S: usize> {
    top_stones: AbstractBoard<[u64; 6], S>,
    stones_in_stack: [AbstractBoard<[u64; 256], S>; 8],
    to_move: [u64; 2],
}

pub fn zobrist_top_stones<const S: usize>(square: Square, piece: Piece) -> u64 {
    match S {
        4 => ZOBRIST_KEYS_4S.top_stones[square][piece as u16 as usize],
        5 => ZOBRIST_KEYS_5S.top_stones[square][piece as u16 as usize],
        6 => ZOBRIST_KEYS_6S.top_stones[square][piece as u16 as usize],
        7 => ZOBRIST_KEYS_7S.top_stones[square][piece as u16 as usize],
        8 => ZOBRIST_KEYS_8S.top_stones[square][piece as u16 as usize],
        _ => panic!("No zobrist keys for size {}. Size not supported.", S),
    }
}

pub fn zobrist_stones_in_stack<const S: usize>(
    square: Square,
    place_in_stack: usize,
    stack_slice: usize,
) -> u64 {
    match S {
        4 => ZOBRIST_KEYS_4S.stones_in_stack[place_in_stack][square][stack_slice],
        5 => ZOBRIST_KEYS_5S.stones_in_stack[place_in_stack][square][stack_slice],
        6 => ZOBRIST_KEYS_6S.stones_in_stack[place_in_stack][square][stack_slice],
        7 => ZOBRIST_KEYS_7S.stones_in_stack[place_in_stack][square][stack_slice],
        8 => ZOBRIST_KEYS_8S.stones_in_stack[place_in_stack][square][stack_slice],
        _ => panic!("No zobrist keys for size {}. Size not supported.", S),
    }
}

pub fn zobrist_to_move<const S: usize>(color: Color) -> u64 {
    match S {
        4 => ZOBRIST_KEYS_4S.to_move[color.disc()],
        5 => ZOBRIST_KEYS_5S.to_move[color.disc()],
        6 => ZOBRIST_KEYS_6S.to_move[color.disc()],
        7 => ZOBRIST_KEYS_7S.to_move[color.disc()],
        8 => ZOBRIST_KEYS_8S.to_move[color.disc()],
        _ => panic!("No zobrist keys for size {}. Size not supported.", S),
    }
}

impl<const S: usize> ZobristKeys<S> {
    pub(crate) fn new() -> Box<Self> {
        let mut rng = rand::rngs::StdRng::from_seed([0; 32]);
        let mut random_vec: Vec<u64> = vec![0; mem::size_of::<ZobristKeys<S>>() / 8];
        for word in random_vec.iter_mut() {
            *word = rng.gen();
        }
        let zobrist = unsafe { mem::transmute(Box::from_raw(random_vec.as_mut_ptr())) };

        mem::forget(random_vec);
        zobrist
    }
}

/// Complete representation of a Tak position
#[derive(Clone)]
pub struct Board<const S: usize> {
    cells: AbstractBoard<Stack, S>,
    to_move: Color,
    white_stones_left: u8,
    black_stones_left: u8,
    white_caps_left: u8,
    black_caps_left: u8,
    half_moves_played: usize,
    moves: Vec<Move>,
    hash: u64,              // Zobrist hash of current position
    hash_history: Vec<u64>, // Zobrist hashes of previous board states, up to the last irreversible move. Does not include the corrent position
}

impl<const S: usize> PartialEq for Board<S> {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
            && self.to_move == other.to_move
            && self.white_stones_left == other.white_stones_left
            && self.black_stones_left == other.black_stones_left
            && self.white_caps_left == other.white_caps_left
            && self.black_caps_left == other.black_caps_left
            && self.half_moves_played == other.half_moves_played
    }
}

impl<const S: usize> Eq for Board<S> {}

impl<const S: usize> Hash for Board<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
        self.to_move.hash(state);
        self.white_stones_left.hash(state);
        self.black_stones_left.hash(state);
        self.white_caps_left.hash(state);
        self.black_caps_left.hash(state);
        self.half_moves_played.hash(state);
    }
}

impl<const S: usize> Index<Square> for Board<S> {
    type Output = Stack;

    fn index(&self, square: Square) -> &Self::Output {
        &self.cells[square]
    }
}

impl<const S: usize> IndexMut<Square> for Board<S> {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self.cells[square]
    }
}

impl<const S: usize> Default for Board<S> {
    fn default() -> Self {
        Board {
            cells: Default::default(),
            to_move: Color::White,
            white_stones_left: starting_stones::<S>(),
            black_stones_left: starting_stones::<S>(),
            white_caps_left: starting_capstones::<S>(),
            black_caps_left: starting_capstones::<S>(),
            half_moves_played: 0,
            moves: vec![],
            hash: zobrist_to_move::<S>(Color::White),
            hash_history: vec![],
        }
    }
}

impl<const S: usize> fmt::Debug for Board<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for y in 0..S {
            for print_row in 0..3 {
                for x in 0..S {
                    for print_column in 0..3 {
                        match self.cells.raw[x][y].get(print_column * 3 + print_row) {
                            None => write!(f, "[.]")?,
                            Some(WhiteFlat) => write!(f, "[w]")?,
                            Some(WhiteWall) => write!(f, "[W]")?,
                            Some(WhiteCap) => write!(f, "[C]")?,
                            Some(BlackFlat) => write!(f, "[b]")?,
                            Some(BlackWall) => write!(f, "[B]")?,
                            Some(BlackCap) => write!(f, "[c]")?,
                        }
                    }
                    write!(f, " ")?;
                }
                writeln!(f)?;
            }
        }
        writeln!(
            f,
            "Stones left: {}/{}.",
            self.white_stones_left, self.black_stones_left
        )?;
        writeln!(
            f,
            "Capstones left: {}/{}.",
            self.white_caps_left, self.black_caps_left
        )?;
        writeln!(f, "{} to move.", self.side_to_move())?;
        writeln!(
            f,
            "Hash: {}, hash history: {:?}",
            self.hash, self.hash_history
        )?;
        Ok(())
    }
}

impl<const S: usize> Board<S> {
    #[cfg(test)]
    pub fn zobrist_hash(&self) -> u64 {
        self.hash
    }

    /// Number of moves/plies played in the game
    pub fn half_moves_played(&self) -> usize {
        self.half_moves_played
    }

    /// All the moves played in the game
    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }

    pub fn null_move(&mut self) {
        self.to_move = !self.to_move;
    }

    pub(crate) fn zobrist_hash_from_scratch(&self) -> u64 {
        let mut hash = 0;
        hash ^= zobrist_to_move::<S>(self.to_move);

        for square in squares_iterator::<S>() {
            hash ^= self.zobrist_hash_for_square(square);
        }
        hash
    }

    pub(crate) fn zobrist_hash_for_square(&self, square: Square) -> u64 {
        let mut hash = 0;
        let stack = &self[square];
        if let Some(top_stone) = stack.top_stone {
            hash ^= zobrist_top_stones::<S>(square, top_stone);
            for i in 0..(stack.len() as usize - 1) / 8 {
                hash ^= zobrist_stones_in_stack::<S>(
                    square,
                    i as usize,
                    stack.bitboard.board as usize >> (i * 8) & 255,
                )
            }
        }
        hash
    }

    fn is_critical_square_from_scratch(
        &self,
        groups: &AbstractBoard<u8, S>,
        amount_in_group: &[(u8, GroupEdgeConnection)],
        square: Square,
        color: Color,
    ) -> bool {
        let sum_of_connections = square
            .neighbours::<S>()
            .filter(|neighbour| self[*neighbour].top_stone().map(Piece::color) == Some(color))
            .map(|neighbour| amount_in_group[groups[neighbour] as usize].1)
            .fold(
                GroupEdgeConnection::default().connect_square::<S>(square),
                |acc, connection| acc | connection,
            );

        sum_of_connections.is_winning()
    }

    pub fn flip_board_y(&self) -> Board<S> {
        let mut new_board = self.clone();
        for x in 0..S as u8 {
            for y in 0..S as u8 {
                new_board[Square(y * S as u8 + x)] = self[Square((S as u8 - y - 1) * S as u8 + x)];
            }
        }
        new_board
    }

    pub fn flip_board_x(&self) -> Board<S> {
        let mut new_board = self.clone();
        for x in 0..S as u8 {
            for y in 0..S as u8 {
                new_board[Square(y * S as u8 + x)] = self[Square(y * S as u8 + (S as u8 - x - 1))];
            }
        }
        new_board
    }

    pub fn rotate_board(&self) -> Board<S> {
        let mut new_board = self.clone();
        for x in 0..S as u8 {
            for y in 0..S as u8 {
                let new_x = y;
                let new_y = S as u8 - x - 1;
                new_board[Square(y * S as u8 + x)] = self[Square(new_y * S as u8 + new_x)];
            }
        }
        new_board
    }

    pub fn flip_colors(&self) -> Board<S> {
        let mut new_board = self.clone();
        for square in squares_iterator::<S>() {
            new_board[square] = Stack::default();
            for piece in self[square] {
                new_board[square].push(piece.flip_color());
            }
        }
        mem::swap(
            &mut new_board.white_stones_left,
            &mut new_board.black_stones_left,
        );
        mem::swap(
            &mut new_board.white_caps_left,
            &mut new_board.black_caps_left,
        );
        new_board.to_move = !new_board.to_move;
        new_board
    }

    /// Returns all 8 symmetries of the board
    pub fn symmetries(&self) -> Vec<Board<S>> {
        vec![
            self.clone(),
            self.flip_board_x(),
            self.flip_board_y(),
            self.rotate_board(),
            self.rotate_board().rotate_board(),
            self.rotate_board().rotate_board().rotate_board(),
            self.rotate_board().flip_board_x(),
            self.rotate_board().flip_board_y(),
        ]
    }

    /// Returns all 16 symmetries of the board, where swapping the colors is also a symmetry
    pub fn symmetries_with_swapped_colors(&self) -> Vec<Board<S>> {
        self.symmetries()
            .into_iter()
            .flat_map(|board| vec![board.clone(), board.flip_colors()])
            .collect()
    }

    fn count_all_pieces(&self) -> u8 {
        self.cells
            .raw
            .iter()
            .flatten()
            .map(|stack: &Stack| stack.len())
            .sum()
    }

    #[inline(never)]
    pub fn group_data(&self) -> GroupData<S> {
        let mut group_data = GroupData::default();

        for square in squares_iterator::<S>() {
            match self[square].top_stone() {
                Some(WhiteFlat) => {
                    group_data.white_flat_stones = group_data.white_flat_stones.set(square.0)
                }
                Some(BlackFlat) => {
                    group_data.black_flat_stones = group_data.black_flat_stones.set(square.0)
                }
                Some(WhiteWall) => group_data.white_walls = group_data.white_walls.set(square.0),
                Some(BlackWall) => group_data.black_walls = group_data.black_walls.set(square.0),
                Some(WhiteCap) => group_data.white_caps = group_data.white_caps.set(square.0),
                Some(BlackCap) => group_data.black_caps = group_data.black_caps.set(square.0),
                None => (),
            }
        }

        let mut highest_component_id = 1;

        connected_components_graph(
            group_data.white_road_pieces(),
            &mut group_data.groups,
            &mut highest_component_id,
        );
        connected_components_graph(
            group_data.black_road_pieces(),
            &mut group_data.groups,
            &mut highest_component_id,
        );

        for square in squares_iterator::<S>() {
            group_data.amount_in_group[group_data.groups[square] as usize].0 += 1;
            if self[square].top_stone().map(Piece::is_road_piece) == Some(true) {
                group_data.amount_in_group[group_data.groups[square] as usize].1 = group_data
                    .amount_in_group[group_data.groups[square] as usize]
                    .1
                    .connect_square::<S>(square);
            }
        }

        for square in squares_iterator::<S>() {
            if self.is_critical_square_from_scratch(
                &group_data.groups,
                &group_data.amount_in_group,
                square,
                Color::White,
            ) {
                group_data.white_critical_squares = group_data.white_critical_squares.set(square.0);
            }
            if self.is_critical_square_from_scratch(
                &group_data.groups,
                &group_data.amount_in_group,
                square,
                Color::Black,
            ) {
                group_data.black_critical_squares = group_data.black_critical_squares.set(square.0);
            }
        }
        group_data
    }

    /// An iterator over the top stones left behind after a stack movement
    pub fn top_stones_left_behind_by_move<'a>(
        &'a self,
        square: Square,
        stack_movement: &'a StackMovement,
    ) -> impl Iterator<Item = Option<Piece>> + 'a {
        stack_movement
            .movements
            .iter()
            .map(move |Movement { pieces_to_take }| {
                let piece_index = self[square].len() - *pieces_to_take;
                if piece_index == 0 {
                    None
                } else {
                    Some(self[square].get(piece_index - 1).unwrap())
                }
            })
            .chain(std::iter::once(self[square].top_stone()))
    }

    pub(crate) fn game_result_with_group_data(
        &self,
        group_data: &GroupData<S>,
    ) -> Option<GameResult> {
        let repetitions = self
            .hash_history
            .iter()
            .filter(|hash| **hash == self.hash)
            .count();

        if repetitions >= 2 {
            return Some(GameResult::Draw);
        }

        if group_data
            .amount_in_group
            .iter()
            .any(|(_, group_connection)| group_connection.is_winning())
        {
            let highest_component_id = group_data
                .amount_in_group
                .iter()
                .enumerate()
                .skip(1)
                .find(|(_i, v)| (**v).0 == 0)
                .map(|(i, _v)| i)
                .unwrap_or(S * S + 1) as u8;

            if let Some(square) = self.is_win_by_road(&group_data.groups, highest_component_id) {
                debug_assert!(self[square].top_stone().unwrap().is_road_piece());
                return if self[square].top_stone().unwrap().color() == Color::White {
                    Some(GameResult::WhiteWin)
                } else {
                    Some(GameResult::BlackWin)
                };
            };
            unreachable!(
                "Board has winning connection, but isn't winning\n{:?}",
                self
            )
        }

        if (self.white_stones_left == 0 && self.white_caps_left == 0)
            || (self.black_stones_left == 0 && self.black_caps_left == 0)
            || squares_iterator::<S>().all(|square| !self[square].is_empty())
        {
            // Count points
            let mut white_points = 0;
            let mut black_points = 0;
            for square in squares_iterator::<S>() {
                match self[square].top_stone() {
                    Some(WhiteFlat) => white_points += 1,
                    Some(BlackFlat) => black_points += 1,
                    _ => (),
                }
            }
            match white_points.cmp(&black_points) {
                Ordering::Greater => Some(WhiteWin),
                Ordering::Less => Some(BlackWin),
                Ordering::Equal => Some(Draw),
            }
        } else {
            None
        }
    }

    /// Check if either side has completed a road
    /// Returns one of the winning squares in the road
    pub(crate) fn is_win_by_road(
        &self,
        components: &AbstractBoard<u8, S>,
        highest_component_id: u8,
    ) -> Option<Square> {
        // If the side to move is already winning,
        // the last move was either a suicide, or a double win
        let mut suicide_win_square = None;

        // TODO: Include highest id?
        for id in 1..highest_component_id {
            if (components.raw[0].iter().any(|&cell| cell == id)
                && components.raw[S - 1].iter().any(|&cell| cell == id))
                || ((0..S).any(|y| components.raw[y][0] == id)
                    && (0..S).any(|y| components.raw[y][S - 1] == id))
            {
                let square = squares_iterator::<S>()
                    .find(|&sq| components[sq] == id)
                    .unwrap();
                if self[square].top_stone.unwrap().color() == self.side_to_move() {
                    suicide_win_square = Some(square)
                } else {
                    return Some(square);
                }
            }
        }
        suicide_win_square
    }

    pub(crate) fn static_eval_with_params_and_data(
        &self,
        group_data: &GroupData<S>,
        params: &[f32],
    ) -> f32 {
        let mut coefficients = vec![0.0; Self::value_params().len()];
        value_eval::static_eval_game_phase(&self, group_data, &mut coefficients);
        coefficients.iter().zip(params).map(|(a, b)| a * b).sum()
    }
}

impl<const S: usize> board::Board for Board<S> {
    type Move = Move;
    type ReverseMove = ReverseMove;

    fn start_board() -> Self {
        Self::default()
    }

    fn side_to_move(&self) -> Color {
        self.to_move
    }

    /// Adds all legal moves to the provided vector. Some notes on the interpretation of the rules:
    /// * Suicide moves are considered legal, and are generated like any other move.
    /// This includes moves that complete a road for the opponent without creating an own road,
    /// and moves that fill the board when that would result in an immediate loss.
    ///
    /// * Capstones are not counted towards a flat win, but all capstones must also be placed to trigger a flat win.
    ///
    /// * A game is considered a draw after a three-fold repetition of the same position.
    fn generate_moves(&self, moves: &mut Vec<Self::Move>) {
        match self.half_moves_played() {
            0 | 1 => {
                for square in squares_iterator::<S>() {
                    if self[square].is_empty() {
                        moves.push(Move::Place(Flat, square));
                    }
                }
            }
            _ => match self.side_to_move() {
                Color::White => self.generate_moves_colortr::<WhiteTr, BlackTr>(moves),
                Color::Black => self.generate_moves_colortr::<BlackTr, WhiteTr>(moves),
            },
        }
    }

    fn do_move(&mut self, mv: Self::Move) -> Self::ReverseMove {
        self.hash_history.push(self.hash);
        let reverse_move = match mv.clone() {
            Move::Place(role, to) => {
                debug_assert!(self[to].is_empty());
                // On the first move, the players place the opponent's color
                let color_to_place = if self.half_moves_played() > 1 {
                    self.side_to_move()
                } else {
                    !self.side_to_move()
                };
                let piece = Piece::from_role_color(role, color_to_place);
                self[to].push(piece);

                match (color_to_place, role) {
                    (Color::White, Flat) => self.white_stones_left -= 1,
                    (Color::White, Wall) => self.white_stones_left -= 1,
                    (Color::White, Cap) => self.white_caps_left -= 1,
                    (Color::Black, Flat) => self.black_stones_left -= 1,
                    (Color::Black, Wall) => self.black_stones_left -= 1,
                    (Color::Black, Cap) => self.black_caps_left -= 1,
                }

                self.hash ^= zobrist_top_stones::<S>(to, piece);
                self.hash_history.clear(); // This move is irreversible, so previous position are never repeated from here

                ReverseMove::Place(to)
            }
            Move::Move(square, direction, stack_movement) => {
                let mut from = square;

                let mut pieces_left_behind: ArrayVec<[u8; MAX_BOARD_SIZE - 1]> = ArrayVec::new();
                let mut flattens_stone = false;

                for sq in <MoveIterator<S>>::new(square, direction, stack_movement.clone()) {
                    self.hash ^= self.zobrist_hash_for_square(sq);
                }

                for Movement { pieces_to_take } in stack_movement.movements.iter() {
                    let to = from.go_direction::<S>(direction).unwrap();

                    if self[to].top_stone.map(Piece::role) == Some(Wall) {
                        flattens_stone = true;
                        debug_assert!(self[from].top_stone().unwrap().role() == Cap);
                    }

                    let pieces_to_leave = self[from].len() - pieces_to_take;
                    pieces_left_behind.push(*pieces_to_take);

                    for _ in pieces_to_leave..self[from].len() {
                        let piece = self[from].get(pieces_to_leave).unwrap();
                        self[to].push(piece);
                        self[from].remove(pieces_to_leave);
                    }

                    from = to;
                }

                for sq in <MoveIterator<S>>::new(square, direction, stack_movement) {
                    self.hash ^= self.zobrist_hash_for_square(sq);
                }

                pieces_left_behind.reverse();
                ReverseMove::Move(
                    from,
                    direction.reverse(),
                    StackMovement {
                        movements: pieces_left_behind
                            .iter()
                            .map(|&pieces_to_take| Movement { pieces_to_take })
                            .collect(),
                    },
                    flattens_stone,
                )
            }
        };

        debug_assert_eq!(
            2 * (starting_stones::<S>() + starting_capstones::<S>())
                - self.white_stones_left
                - self.black_stones_left
                - self.white_caps_left
                - self.black_caps_left,
            self.count_all_pieces(),
            "Wrong number of stones on board:\n{:?}",
            self
        );

        self.moves.push(mv);
        self.half_moves_played += 1;

        self.hash ^= zobrist_to_move::<S>(self.to_move);
        self.to_move = !self.to_move;
        self.hash ^= zobrist_to_move::<S>(self.to_move);

        reverse_move
    }

    fn reverse_move(&mut self, reverse_move: Self::ReverseMove) {
        match reverse_move {
            ReverseMove::Place(square) => {
                let piece = self[square].pop().unwrap();

                self.hash ^= zobrist_top_stones::<S>(square, piece);

                debug_assert!(piece.color() != self.side_to_move() || self.half_moves_played() < 3);

                match piece {
                    WhiteFlat | WhiteWall => self.white_stones_left += 1,
                    WhiteCap => self.white_caps_left += 1,
                    BlackFlat | BlackWall => self.black_stones_left += 1,
                    BlackCap => self.black_caps_left += 1,
                };
            }

            ReverseMove::Move(from, direction, stack_movement, flattens_wall) => {
                let mut square = from;

                for square in <MoveIterator<S>>::new(from, direction, stack_movement.clone()) {
                    self.hash ^= self.zobrist_hash_for_square(square);
                }

                for Movement { pieces_to_take } in stack_movement.movements.iter() {
                    let to = square.go_direction::<S>(direction).unwrap();

                    let pieces_to_leave = self[square].len() - pieces_to_take;

                    for _ in pieces_to_leave..self[square].len() {
                        let piece = self[square].get(pieces_to_leave).unwrap();
                        self[to].push(piece);
                        self[square].remove(pieces_to_leave);
                    }
                    square = to;
                }

                if flattens_wall {
                    match self[from].top_stone().unwrap().color() {
                        Color::White => self[from].replace_top(WhiteWall),
                        Color::Black => self[from].replace_top(BlackWall),
                    };
                };

                for square in <MoveIterator<S>>::new(from, direction, stack_movement) {
                    self.hash ^= self.zobrist_hash_for_square(square);
                }
            }
        }

        self.moves.pop();
        self.hash_history.pop();
        self.half_moves_played -= 1;

        self.hash ^= zobrist_to_move::<S>(self.to_move);
        self.to_move = !self.to_move;
        self.hash ^= zobrist_to_move::<S>(self.to_move);
    }

    fn game_result(&self) -> Option<GameResult> {
        self.game_result_with_group_data(&self.group_data())
    }
}

pub(crate) struct MoveIterator<const S: usize> {
    square: Square,
    direction: Direction,
    squares_left: usize,
    _size: [(); S],
}

impl<const S: usize> MoveIterator<S> {
    pub fn new(square: Square, direction: Direction, stack_movement: StackMovement) -> Self {
        MoveIterator {
            square,
            direction,
            squares_left: stack_movement.movements.len() + 1,
            _size: [(); S],
        }
    }
}

impl<const S: usize> Iterator for MoveIterator<S> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.squares_left == 0 {
            None
        } else {
            let next_square = self.square;
            self.square = self
                .square
                .go_direction::<S>(self.direction)
                .unwrap_or(Square(0));
            self.squares_left -= 1;
            Some(next_square)
        }
    }
}

impl<const S: usize> EvalBoardTrait for Board<S> {
    fn static_eval(&self) -> f32 {
        self.static_eval_with_params(&Self::value_params())
    }
}

impl<const S: usize> TunableBoard for Board<S> {
    type ExtraData = GroupData<S>;

    fn value_params() -> &'static [f32] {
        match S {
            4 => &VALUE_PARAMS_4S,
            5 => &VALUE_PARAMS_5S,
            6 => &VALUE_PARAMS_6S,
            _ => &[],
        }
    }

    fn policy_params() -> &'static [f32] {
        match S {
            4 => &POLICY_PARAMS_4S,
            5 => &POLICY_PARAMS_5S,
            6 => &POLICY_PARAMS_6S,
            _ => &[],
        }
    }

    fn static_eval_coefficients(&self, coefficients: &mut [f32]) {
        debug_assert!(self.game_result().is_none());

        let group_data = self.group_data();
        value_eval::static_eval_game_phase(&self, &group_data, coefficients)
    }

    fn generate_moves_with_params(
        &self,
        params: &[f32],
        group_data: &GroupData<S>,
        simple_moves: &mut Vec<Self::Move>,
        moves: &mut Vec<(Self::Move, f32)>,
    ) {
        debug_assert!(simple_moves.is_empty());
        self.generate_moves(simple_moves);
        match self.side_to_move() {
            Color::White => self.generate_moves_with_probabilities_colortr::<WhiteTr, BlackTr>(
                params,
                group_data,
                simple_moves,
                moves,
            ),
            Color::Black => self.generate_moves_with_probabilities_colortr::<BlackTr, WhiteTr>(
                params,
                group_data,
                simple_moves,
                moves,
            ),
        }
    }

    fn coefficients_for_move(
        &self,
        coefficients: &mut [f32],
        mv: &Move,
        group_data: &GroupData<S>,
        num_legal_moves: usize,
    ) {
        match self.side_to_move() {
            Color::White => policy_eval::coefficients_for_move_colortr::<WhiteTr, BlackTr, S>(
                &self,
                coefficients,
                mv,
                group_data,
                num_legal_moves,
            ),
            Color::Black => policy_eval::coefficients_for_move_colortr::<BlackTr, WhiteTr, S>(
                &self,
                coefficients,
                mv,
                group_data,
                num_legal_moves,
            ),
        }
    }
    /// Move generation that includes a heuristic probability of each move being played.
    ///
    /// # Arguments
    ///
    /// * `simple_moves` - An empty vector to temporarily store moves without probabilities. The vector will be emptied before the function returns, and only serves to re-use allocated memory.
    /// * `moves` A vector to place the moves and associated probabilities.
    fn generate_moves_with_probabilities(
        &self,
        group_data: &GroupData<S>,
        simple_moves: &mut Vec<Move>,
        moves: &mut Vec<(Move, search::Score)>,
    ) {
        self.generate_moves_with_params(Self::policy_params(), group_data, simple_moves, moves)
    }
}

impl<const S: usize> pgn_traits::pgn::PgnBoard for Board<S> {
    fn from_fen(fen: &str) -> Result<Self, pgn::Error> {
        let fen_words: Vec<&str> = fen.split_whitespace().collect();

        if fen_words.len() < 3 {
            return Err(pgn::Error::new_parse_error(format!(
                "Couldn't parse TPS string \"{}\", missing move counter.",
                fen
            )));
        }
        if fen_words.len() > 3 {
            return Err(pgn::Error::new_parse_error(format!(
                "Couldn't parse TPS string \"{}\", unexpected \"{}\"",
                fen, fen_words[3]
            )));
        }

        let fen_rows: Vec<&str> = fen_words[0].split('/').collect();
        if fen_rows.len() != S {
            return Err(pgn::Error::new_parse_error(format!(
                "Couldn't parse TPS string \"{}\", had {} rows instead of {}.",
                fen,
                fen_rows.len(),
                S
            )));
        }

        let rows: Vec<[Stack; S]> = fen_rows
            .into_iter()
            .map(parse_row)
            .collect::<Result<_, _>>()
            .map_err(|e| {
                pgn::Error::new_caused_by(
                    pgn::ErrorKind::ParseError,
                    format!("Couldn't parse TPS string \"{}\"", fen),
                    e,
                )
            })?;
        let mut board = Board::default();
        for square in squares_iterator::<S>() {
            let (file, rank) = (square.file::<S>(), square.rank::<S>());
            let stack = rows[rank as usize][file as usize];
            for piece in stack.into_iter() {
                match piece {
                    WhiteFlat | WhiteWall => board.white_stones_left -= 1,
                    WhiteCap => board.white_caps_left -= 1,
                    BlackFlat | BlackWall => board.black_stones_left -= 1,
                    BlackCap => board.black_caps_left -= 1,
                }
            }
            board[square] = stack;
        }

        match fen_words[1] {
            "1" => board.to_move = Color::White,
            "2" => board.to_move = Color::Black,
            s => {
                return Err(pgn::Error::new_parse_error(format!(
                    "Error parsing TPS \"{}\": Got bad side to move \"{}\"",
                    fen, s
                )))
            }
        }

        match fen_words[2].parse::<usize>() {
            Ok(n) => match board.side_to_move() {
                Color::White => board.half_moves_played = (n - 1) * 2,
                Color::Black => board.half_moves_played = (n - 1) * 2 + 1,
            },
            Err(e) => {
                return Err(pgn::Error::new_caused_by(
                    pgn::ErrorKind::ParseError,
                    format!(
                        "Error parsing TPS \"{}\": Got bad move number \"{}\"",
                        fen, fen_words[2]
                    ),
                    e,
                ))
            }
        }

        board.hash = board.zobrist_hash_from_scratch();

        return Ok(board);

        fn parse_row<const S: usize>(row_str: &str) -> Result<[Stack; S], pgn::Error> {
            let mut column_id = 0;
            let mut row = [Stack::default(); S];
            let mut row_str_iter = row_str.chars().peekable();
            while column_id < S as u8 {
                match row_str_iter.peek() {
                    None => {
                        return Err(pgn::Error::new_parse_error(format!(
                            "Couldn't parse row \"{}\": not enough pieces",
                            row_str
                        )))
                    }
                    Some('x') => {
                        row_str_iter.next();
                        if let Some(n) = row_str_iter.peek().and_then(|ch| ch.to_digit(10)) {
                            row_str_iter.next();
                            column_id += n as u8;
                        } else {
                            column_id += 1;
                        }
                        if let Some(',') | None = row_str_iter.peek() {
                            row_str_iter.next();
                        } else {
                            return Err(pgn::Error::new_parse_error(format!(
                                "Expected ',' on row \"{}\", found {:?}",
                                row_str,
                                row_str_iter.next()
                            )));
                        }
                    }
                    Some('1') | Some('2') => {
                        let stack = &mut row[column_id as usize];
                        loop {
                            match row_str_iter.next() {
                                Some('1') => stack.push(Piece::from_role_color(Flat, Color::White)),
                                Some('2') => stack.push(Piece::from_role_color(Flat, Color::Black)),
                                Some('S') => {
                                    let piece = stack.pop().unwrap();
                                    stack.push(Piece::from_role_color(Wall, piece.color()));
                                }
                                Some('C') => {
                                    let piece = stack.pop().unwrap();
                                    stack.push(Piece::from_role_color(Cap, piece.color()));
                                }
                                Some(',') | None => {
                                    column_id += 1;
                                    break;
                                }
                                Some(ch) => {
                                    return Err(pgn::Error::new_parse_error(format!(
                                        "Expected '1', '2', 'S' or 'C' on row \"{}\", found {}",
                                        row_str, ch
                                    )))
                                }
                            }
                        }
                    }
                    Some(x) => {
                        return Err(pgn::Error::new_parse_error(format!(
                            "Unexpected '{}' in row \"{}\".",
                            x, row_str
                        )))
                    }
                }
            }
            Ok(row)
        }
    }

    fn to_fen(&self) -> String {
        let mut f = String::new();
        squares_iterator::<S>()
            .map(|square| self[square])
            .for_each(|stack: Stack| {
                (match stack.top_stone() {
                    None => write!(f, "-"),
                    Some(WhiteFlat) => write!(f, "w"),
                    Some(WhiteWall) => write!(f, "W"),
                    Some(WhiteCap) => write!(f, "C"),
                    Some(BlackFlat) => write!(f, "b"),
                    Some(BlackWall) => write!(f, "B"),
                    Some(BlackCap) => write!(f, "c"),
                })
                .unwrap()
            });
        f
    }

    fn move_from_san(&self, input: &str) -> Result<Self::Move, pgn::Error> {
        Self::Move::from_string::<S>(input)
    }

    fn move_to_san(&self, mv: &Self::Move) -> String {
        mv.to_string::<S>()
    }

    fn move_from_lan(&self, input: &str) -> Result<Self::Move, pgn::Error> {
        Self::Move::from_string::<S>(input)
    }

    fn move_to_lan(&self, mv: &Self::Move) -> String {
        self.move_to_san(mv)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct AbstractBoard<T, const S: usize> {
    raw: [[T; S]; S],
}

impl<T: Default + Copy, const S: usize> Default for AbstractBoard<T, S> {
    fn default() -> Self {
        AbstractBoard {
            raw: [[T::default(); S]; S],
        }
    }
}

impl<T, const S: usize> Index<Square> for AbstractBoard<T, S> {
    type Output = T;

    fn index(&self, square: Square) -> &Self::Output {
        &self.raw[square.0 as usize % S][square.0 as usize / S]
    }
}

impl<T, const S: usize> IndexMut<Square> for AbstractBoard<T, S> {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self.raw[square.0 as usize % S][square.0 as usize / S]
    }
}

pub(crate) fn connected_components_graph<const S: usize>(
    road_pieces: BitBoard,
    components: &mut AbstractBoard<u8, S>,
    id: &mut u8,
) {
    for square in squares_iterator::<S>() {
        if components[square] == 0 && road_pieces.get(square.0) {
            connect_component(road_pieces, components, square, *id);
            *id += 1;
        }
    }
}

fn connect_component<const S: usize>(
    road_pieces: BitBoard,
    components: &mut AbstractBoard<u8, S>,
    square: Square,
    id: u8,
) {
    components[square] = id;
    for neighbour in square.neighbours::<S>() {
        if road_pieces.get(neighbour.0) && components[neighbour] == 0 {
            connect_component(road_pieces, components, neighbour, id);
        }
    }
}
