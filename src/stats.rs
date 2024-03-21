use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
pub struct SingleGameTermination {
    pub final_move: String,
    pub final_move_by_white: bool,
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TerminationStats {
    pub by_date: HashMap<String, StatsByRatingBucket>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StatsByRatingBucket {
    pub ratings: HashMap<String, ByOpening>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ByOpening {
    pub opening: HashMap<String, StatsByTermination>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StatsByTermination {
    pub termination: HashMap<String, StatsByFinishingColor>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StatsByFinishingColor {
    pub white: CountsByMove,
    pub black: CountsByMove,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CountsByMove(HashMap<String, usize>);

impl TerminationStats {
    pub fn increment(&mut self, how: SingleGameTermination) {
        let white_rating_round = round_to_hundred(how.white_elo);
        let black_rating_round = round_to_hundred(how.black_elo);
        let rating = format!("{white_rating_round}-{black_rating_round}");

        let color = self
            .by_date
            .entry(how.date)
            .or_default()
            .ratings
            .entry(rating)
            .or_default()
            .opening
            .entry(how.opening_name)
            .or_default()
            .termination
            .entry(how.termination_string)
            .or_default();
        let my_color = if how.final_move_by_white {
            &mut color.white
        } else {
            &mut color.black
        };
        my_color
            .0
            .entry(how.final_move)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
}
