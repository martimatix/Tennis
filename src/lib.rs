const TIE_BREAK_SET_VALUE: u8 = 6;
const MIN_DEUCE_POINTS: usize = 3;
const MIN_POINTS_TO_WIN_REGULAR_GAME: usize = 4;
const MIN_MARGIN_TO_WIN_GAME: usize = 2;
const MIN_POINTS_TO_WIN_TIE_BREAK_GAME: usize = 7;

pub struct Match {
    player_1: Player,
    player_2: Player,
}

impl Match {
    pub fn new(player_1_name: String, player_2_name: String) -> Match {
        Match {
            player_1: Player::new(player_1_name),
            player_2: Player::new(player_2_name),
        }
    }

    pub fn point_won_by(&mut self, player_name: String) -> &Match {
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
        let game_score = if self.is_tie_break() {
            self.tie_break_game_score()
        } else {
            self.regular_game_score()
        };

        let set_score = format!("{}-{}", self.player_1.set_score, self.player_2.set_score);

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
}

struct Player {
    name: String,
    game_score: usize,
    set_score: u8,
}

impl Player {
    fn new(name: String) -> Player {
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
