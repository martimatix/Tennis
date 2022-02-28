extern crate tennis;

mod helpers;

#[test]
fn test_score_0_0() {
    let tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    assert_eq!(tennis_match.score(), "0-0");
}

#[test]
fn test_score_15_15() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match.point_won_by("player 1".to_string());
    tennis_match.point_won_by("player 2".to_string());
    assert_eq!(tennis_match.score(), "0-0, 15-15");
}

#[test]
fn test_score_40_15() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match.point_won_by("player 1".to_string());
    tennis_match.point_won_by("player 2".to_string());
    tennis_match.point_won_by("player 1".to_string());
    tennis_match.point_won_by("player 1".to_string());
    assert_eq!(tennis_match.score(), "0-0, 40-15");
}

#[test]
fn test_score_deuce() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 3);
    tennis_match = helpers::win_n_points(tennis_match, "player 2", 3);
    assert_eq!(tennis_match.score(), "0-0, Deuce");
}

#[test]
fn test_score_advantage_player_1() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 3);
    tennis_match = helpers::win_n_points(tennis_match, "player 2", 3);
    tennis_match.point_won_by("player 1".to_string());
    assert_eq!(tennis_match.score(), "0-0, Advantage player 1");
}

#[test]
fn test_player_1_wins_first_tennis_set() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 4);
    assert_eq!(tennis_match.score(), "1-0");
}

#[test]
fn test_player_1_wins_first_tennis_set_from_deuce() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 3);
    tennis_match = helpers::win_n_points(tennis_match, "player 2", 3);
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 2);
    assert_eq!(tennis_match.score(), "1-0");
}

#[test]
fn test_player_1_wins_first_two_tennis_sets() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 8);
    assert_eq!(tennis_match.score(), "2-0");
}

#[test]
fn test_initialize_tie_break() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::initialize_tie_break(tennis_match, "player 1", "player 2");
    assert_eq!(tennis_match.score(), "6-6");
}

#[test]
fn test_score_tie_break_1_0() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::initialize_tie_break(tennis_match, "player 1", "player 2");
    tennis_match.point_won_by("player 1".to_string());
    assert_eq!(tennis_match.score(), "6-6, 1-0");
}

#[test]
fn test_score_tie_break_3_3() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::initialize_tie_break(tennis_match, "player 1", "player 2");
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 3);
    tennis_match = helpers::win_n_points(tennis_match, "player 2", 3);
    assert_eq!(tennis_match.score(), "6-6, 3-3");
}

#[test]
fn test_win_tie_break_game() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::initialize_tie_break(tennis_match, "player 1", "player 2");
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 7);
    assert_eq!(tennis_match.score(), "7-6, player 1 wins");
}

#[test]
fn test_score_tie_break_7_6() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::initialize_tie_break(tennis_match, "player 1", "player 2");
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 6);
    tennis_match = helpers::win_n_points(tennis_match, "player 2", 6);
    tennis_match.point_won_by("player 1".to_string());
    assert_eq!(tennis_match.score(), "6-6, 7-6");
}

#[test]
fn test_win_tie_break_game_with_more_than_seven_points() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::initialize_tie_break(tennis_match, "player 1", "player 2");
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 6);
    tennis_match = helpers::win_n_points(tennis_match, "player 2", 6);
    tennis_match = helpers::win_n_points(tennis_match, "player 1", 2);
    assert_eq!(tennis_match.score(), "7-6, player 1 wins");
}

#[test]
fn test_win_consecutive_games() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::win_n_games(tennis_match, "player 2", 6);
    assert_eq!(tennis_match.score(), "0-6, player 2 wins");
}

#[test]
fn test_win_seven_five() {
    let mut tennis_match = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    tennis_match = helpers::win_n_games(tennis_match, "player 2", 5);
    tennis_match = helpers::win_n_games(tennis_match, "player 1", 7);
    assert_eq!(tennis_match.score(), "7-5, player 1 wins");
}
