use std::time::Duration;
use common::DeltaTime;

#[derive(Default)]
pub struct GameTime {
    pub delta: DeltaTime
}

impl GameTime {
    pub fn new() -> Self {
        GameTime::default()
    }

    pub fn set_delta(&mut self, delta: Duration) {
        self.delta = DeltaTime { elapsed: delta }
    }
}