use std::fmt;

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

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let location = self.current_planet().unwrap_or("ORBIT");
        writeln!(f, "{:<10} {} CR", "CREDITS:", self.credits)?;
        writeln!(f, "{:<10} {}", "DAY:", self.day)?;
        write!(f, "{:<10} {}", "LOCATION:", location)
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

    #[test]
    fn display_shows_credits_day_and_location() {
        let state = GameState::new();
        let output = format!("{state}");
        assert!(output.contains("CREDITS:"));
        assert!(output.contains("100 CR"));
        assert!(output.contains("DAY:"));
        assert!(output.contains("LOCATION:"));
        assert!(output.contains("ORBIT"));
    }
}
