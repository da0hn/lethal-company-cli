#[derive(Debug)]
pub struct GameState {
    pub credits: u32,
    pub day: u16,
    pub current_planet: Option<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            credits: 100,
            day: 1,
            current_planet: None,
        }
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
        assert_eq!(state.credits, 100);
        assert_eq!(state.day, 1);
        assert_eq!(state.current_planet, None);
    }

    #[test]
    fn default_returns_initial_state() {
        let state = GameState::default();
        assert_eq!(state.credits, 100);
        assert_eq!(state.day, 1);
        assert_eq!(state.current_planet, None);
    }
}
