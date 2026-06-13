use crate::state::GameState;
use crate::store::catalog;

const COMMANDS: [(&str, &str); 8] = [
    ("help", "Show this help message"),
    ("exit", "Terminate session"),
    ("clear", "Clear screen"),
    ("tick", "Advance game day by one"),
    ("status", "Show current game state"),
    ("credits", "Show current balance"),
    ("inventory", "Show current inventory"),
    ("store", "List items for sale"),
];

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Exit,
    Clear,
    Empty,
    Buy(String),
    InvalidUsage(String),
    Unknown(String),
    Tick,
    Status,
    Credits,
    Inventory,
    Store,
}

#[derive(Debug, PartialEq)]
pub enum LoopAction {
    Continue,
    Exit,
}

pub fn parse(input: &str) -> Command {
    let mut parts = input.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some("buy"), Some(item)) => Command::Buy(item.into()),
        (Some("buy"), None) => Command::InvalidUsage("buy".into()),
        (Some("help"), _) => Command::Help,
        (Some("exit"), _) => Command::Exit,
        (Some("clear"), _) => Command::Clear,
        (Some("tick"), _) => Command::Tick,
        (Some("status"), _) => Command::Status,
        (Some("credits"), _) => Command::Credits,
        (Some("inventory"), _) => Command::Inventory,
        (Some("store"), _) => Command::Store,
        (Some(other), _) => Command::Unknown(other.into()),
        (None, _) => Command::Empty,
    }
}

pub fn dispatch(command: Command, state: &mut GameState) -> LoopAction {
    match command {
        Command::Help => {
            print_help();
            LoopAction::Continue
        }
        Command::Exit => LoopAction::Exit,
        Command::Clear => {
            println!("\x1B[2J\x1B[1;1H");
            LoopAction::Continue
        }
        Command::Unknown(input) => {
            println!("[ALERT] UNRECOGNIZED COMMAND: {input}");
            LoopAction::Continue
        }
        Command::Empty => LoopAction::Continue,
        Command::Buy(item) => {
            println!("[STORE] PURCHASE REQUEST: {item} (NOT IMPLEMENTED YET)");
            LoopAction::Continue
        }
        Command::InvalidUsage(cmd) => {
            println!("[ALERT] INVALID USAGE: {cmd} <argument>");
            LoopAction::Continue
        }
        Command::Tick => {
            state.advance_day();
            println!("[CLOCK] DAY ADVANCED TO {}", state.day());
            LoopAction::Continue
        }
        Command::Credits => {
            println!("[WALLET] BALANCE: {} CR", state.credits());
            LoopAction::Continue
        }
        Command::Status => {
            println!("{state}");
            LoopAction::Continue
        }
        Command::Inventory => {
            print_inventory(state);
            LoopAction::Continue
        }
        Command::Store => {
            print_store();
            LoopAction::Continue
        }
    }
}

fn print_store() {
    println!("STORE CATALOG:");
    for item in catalog() {
        println!("{item}");
    }
}

fn print_inventory(state: &GameState) {
    let grouped_items = state.inventory().counts();

    if grouped_items.is_empty() {
        println!("INVENTORY EMPTY");
        return;
    }

    let mut summary: Vec<String> = Vec::new();
    for (item, count) in grouped_items {
        summary.push(format!("{item} x{count}"));
    }
    println!("INVENTORY SUMMARY: ");
    println!("{}", summary.join(" / "));
}

fn print_help() {
    println!(
        "[MAINFRAME {}] AVAILABLE OPERATIONS:",
        env!("CARGO_PKG_VERSION")
    );
    for (name, description) in COMMANDS {
        println!("   {name:<10}   {description}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_help_returns_help() {
        assert_eq!(parse("help"), Command::Help);
    }

    #[test]
    fn parse_exit_returns_exit() {
        assert_eq!(parse("exit"), Command::Exit);
    }

    #[test]
    fn parse_tick_returns_tick() {
        assert_eq!(parse("tick"), Command::Tick);
    }

    #[test]
    fn parse_status_returns_status() {
        assert_eq!(parse("status"), Command::Status);
    }

    #[test]
    fn parse_strips_whitespace_around_all_commands() {
        assert_eq!(parse("   help   "), Command::Help);
        assert_eq!(parse("exit   "), Command::Exit);
        assert_eq!(parse("clear   "), Command::Clear);
        assert!(matches!(
            parse("   unknown_command   "),
            Command::Unknown(_)
        ));
        assert_eq!(parse("   "), Command::Empty);
    }

    #[test]
    fn parse_unknown_command() {
        assert!(matches!(parse("xyz"), Command::Unknown(_)));
    }

    #[test]
    fn parse_empty_returns_empty() {
        assert_eq!(parse(""), Command::Empty);
        assert_eq!(parse("\t\t\t"), Command::Empty);
    }

    #[test]
    fn parse_only_first_token_matters() {
        assert_eq!(parse("help me"), Command::Help);
        assert_eq!(parse("exit this"), Command::Exit);
        assert_eq!(parse("clear all"), Command::Clear);
    }

    #[test]
    fn parse_buy_with_item_returns_buy() {
        assert_eq!(parse("buy sword"), Command::Buy("sword".into()));
    }

    #[test]
    fn parse_buy_without_item_returns_invalid_usage() {
        assert_eq!(parse("buy"), Command::InvalidUsage("buy".into()));
    }

    #[test]
    fn dispatch_exit_returns_loop_exit() {
        let mut state = GameState::new();
        assert_eq!(dispatch(Command::Exit, &mut state), LoopAction::Exit);
    }

    #[test]
    fn dispatch_help_returns_loop_continue() {
        let mut state = GameState::new();
        assert_eq!(dispatch(Command::Help, &mut state), LoopAction::Continue);
    }

    #[test]
    fn dispatch_tick_returns_loop_continue() {
        let mut state = GameState::new();
        assert_eq!(dispatch(Command::Tick, &mut state), LoopAction::Continue);
    }

    #[test]
    fn dispatch_tick_increments_day() {
        let mut state = GameState::new();
        let current_day = state.day();
        dispatch(Command::Tick, &mut state);
        assert_eq!(state.day(), current_day + 1);
    }

    #[test]
    fn dispatch_status_returns_loop_continue() {
        let mut state = GameState::new();
        assert_eq!(dispatch(Command::Status, &mut state), LoopAction::Continue);
    }

    #[test]
    fn parse_credits_returns_credits() {
        assert_eq!(parse("credits"), Command::Credits);
    }

    #[test]
    fn dispatch_credits_returns_loop_continue() {
        let mut state = GameState::new();
        assert_eq!(dispatch(Command::Credits, &mut state), LoopAction::Continue);
    }

    #[test]
    fn parse_inventory_returns_inventory() {
        assert_eq!(parse("inventory"), Command::Inventory);
    }

    #[test]
    fn dispatch_inventory_returns_loop_continue() {
        let mut state = GameState::new();
        assert_eq!(
            dispatch(Command::Inventory, &mut state),
            LoopAction::Continue
        );
    }

    #[test]
    fn parse_store_returns_store() {
        assert_eq!(parse("store"), Command::Store);
    }

    #[test]
    fn dispatch_store_returns_loop_continue() {
        let mut state = GameState::new();
        assert_eq!(dispatch(Command::Store, &mut state), LoopAction::Continue);
    }
}
