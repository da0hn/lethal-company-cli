#[derive(Debug)]
pub struct GameState {
    credits: u32,
    day: u16,
    current_planet: Option<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            credits: 100,
            day: 1,
            current_planet: None,
        }
    }

    pub fn credits(&self) -> u32 {
        self.credits
    }

    pub fn day(&self) -> u16 {
        self.day
    }

    pub fn current_planet(&self) -> Option<&str> {
        self.current_planet.as_deref()
    }

    pub fn advance_day(&mut self) {
        self.day += 1;
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_initial_state() {
        let state = GameState::new();
        assert_eq!(state.credits(), 100);
        assert_eq!(state.day(), 1);
        assert_eq!(state.current_planet(), None);
    }

    #[test]
    fn default_returns_initial_state() {
        let state = GameState::default();
        assert_eq!(state.credits(), 100);
        assert_eq!(state.day(), 1);
        assert_eq!(state.current_planet(), None);
    }

    #[test]
    fn advance_day_increments_day_by_one() {
        let mut state = GameState::new();
        let current_day = state.day();
        state.advance_day();
        assert_eq!(state.day(), current_day + 1);
    }
}
