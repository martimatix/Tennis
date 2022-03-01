extern crate tennis;

const POINTS_TO_WIN_GAME: u8 = 4;

pub fn win_n_points<'a>(
    mut tennis_match: tennis::Match<'a>,
    player_name: &str,
    n: usize,
) -> tennis::Match<'a> {
    for _ in 0..n {
        tennis_match.point_won_by(player_name);
    }
    tennis_match
}

pub fn win_n_games<'a>(
    mut tennis_match: tennis::Match<'a>,
    player_name: &str,
    n: u8,
) -> tennis::Match<'a> {
    for _ in 0..(n * POINTS_TO_WIN_GAME) {
        tennis_match.point_won_by(player_name);
    }
    tennis_match
}

pub fn initialize_tie_break<'a>(
    mut tennis_match: tennis::Match<'a>,
    player_1_name: &str,
    player_2_name: &str,
) -> tennis::Match<'a> {
    tennis_match = win_n_games(tennis_match, player_1_name, 5);
    tennis_match = win_n_games(tennis_match, player_2_name, 6);
    tennis_match = win_n_games(tennis_match, player_1_name, 1);
    tennis_match
}
