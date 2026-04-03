use rand::RngExt;
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum Turn {
    PlayerOne,
    PlayerTwo,
}
pub struct WordSelectionData {
    pub turn: Turn,
    pub word_length: u8,
    pub input: String,
    pub player_one_word: Option<String>,
    pub player_two_word: Option<String>,
}

impl WordSelectionData {
    pub fn push_char(&mut self, c: char) {
        let c = c.to_ascii_lowercase();
        if c.is_ascii_alphabetic() && self.input.len() < self.word_length as usize {
            self.input.push(c);
        }
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn on_enter(&mut self) -> Option<GuessingData> {
        if self.input.len() == self.word_length as usize {
            match self.turn {
                Turn::PlayerOne => {
                    self.player_one_word = Some(self.input.clone());
                    self.turn = Turn::PlayerTwo;
                }
                Turn::PlayerTwo => {
                    self.player_two_word = Some(self.input.clone());
                    self.turn = Turn::PlayerOne;
                }
            }
            self.input.clear();
        }

        let (p1, p2) = (self.player_one_word.take(), self.player_two_word.take());
        match (p1, p2) {
            (Some(player_one_word), Some(player_two_word)) => Some(GuessingData {
                turn: self.turn,
                player_one_word,
                player_two_word,
                guessed_letters: HashSet::new(),
            }),
            (p1, p2) => {
                self.player_one_word = p1;
                self.player_two_word = p2;
                None
            }
        }
    }
}

pub struct GuessingData {
    pub turn: Turn,
    pub player_one_word: String,
    pub player_two_word: String,
    pub guessed_letters: HashSet<char>,
}

pub enum RoundState {
    WordSelection(WordSelectionData),
    Guessing(GuessingData),
    Finished(RoundResult),
}

impl RoundState {
    pub fn new(starting_turn: Turn) -> Self {
        RoundState::WordSelection(WordSelectionData {
            turn: starting_turn,
            word_length: rand::rng().random_range(3..=8),
            input: String::new(),
            player_one_word: None,
            player_two_word: None,
        })
    }
}

pub enum RoundResult {
    Won(MatchParticipant),
    Draw,
}

pub struct MatchData {
    pub player_one: MatchParticipant,
    pub player_two: MatchParticipant,
    pub match_score: MatchScore,
    pub current_round: RoundState,
}

impl MatchData {
    pub fn new() -> Self {
        Self {
            player_one: MatchParticipant {
                player: PlayerProfile {
                    name: String::from("Player One"),
                },
            },
            player_two: MatchParticipant {
                player: PlayerProfile {
                    name: String::from("Player Two"),
                },
            },
            match_score: MatchScore {
                player_one: 0,
                player_two: 0,
            },
            current_round: RoundState::new(Turn::PlayerOne),
        }
    }

    pub fn word_selection_push_char(&mut self, c: char) {
        if let RoundState::WordSelection(ws) = &mut self.current_round {
            ws.push_char(c);
        }
    }

    pub fn word_selection_backspace(&mut self) {
        if let RoundState::WordSelection(ws) = &mut self.current_round {
            ws.backspace();
        }
    }

    pub fn word_selection_enter(&mut self) {
        if let RoundState::WordSelection(ws) = &mut self.current_round
            && let Some(guessing) = ws.on_enter()
        {
            self.current_round = RoundState::Guessing(guessing);
        }
    }

    pub fn guessing_char(&mut self, c: char) {
        let c = c.to_ascii_lowercase();
        if !c.is_ascii_alphabetic() {
            return;
        }

        if let RoundState::Guessing(g_data) = &mut self.current_round {
            g_data.guessed_letters.insert(c);
            let player_one_won = word_fully_guessed(&g_data.guessed_letters, &g_data.player_two_word);
            let player_two_won = word_fully_guessed(&g_data.guessed_letters, &g_data.player_one_word);

            if player_one_won && player_two_won {
                self.current_round = RoundState::Finished(RoundResult::Draw);
            } else if player_one_won {
                self.match_score.player_one += 1;
                self.current_round = RoundState::Finished(RoundResult::Won(self.player_one.clone()));
            } else if player_two_won {
                self.match_score.player_two += 1;
                self.current_round = RoundState::Finished(RoundResult::Won(self.player_two.clone()));
            } else {
                g_data.turn = match g_data.turn {
                    Turn::PlayerOne => Turn::PlayerTwo,
                    Turn::PlayerTwo => Turn::PlayerOne,
                };
            }
        }
    }

    pub fn round_finished_enter(&mut self) {
        if let RoundState::Finished(_) = &self.current_round {
            self.current_round = RoundState::new(Turn::PlayerOne);
        }
    }
}

pub struct MatchScore {
    pub player_one: u8,
    pub player_two: u8,
}

pub struct MatchFinishedData {
    pub match_data: MatchData,
    pub winner: MatchParticipant,
}

pub enum MatchState {
    MatchInProgress(MatchData),
    MatchFinised(MatchFinishedData),
}

#[derive(Clone)]
pub struct PlayerProfile {
    pub name: String,
}

#[derive(Clone)]
pub struct MatchParticipant {
    pub player: PlayerProfile,
}

pub fn word_fully_guessed(guessed: &HashSet<char>, word: &str) -> bool {
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| guessed.contains(&c.to_ascii_lowercase()))
}
