#[derive(Default, Clone, Debug)]
pub struct SingleGameTermination {
    pub final_move: String,
    pub final_move_by_white: bool,
    pub date: (u16, u8, u8),
    pub opening_name: String,
    pub white_elo: u16,
    pub black_elo: u16,
    pub termination_string: String,
}
