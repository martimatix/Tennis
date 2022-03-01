const TIE_BREAK_SET_VALUE: u8 = 6;
const MIN_DEUCE_POINTS: usize = 3;
const MIN_POINTS_TO_WIN_REGULAR_GAME: usize = 4;
const MIN_MARGIN_TO_WIN_GAME: usize = 2;
const MIN_POINTS_TO_WIN_TIE_BREAK_GAME: usize = 7;
const MIN_MARGIN_TO_WIN_SET: u8 = 2;
const MIN_GAMES_TO_WIN_SET: u8 = 6;
const TIE_BREAK_VICTORY_GAMES: u8 = 7;

pub struct Match<'a> {
    player_1: Player<'a>,
    player_2: Player<'a>,
}

impl<'a> Match<'a> {
    pub fn new(player_1_name: &'a str, player_2_name: &'a str) -> Match<'a> {
        Match {
            player_1: Player::new(player_1_name),
            player_2: Player::new(player_2_name),
        }
    }

    pub fn point_won_by(&mut self, player_name: &str) -> &Match {
        let (match_ended, _) = Match::match_ended(&self);
        if match_ended {
            return self;
        }

        let is_tie_break = self.is_tie_break();

        let (winner, loser) = if player_name == self.player_1.name {
            (&mut self.player_1, &mut self.player_2)
        } else {
            (&mut self.player_2, &mut self.player_1)
        };

        winner.point_won();

        let min_points_to_win_game = if is_tie_break {
            MIN_POINTS_TO_WIN_TIE_BREAK_GAME
        } else {
            MIN_POINTS_TO_WIN_REGULAR_GAME
        };

        if winner.game_score >= min_points_to_win_game
            && winner.game_score - loser.game_score >= MIN_MARGIN_TO_WIN_GAME
        {
            winner.game_won();
            loser.game_lost();
        }

        self
    }

    pub fn score(self) -> String {
        let (match_ended, winner_name) = Match::match_ended(&self);
        let set_score = format!("{}-{}", self.player_1.set_score, self.player_2.set_score);

        if match_ended {
            let win_text = format!("{} wins", winner_name);
            return format!("{}, {}", set_score, win_text);
        }

        let game_score = if self.is_tie_break() {
            self.tie_break_game_score()
        } else {
            self.regular_game_score()
        };

        if (self.player_1.game_score + self.player_2.game_score) == 0 {
            set_score
        } else {
            format!("{}, {}", set_score, game_score)
        }
    }

    fn tie_break_game_score(&self) -> String {
        format!("{}-{}", self.player_1.game_score, self.player_2.game_score)
    }

    fn regular_game_score(&self) -> String {
        if self.before_deuce() {
            format!(
                "{}-{}",
                Match::game_to_tennis_score(self.player_1.game_score),
                Match::game_to_tennis_score(self.player_2.game_score)
            )
        } else {
            if self.player_1.game_score == self.player_2.game_score {
                "Deuce".to_string()
            } else if self.player_1.game_score > self.player_2.game_score {
                format!("Advantage {}", self.player_1.name)
            } else {
                format!("Advantage {}", self.player_2.name)
            }
        }
    }

    fn before_deuce(&self) -> bool {
        self.player_1.game_score <= MIN_DEUCE_POINTS
            && self.player_2.game_score <= MIN_DEUCE_POINTS
            && self.player_1.game_score + self.player_2.game_score < (2 * MIN_DEUCE_POINTS)
    }

    fn game_to_tennis_score(game_score: usize) -> String {
        match game_score {
            0 => "0".to_string(),
            1 => "15".to_string(),
            2 => "30".to_string(),
            3 => "40".to_string(),
            _ => panic!("Invalid game score"),
        }
    }

    fn is_tie_break(&self) -> bool {
        self.player_1.set_score == TIE_BREAK_SET_VALUE
            && self.player_2.set_score == TIE_BREAK_SET_VALUE
    }

    fn match_ended(&self) -> (bool, &str) {
        if Match::has_won_match(self.player_1.set_score, self.player_2.set_score) {
            (true, self.player_1.name)
        } else if Match::has_won_match(self.player_2.set_score, self.player_1.set_score) {
            (true, self.player_2.name)
        } else {
            (false, "")
        }
    }

    fn has_won_match(potential_winner_set_score: u8, other_player_set_score: u8) -> bool {
        potential_winner_set_score > other_player_set_score
            && potential_winner_set_score - other_player_set_score >= MIN_MARGIN_TO_WIN_SET
            && potential_winner_set_score >= MIN_GAMES_TO_WIN_SET
            || potential_winner_set_score == TIE_BREAK_VICTORY_GAMES
    }
}

struct Player<'a> {
    name: &'a str,
    game_score: usize,
    set_score: u8,
}

impl Player<'_> {
    fn new(name: &str) -> Player {
        Player {
            name,
            game_score: 0,
            set_score: 0,
        }
    }

    fn point_won(&mut self) {
        self.game_score += 1;
    }

    fn game_won(&mut self) {
        self.game_score = 0;
        self.set_score += 1;
    }

    fn game_lost(&mut self) {
        self.game_score = 0;
    }
}
