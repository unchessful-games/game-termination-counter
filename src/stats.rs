use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::generic_stats::GameCountingContainer;

#[derive(Default, Clone, Debug)]
pub struct SingleGameTermination {
    pub final_move: String,
    pub final_move_by_white: bool,
    pub id: GameId,
    pub date: String,
    pub opening_name: String,
    pub white_elo: u16,
    pub black_elo: u16,
    pub termination_string: String,
}

fn round_to_hundred(v: u16) -> u16 {
    ((((v as f64) / 100.0).round()) * 100.0) as u16
}

#[cfg(test)]
mod test {
    use crate::stats::round_to_hundred;

    #[test]
    fn test_rounding() {
        assert_eq!(round_to_hundred(120), 100);
        assert_eq!(round_to_hundred(151), 200);
        assert_eq!(round_to_hundred(0), 0);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GameId(pub String);

/// The leaf structure that records games that match a particular property.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GameCounter {
    pub count: usize,
    pub exemplars: Vec<GameId>,
}

impl GameCountingContainer for GameCounter {
    fn increment(&mut self, term: SingleGameTermination) {
        self.count += 1;
        if self.exemplars.len() < 5 {
            self.exemplars.push(term.id);
        }
    }
}

/// Count games by their opening
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(bound = "T: Serialize+DeserializeOwned")]
pub struct ByOpening<T: GameCountingContainer>(pub HashMap<String, T>);

impl<T: GameCountingContainer> GameCountingContainer for ByOpening<T> {
    fn increment(&mut self, term: SingleGameTermination) {
        self.0
            .entry(term.opening_name.clone())
            .or_default()
            .increment(term)
    }
}
