use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};

use crate::{
    generic_stats::GameCountingContainer,
    stats::{GameId, SingleGameTermination},
};

struct StatsVisitor {
    last_move: SanPlus,
    game_did_start: bool,
    white_to_move: bool,
    data: SingleGameTermination,
}

impl StatsVisitor {
    fn new() -> StatsVisitor {
        StatsVisitor {
            game_did_start: false,
            white_to_move: true,
            last_move: SanPlus::from_ascii(b"e4").unwrap(),
            data: SingleGameTermination::default(),
        }
    }
}

impl Visitor for StatsVisitor {
    type Result = SingleGameTermination;

    fn begin_game(&mut self) {
        self.white_to_move = true;
    }

    fn san(&mut self, san_plus: SanPlus) {
        self.last_move = san_plus;
        self.game_did_start = true;
        self.white_to_move = !self.white_to_move;
    }
    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        match key {
            b"UTCDate" => {
                let value = value.decode_utf8().unwrap();
                self.data.date = value.to_string();
            }
            b"Opening" => {
                self.data.opening_name = value.decode_utf8().unwrap().to_string();
            }
            b"Termination" => {
                self.data.termination_string = value.decode_utf8().unwrap().to_string();
            }
            b"WhiteElo" => {
                self.data.white_elo = value.decode_utf8().unwrap().parse().unwrap_or_default();
            }
            b"BlackElo" => {
                self.data.black_elo = value.decode_utf8().unwrap().parse().unwrap_or_default();
            }
            b"Site" => {
                self.data.id = GameId(
                    value
                        .decode_utf8()
                        .unwrap()
                        .split("https://lichess.org/")
                        .skip(1)
                        .next()
                        .unwrap()
                        .to_string(),
                )
            }
            _ => {}
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn end_game(&mut self) -> Self::Result {
        self.data.final_move = self.last_move.to_string();
        self.data.final_move_by_white = !self.white_to_move; // now the losing side is to move but is checkmated
        self.data.clone()
    }
}

pub fn visit_reader<T: GameCountingContainer>(v: impl std::io::Read) -> anyhow::Result<T> {
    let reader = BufferedReader::new(v);
    let mut visitor = StatsVisitor::new();
    println!("Starting iteration");
    // let mut stats = TerminationStats::default();
    let mut stats = T::default();
    for game in reader.into_iter(&mut visitor) {
        let term = game.unwrap();
        // println!("{game:?}");
        stats.increment(term);
    }

    Ok(stats)
}
