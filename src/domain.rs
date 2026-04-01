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
