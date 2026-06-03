use std::fmt;

#[derive(Debug, PartialEq)]
pub enum WalletError {
    InsufficientFunds { available: u32, requested: u32 },
    Overflow,
}

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

    pub fn add_credits(&mut self, amount: u32) -> Result<(), WalletError> {
        match self.credits.checked_add(amount) {
            Some(new_credits) => {
                self.credits = new_credits;
                Ok(())
            }
            None => Err(WalletError::Overflow),
        }
    }

    pub fn spend_credits(&mut self, amount: u32) -> Result<(), WalletError> {
        match self.credits.checked_sub(amount) {
            Some(new_credits) => {
                self.credits = new_credits;
                Ok(())
            }
            None => Err(WalletError::InsufficientFunds {
                available: self.credits,
                requested: amount,
            }),
        }
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

    #[test]
    fn add_100_credits_increments_credits_by_100() {
        let mut state = GameState::new();
        let result = state.add_credits(100);
        assert!(result.is_ok());
        assert_eq!(state.credits(), 200);
    }

    #[test]
    fn add_max_u32_credits_returns_overflow_error() {
        let mut state = GameState::new();
        let result = state.add_credits(u32::MAX);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), WalletError::Overflow);
    }

    #[test]
    fn spend_insufficient_credits_returns_insufficient_funds_error() {
        let mut state = GameState::new();
        let result = state.spend_credits(1000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            WalletError::InsufficientFunds {
                available: 100,
                requested: 1000
            }
        );
    }

    #[test]
    fn spend_10_credits_returns_ok() {
        let mut state = GameState::new();
        let result = state.spend_credits(10);
        assert!(result.is_ok());
        assert_eq!(state.credits(), 90);
    }
}
