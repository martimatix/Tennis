extern crate tennis;

const POINTS_TO_WIN_GAME: u8 = 4;

pub fn win_n_points(mut tennis_match: tennis::Match, player_name: &str, n: usize) -> tennis::Match {
    for _ in 0..n {
        tennis_match.point_won_by(player_name.to_string());
    }
    tennis_match
}

pub fn win_n_games(mut tennis_match: tennis::Match, player_name: &str, n: u8) -> tennis::Match {
    for _ in 0..(n * POINTS_TO_WIN_GAME) {
        tennis_match.point_won_by(player_name.to_string());
    }
    tennis_match
}

pub fn initialize_tie_break(
    mut tennis_match: tennis::Match,
    player_1_name: &str,
    player_2_name: &str,
) -> tennis::Match {
    tennis_match = win_n_games(tennis_match, player_1_name, 5);
    tennis_match = win_n_games(tennis_match, player_2_name, 6);
    tennis_match = win_n_games(tennis_match, player_1_name, 1);
    tennis_match
}
