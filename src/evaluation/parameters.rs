use crate::position::num_square_symmetries;

pub const NUM_VALUE_FEATURES_4S: usize = 51;
pub const NUM_POLICY_FEATURES_4S: usize = 130;

pub const NUM_VALUE_FEATURES_5S: usize = 69;
pub const NUM_POLICY_FEATURES_5S: usize = 149;

pub const NUM_VALUE_FEATURES_6S: usize = 72;
pub const NUM_POLICY_FEATURES_6S: usize = 159;

#[derive(Debug)]
pub struct ValueFeatures<'a> {
    pub flat_psqt: &'a mut [f32],
    pub wall_psqt: &'a mut [f32],
    pub cap_psqt: &'a mut [f32],
    pub our_stack_psqt: &'a mut [f32],
    pub their_stack_psqt: &'a mut [f32],
    pub side_to_move: &'a mut [f32],
    pub flatstone_lead: &'a mut [f32],
    pub i_number_of_groups: &'a mut [f32],
    pub critical_squares: &'a mut [f32],
    pub capstone_over_own_piece: &'a mut [f32],
    pub capstone_on_stack: &'a mut [f32],
    pub standing_stone_on_stack: &'a mut [f32],
    pub flat_stone_next_to_our_stack: &'a mut [f32],
    pub standing_stone_next_to_our_stack: &'a mut [f32],
    pub capstone_next_to_our_stack: &'a mut [f32],
    pub num_lines_occupied: &'a mut [f32],
    pub line_control: &'a mut [f32],
    pub block_their_line: &'a mut [f32],
}

impl<'a> ValueFeatures<'a> {
    pub fn new<const S: usize>(coefficients: &'a mut [f32]) -> Self {
        assert_eq!(coefficients.len(), num_value_features::<S>());

        let (flat_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (wall_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (cap_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (our_stack_psqt, coefficients) =
            coefficients.split_at_mut(num_square_symmetries::<S>());
        let (their_stack_psqt, coefficients) =
            coefficients.split_at_mut(num_square_symmetries::<S>());
        let (side_to_move, coefficients) = coefficients.split_at_mut(3);
        let (flatstone_lead, coefficients) = coefficients.split_at_mut(3);
        let (i_number_of_groups, coefficients) = coefficients.split_at_mut(3);
        let (critical_squares, coefficients) = coefficients.split_at_mut(6);
        let (capstone_over_own_piece, coefficients) = coefficients.split_at_mut(1);
        let (capstone_on_stack, coefficients) = coefficients.split_at_mut(1);
        let (standing_stone_on_stack, coefficients) = coefficients.split_at_mut(1);
        let (flat_stone_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (standing_stone_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (capstone_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (num_lines_occupied, coefficients) = coefficients.split_at_mut(S + 1);
        let (line_control, coefficients) = coefficients.split_at_mut(S + 1);
        let (block_their_line, coefficients) = coefficients.split_at_mut(S + 1);

        assert!(coefficients.is_empty());

        ValueFeatures {
            flat_psqt,
            wall_psqt,
            cap_psqt,
            our_stack_psqt,
            their_stack_psqt,
            side_to_move,
            flatstone_lead,
            i_number_of_groups,
            critical_squares,
            capstone_over_own_piece,
            capstone_on_stack,
            standing_stone_on_stack,
            flat_stone_next_to_our_stack,
            standing_stone_next_to_our_stack,
            capstone_next_to_our_stack,
            num_lines_occupied,
            line_control,
            block_their_line,
        }
    }
}

#[derive(Debug)]
pub struct PolicyFeatures<'a> {
    pub decline_win: &'a mut [f32],
    pub place_to_win: &'a mut [f32],
    pub place_to_draw: &'a mut [f32],
    pub place_to_loss: &'a mut [f32],
    pub place_to_allow_opponent_to_end: &'a mut [f32],
    pub two_flats_left: &'a mut [f32],
    pub three_flats_left: &'a mut [f32],
    pub flat_psqt: &'a mut [f32],
    pub wall_psqt: &'a mut [f32],
    pub cap_psqt: &'a mut [f32],
    pub our_road_stones_in_line: &'a mut [f32],
    pub their_road_stones_in_line: &'a mut [f32],
    pub extend_single_group_base: &'a mut [f32],
    pub extend_single_group_linear: &'a mut [f32],
    pub extend_single_group_to_new_line_base: &'a mut [f32],
    pub extend_single_group_to_new_line_linear: &'a mut [f32],
    pub merge_two_groups_base: &'a mut [f32],
    pub merge_two_groups_linear: &'a mut [f32],
    pub block_merger_base: &'a mut [f32],
    pub block_merger_linear: &'a mut [f32],
    pub place_our_critical_square: &'a mut [f32],
    pub place_their_critical_square: &'a mut [f32],
    pub ignore_their_critical_square: &'a mut [f32],
    pub next_to_our_last_stone: &'a mut [f32],
    pub next_to_their_last_stone: &'a mut [f32],
    pub diagonal_to_our_last_stone: &'a mut [f32],
    pub diagonal_to_their_last_stone: &'a mut [f32],
    pub attack_strong_flats: &'a mut [f32],
    pub blocking_stone_blocks_extensions_of_two_flats: &'a mut [f32],
    pub attack_strong_stack_with_wall: &'a mut [f32],
    pub attack_strong_stack_with_cap: &'a mut [f32],
    pub attack_last_movement: &'a mut [f32],
    pub place_last_movement: &'a mut [f32],
    pub move_role_bonus: &'a mut [f32],
    pub stack_movement_that_gives_us_top_pieces: &'a mut [f32],
    pub stack_captured_by_movement: &'a mut [f32],
    pub stack_capture_in_strong_line: &'a mut [f32],
    pub stack_capture_in_strong_line_cap: &'a mut [f32],
    pub move_cap_onto_strong_line: &'a mut [f32],
    pub move_cap_onto_strong_line_with_critical_square: &'a mut [f32],
    pub recapture_stack_pure: &'a mut [f32],
    pub recapture_stack_impure: &'a mut [f32],
    pub move_last_placement: &'a mut [f32],
    pub continue_spread: &'a mut [f32],
    pub move_onto_critical_square: &'a mut [f32],
    pub spread_that_connects_groups_to_win: &'a mut [f32],
}

impl<'a> PolicyFeatures<'a> {
    #[inline(never)]
    pub fn new<const S: usize>(coefficients: &'a mut [f32]) -> PolicyFeatures<'a> {
        assert_eq!(coefficients.len(), num_policy_features::<S>());

        let (decline_win, coefficients) = coefficients.split_at_mut(1);
        let (place_to_win, coefficients) = coefficients.split_at_mut(1);
        let (place_to_draw, coefficients) = coefficients.split_at_mut(1);
        let (place_to_loss, coefficients) = coefficients.split_at_mut(1);
        let (place_to_allow_opponent_to_end, coefficients) = coefficients.split_at_mut(3);
        let (two_flats_left, coefficients) = coefficients.split_at_mut(2);
        let (three_flats_left, coefficients) = coefficients.split_at_mut(2);
        let (flat_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (wall_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (cap_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (our_road_stones_in_line, coefficients) = coefficients.split_at_mut(S * 3);
        let (their_road_stones_in_line, coefficients) = coefficients.split_at_mut(S * 3);
        let (extend_single_group_to_new_line_base, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_to_new_line_linear, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_base, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_linear, coefficients) = coefficients.split_at_mut(3);
        let (merge_two_groups_base, coefficients) = coefficients.split_at_mut(3);
        let (merge_two_groups_linear, coefficients) = coefficients.split_at_mut(3);
        let (block_merger_base, coefficients) = coefficients.split_at_mut(3);
        let (block_merger_linear, coefficients) = coefficients.split_at_mut(3);
        let (place_our_critical_square, coefficients) = coefficients.split_at_mut(1);
        let (place_their_critical_square, coefficients) = coefficients.split_at_mut(4);
        let (ignore_their_critical_square, coefficients) = coefficients.split_at_mut(2);
        let (next_to_our_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (next_to_their_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (diagonal_to_our_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (diagonal_to_their_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (attack_strong_flats, coefficients) = coefficients.split_at_mut(1);
        let (blocking_stone_blocks_extensions_of_two_flats, coefficients) =
            coefficients.split_at_mut(1);
        let (attack_strong_stack_with_wall, coefficients) = coefficients.split_at_mut(6);
        let (attack_strong_stack_with_cap, coefficients) = coefficients.split_at_mut(6);
        let (attack_last_movement, coefficients) = coefficients.split_at_mut(4);
        let (place_last_movement, coefficients) = coefficients.split_at_mut(3);
        let (move_role_bonus, coefficients) = coefficients.split_at_mut(3);
        let (stack_movement_that_gives_us_top_pieces, coefficients) = coefficients.split_at_mut(6);
        let (stack_captured_by_movement, coefficients) = coefficients.split_at_mut(1);
        let (stack_capture_in_strong_line, coefficients) = coefficients.split_at_mut(S - 3);
        let (stack_capture_in_strong_line_cap, coefficients) = coefficients.split_at_mut(S - 3);
        let (move_cap_onto_strong_line, coefficients) = coefficients.split_at_mut(S - 3);
        let (move_cap_onto_strong_line_with_critical_square, coefficients) =
            coefficients.split_at_mut(S - 3);
        let (recapture_stack_pure, coefficients) = coefficients.split_at_mut(3);
        let (recapture_stack_impure, coefficients) = coefficients.split_at_mut(3);
        let (move_last_placement, coefficients) = coefficients.split_at_mut(3);
        let (continue_spread, coefficients) = coefficients.split_at_mut(3);
        let (move_onto_critical_square, coefficients) = coefficients.split_at_mut(3);
        let (spread_that_connects_groups_to_win, coefficients) = coefficients.split_at_mut(1);

        assert!(coefficients.is_empty());

        PolicyFeatures {
            decline_win,
            place_to_win,
            place_to_draw,
            place_to_loss,
            place_to_allow_opponent_to_end,
            two_flats_left,
            three_flats_left,
            flat_psqt,
            wall_psqt,
            cap_psqt,
            our_road_stones_in_line,
            their_road_stones_in_line,
            extend_single_group_base,
            extend_single_group_linear,
            extend_single_group_to_new_line_base,
            extend_single_group_to_new_line_linear,
            merge_two_groups_base,
            merge_two_groups_linear,
            block_merger_base,
            block_merger_linear,
            place_our_critical_square,
            place_their_critical_square,
            ignore_their_critical_square,
            next_to_our_last_stone,
            next_to_their_last_stone,
            diagonal_to_our_last_stone,
            diagonal_to_their_last_stone,
            attack_strong_flats,
            blocking_stone_blocks_extensions_of_two_flats,
            attack_strong_stack_with_wall,
            attack_strong_stack_with_cap,
            attack_last_movement,
            place_last_movement,
            move_role_bonus,
            stack_movement_that_gives_us_top_pieces,
            stack_captured_by_movement,
            stack_capture_in_strong_line,
            stack_capture_in_strong_line_cap,
            move_cap_onto_strong_line,
            move_cap_onto_strong_line_with_critical_square,
            recapture_stack_pure,
            recapture_stack_impure,
            move_last_placement,
            continue_spread,
            move_onto_critical_square,
            spread_that_connects_groups_to_win,
        }
    }
}

pub fn num_value_features<const S: usize>() -> usize {
    match S {
        4 => NUM_VALUE_FEATURES_4S,
        5 => NUM_VALUE_FEATURES_5S,
        6 => NUM_VALUE_FEATURES_6S,
        _ => unimplemented!(),
    }
}

pub fn num_policy_features<const S: usize>() -> usize {
    match S {
        4 => NUM_POLICY_FEATURES_4S,
        5 => NUM_POLICY_FEATURES_5S,
        6 => NUM_POLICY_FEATURES_6S,
        _ => unimplemented!(),
    }
}

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_4S: [f32; NUM_VALUE_FEATURES_4S] = [
    0.57654625,
    0.7234519,
    0.9947288,
    1.3619405,
    1.8534485,
    2.3675127,
    0.002932434,
    -0.0025715088,
    -0.00047107227,
    1.9982809,
    2.0331223,
    2.2268746,
    1.2895253,
    1.425842,
    1.6549771,
    1.735017,
    1.6928818,
    2.3276055,
    0.5782846,
    0.23609775,
    1.4875325,
    -0.20662633,
    0.06555736,
    0.06037165,
    0.35478404,
    -0.043982003,
    0.07589103,
    0.0114656,
    0.005802648,
    0.005024814,
    0.0038007405,
    -0.008236115,
    1.1429316,
    0.015894145,
    -0.27121583,
    -0.0027140165,
    1.1204548,
    -1.4377198,
    -0.6954974,
    0.07622381,
    0.9358552,
    -1.1236336,
    -0.7948103,
    0.34426185,
    1.5628102,
    -0.005560641,
    -0.036541518,
    -0.0077532027,
    0.12844211,
    -0.0055643534,
    -0.0015320918,
];
#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_4S: [f32; NUM_POLICY_FEATURES_4S] = [
    -3.9657896,
    1.4110678,
    1.0998982,
    -3.1612859,
    -3.3797283,
    0.22887644,
    0.46740365,
    -0.25038686,
    0.19303164,
    -0.16947459,
    0.15422852,
    0.06293144,
    0.16235097,
    0.43227813,
    -0.38303038,
    -0.504268,
    0.21411678,
    0.007571606,
    0.009635687,
    -0.0014058612,
    0.0085868295,
    -0.27475312,
    0.46439883,
    1.078042,
    0.020157898,
    0.14288536,
    0.055918362,
    -1.5474207,
    0.0061870757,
    -0.0077467104,
    -0.007937893,
    0.0076041985,
    0.14159878,
    -0.38050216,
    0.3239574,
    1.2015599,
    -0.7297661,
    -0.6360258,
    -0.5853736,
    0.6284847,
    -0.0057374667,
    0.00015021767,
    0.008557079,
    -0.0009557083,
    0.6104631,
    -0.5925414,
    -0.009191279,
    -1.3853506,
    0.5065972,
    -0.00008111913,
    0.45035902,
    0.8281232,
    -0.008808966,
    0.14468262,
    -0.81799924,
    -0.0049592447,
    0.8167471,
    1.2574946,
    0.002150652,
    0.1963947,
    -0.7948367,
    -0.004483435,
    0.17575556,
    0.4369178,
    0.008150065,
    -0.84050864,
    -0.34566236,
    0.0040705632,
    2.4563792,
    0.23108627,
    0.9188401,
    0.008161059,
    0.79233116,
    -3.536681,
    -1.7433494,
    0.24274868,
    1.396689,
    0.43048444,
    0.28698713,
    0.27532858,
    0.58915645,
    0.18701936,
    0.33074096,
    -0.3476396,
    0.54176253,
    0.008410923,
    0.0043121455,
    -0.008000903,
    0.0010137418,
    -0.006815267,
    0.0031001903,
    -0.002478423,
    -0.002353027,
    0.35292476,
    0.07396798,
    -0.007463541,
    -0.009312019,
    0.06629895,
    -0.21437466,
    0.0035254955,
    -1.3564991,
    -1.2744789,
    0.008508706,
    -0.04956134,
    0.8928778,
    -1.5557747,
    0.62927735,
    -1.0346689,
    0.24117747,
    0.5185278,
    -0.20043422,
    -0.005868571,
    -0.0075792954,
    0.00952702,
    1.4246035,
    0.761231,
    -0.0077517964,
    -1.1132826,
    0.28384545,
    -0.0027733874,
    0.427133,
    1.0458082,
    0.001523626,
    0.120106295,
    -0.09654278,
    -0.0018659113,
    0.7107393,
    2.2507515,
    0.70542514,
    2.967115,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_5S: [f32; NUM_VALUE_FEATURES_5S] = [
    0.041438956,
    0.18446095,
    0.1868889,
    0.36185566,
    0.2977145,
    0.24137947,
    1.0061449,
    1.2283852,
    1.2568935,
    1.5351268,
    1.5653604,
    1.5840644,
    -0.34000713,
    0.14839143,
    0.17265277,
    0.821358,
    1.0685441,
    1.3443707,
    1.565895,
    1.6729121,
    1.7397277,
    1.7779435,
    1.8452426,
    1.9811568,
    0.91039133,
    0.993802,
    1.0050044,
    1.0602405,
    1.0913749,
    1.1558495,
    1.4146647,
    1.1090266,
    1.4144582,
    0.8148615,
    -0.0011314729,
    0.50623226,
    -0.24223273,
    -0.17486069,
    -0.062269866,
    0.31759375,
    0.05862558,
    0.14427549,
    0.014593944,
    0.073600724,
    0.17297962,
    1.0740496,
    0.8490231,
    0.39128616,
    -0.035940174,
    -0.24668843,
    -0.2195351,
    1.0292486,
    -1.1224568,
    -0.664381,
    -0.23911083,
    0.23164476,
    0.7603211,
    -1.5782015,
    -1.1137623,
    -0.18412527,
    0.926742,
    1.9477305,
    0.0022945618,
    0.02982622,
    0.011772434,
    0.105311766,
    0.23329736,
    0.11333126,
    0.13913,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_5S: [f32; NUM_POLICY_FEATURES_5S] = [
    -4.755364,
    1.1311738,
    0.58556867,
    -3.4021764,
    -3.1373155,
    0.04412974,
    0.458972,
    -0.30885246,
    0.14668843,
    -0.14313453,
    0.0872659,
    -0.1082516,
    0.16851811,
    0.065404534,
    0.3631787,
    0.06401016,
    -0.104257725,
    -0.21088494,
    -0.31087786,
    -0.27435446,
    0.08521605,
    0.14504233,
    0.24366389,
    -1.4423789,
    -1.3047569,
    -0.94594556,
    0.4572711,
    1.0609539,
    2.7472804,
    0.013866621,
    -0.27202812,
    -0.0010028178,
    0.4315712,
    0.7059404,
    -0.20276293,
    -0.14279109,
    -0.023965461,
    -0.047358107,
    -0.24944656,
    0.013453558,
    0.04208832,
    0.5729235,
    1.3989291,
    -0.8662238,
    0.28540576,
    -0.17574224,
    0.02379508,
    0.37689117,
    0.40937334,
    -0.33514708,
    -0.4670903,
    -0.4427345,
    0.05848252,
    0.5580883,
    -0.5805978,
    -0.70629185,
    -0.30271703,
    0.4686727,
    2.294098,
    0.60297436,
    0.11505742,
    1.4712111,
    -0.02313266,
    -0.7007273,
    -2.5861368,
    0.36617997,
    -0.029973844,
    -0.6797968,
    0.25856647,
    0.19340551,
    0.5791781,
    1.587876,
    0.3253362,
    1.4125295,
    -0.15290302,
    -0.17460127,
    -2.217749,
    0.4083747,
    0.71800256,
    1.4114511,
    -0.22816692,
    -0.58337367,
    -0.45859587,
    2.230897,
    0.21277095,
    2.1068072,
    2.6636884,
    0.7823231,
    -4.1001625,
    -1.795641,
    0.39795363,
    1.4063269,
    0.32883975,
    0.41488165,
    0.15165795,
    0.30999684,
    0.27579188,
    0.3322012,
    -0.046668332,
    0.3830913,
    -0.015490389,
    0.39347458,
    0.2111288,
    -0.9909295,
    0.12670685,
    0.4658484,
    -0.28010443,
    -0.16104476,
    0.27260667,
    -0.04851852,
    0.004360006,
    0.009378757,
    0.33778733,
    0.092597574,
    1.1445029,
    -0.93961936,
    -0.5925652,
    -0.45683023,
    0.20834558,
    0.2884825,
    -0.9228111,
    -0.14170785,
    -1.2900378,
    -0.25622728,
    0.44415691,
    -0.028698953,
    -0.14867443,
    -0.0018671652,
    -0.004052444,
    -0.027956044,
    0.35590026,
    -1.2162863,
    -0.5925793,
    1.9376279,
    1.0999765,
    1.5152308,
    -0.39294106,
    1.1180122,
    1.2197584,
    0.5836516,
    1.1542743,
    1.2598281,
    0.25476572,
    0.25285935,
    -0.3309673,
    0.12298836,
    2.8491154,
    1.186176,
    3.5280263,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_6S: [f32; NUM_VALUE_FEATURES_6S] = [
    0.12913151,
    0.21049066,
    0.2136232,
    0.3175009,
    0.3217562,
    0.32404542,
    0.5836351,
    0.7497246,
    0.7110564,
    0.9339223,
    0.9673687,
    1.0005938,
    -0.41183302,
    0.2462945,
    -0.15353133,
    0.50545806,
    0.59825927,
    0.87474155,
    0.93509305,
    1.0190737,
    1.161882,
    1.0968753,
    1.16822,
    1.2531847,
    0.6472724,
    0.7134064,
    0.7063712,
    0.74101335,
    0.7333811,
    0.79781294,
    0.78125894,
    0.6212712,
    0.84323263,
    0.75081724,
    0.19698575,
    0.57489365,
    -0.28342775,
    -0.20531914,
    -0.0103869,
    0.19275716,
    0.07498755,
    0.1520524,
    0.11034027,
    0.017167749,
    0.15252541,
    0.4625866,
    0.7375631,
    0.33749494,
    -0.023914497,
    -0.14729449,
    -0.15327369,
    0.6941552,
    -0.51021713,
    -0.39183077,
    -0.2715756,
    -0.09143286,
    0.15937762,
    0.41440424,
    -0.7953427,
    -0.62220746,
    -0.27394596,
    0.12192368,
    0.6079163,
    0.9622828,
    0.0074355192,
    -0.0010859311,
    0.021004284,
    -0.047369882,
    0.03462406,
    0.02916825,
    0.1299186,
    0.082509495,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_6S: [f32; NUM_POLICY_FEATURES_6S] = [
    -3.550531,
    1.2050589,
    0.08534944,
    -3.101333,
    -2.6935265,
    -0.33181474,
    0.37266365,
    -0.3711306,
    0.15151322,
    -0.07006235,
    0.07393767,
    -0.0680335,
    0.11945623,
    -0.008669627,
    0.34058875,
    0.18554015,
    0.17549106,
    -0.30724218,
    -0.22683549,
    -0.40332624,
    0.18168017,
    0.1415329,
    0.30604914,
    -0.693156,
    -0.6349139,
    -1.3581941,
    -0.012733441,
    0.34799147,
    2.6680872,
    -0.03378376,
    -0.2747797,
    -0.04915343,
    0.31591895,
    0.7028493,
    0.777295,
    -0.22491217,
    -0.1902395,
    -0.09779265,
    -0.04139276,
    -0.058598585,
    -0.019598523,
    0.049780674,
    -0.19015957,
    0.045224924,
    0.20604227,
    0.66072845,
    -0.051861376,
    0.27487612,
    -0.08455412,
    -0.012857437,
    0.23040591,
    0.5361399,
    0.5328841,
    -0.31654373,
    -0.36496377,
    -0.41385058,
    -0.25641102,
    0.2053836,
    0.5369314,
    -0.6030351,
    -0.9065343,
    -0.6682443,
    -0.069989346,
    0.70962363,
    2.2435806,
    0.57447076,
    0.04382714,
    0.66673404,
    0.25973597,
    -0.2784837,
    -0.9918223,
    0.3029386,
    0.13544987,
    0.6182652,
    0.24783571,
    0.1912321,
    -0.74234766,
    1.5904334,
    0.5417244,
    1.5277872,
    -0.094195165,
    -0.0854193,
    -1.3091526,
    0.47011936,
    0.57340926,
    1.3287479,
    -0.10471499,
    -0.28266308,
    -0.66686076,
    2.4135087,
    0.05458162,
    2.2658947,
    1.2613202,
    0.59063333,
    -4.006143,
    -2.4372873,
    0.78284323,
    1.4926254,
    0.4988849,
    0.6356159,
    0.10166236,
    0.40014786,
    0.28954545,
    0.24012913,
    -0.053943917,
    0.3206997,
    -0.038345043,
    0.27737284,
    -0.27996564,
    -1.0794263,
    0.18159382,
    0.2771519,
    -0.47490522,
    -0.009172353,
    0.28810635,
    -0.04270378,
    0.0054334905,
    -0.006010456,
    0.35744894,
    -0.055825926,
    1.6105186,
    -0.9115012,
    -0.5880818,
    -0.7681756,
    0.3987682,
    0.26745403,
    -0.8625345,
    -0.06436724,
    -1.7961493,
    -1.0450668,
    0.41508627,
    -0.035574332,
    -0.044374008,
    -0.12733561,
    -0.042140316,
    0.00841472,
    0.13146819,
    -0.1487403,
    -0.05567862,
    -0.20147263,
    -0.11960911,
    -0.44396985,
    -0.28371426,
    2.2582805,
    1.6303967,
    2.0329573,
    0.04463243,
    1.307071,
    1.9419839,
    0.82215774,
    1.7664212,
    1.8839813,
    0.35882273,
    0.35288993,
    -0.53976524,
    0.17733541,
    3.2906303,
    1.184876,
    3.3339803,
];
