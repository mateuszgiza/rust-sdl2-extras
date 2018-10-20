use std::time::Duration;

#[derive(Default)]
pub struct GameTime {
    pub elapsed: Duration
}

impl GameTime {
    pub fn new(elapsed: Option<Duration>) -> Self {
        GameTime {
            elapsed: match elapsed {
                Some(duration) => duration,
                None => Duration::from_nanos(0)
            }
        }
    }
}