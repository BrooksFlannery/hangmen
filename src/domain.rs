pub struct WordSelectionData {
    pub filler: i32,
}

pub struct GuessingData {
    pub filler: i32,
}

pub enum RoundState {
    WordSelection(WordSelectionData),
    Guessing(GuessingData),
    Finished(RoundResult),
}

pub enum RoundResult {
    Won(MatchParticipant),
    Draw,
}

pub struct MatchData {
    pub current_round: RoundState,
    pub player_one: MatchParticipant,
    pub player_two: MatchParticipant,
    pub match_score: MatchScore,
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

pub struct PlayerProfile {
    pub name: String,
}

pub struct MatchParticipant {
    pub player: PlayerProfile,
}