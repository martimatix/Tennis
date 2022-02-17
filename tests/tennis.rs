extern crate tennis;

#[test]
fn test_score_0_0() {
    let game = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    assert_eq!(game.score(), "0-0");
}

#[test]
fn test_score_15_15() {
    let mut game = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 2".to_string());
    assert_eq!(game.score(), "0-0, 15-15");
}

#[test]
fn test_score_40_15() {
    let mut game = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    assert_eq!(game.score(), "0-0, 40-15");
}

#[test]
fn test_score_deuce() {
    let mut game = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 2".to_string());
    assert_eq!(game.score(), "0-0, Deuce");
}

#[test]
fn test_score_advantage_player_1() {
    let mut game = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 1".to_string());
    assert_eq!(game.score(), "0-0, Advantage player 1");
}

#[test]
fn test_player_1_wins_first_game() {
    let mut game = tennis::Match::new("player 1".to_string(), "player 2".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 2".to_string());
    game.point_won_by("player 1".to_string());
    game.point_won_by("player 1".to_string());
    assert_eq!(game.score(), "1-0");
}

#[test]
fn test_tie_break() {}

#[test]
fn player_1_wins_set() {}
