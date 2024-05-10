use serde::{de::DeserializeOwned, Serialize};

use crate::stats::SingleGameTermination;

pub trait GameCountingContainer: Serialize + DeserializeOwned + Default + Send + Sync {
    fn increment(&mut self, term: SingleGameTermination);
}
