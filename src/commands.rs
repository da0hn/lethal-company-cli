const COMMANDS: [(&str, &str); 3] = [
    ("help", "Show this help message"),
    ("exit", "Terminate session"),
    ("clear", "Clear screen"),
];

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Exit,
    Clear,
    Unknown(String),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum LoopAction {
    Continue,
    Exit,
}

pub fn parse(input: &str) -> Command {
    match input.split_whitespace().next() {
        Some("help") => Command::Help,
        Some("exit") => Command::Exit,
        Some("clear") => Command::Clear,
        Some(other) => Command::Unknown(other.into()),
        None => Command::Empty,
    }
}

pub fn dispatch(command: Command) -> LoopAction {
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
    }
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
    fn dispatch_exit_returns_loop_exit() {
        assert_eq!(dispatch(Command::Exit), LoopAction::Exit);
    }

    #[test]
    fn dispatch_help_returns_loop_continue() {
        assert_eq!(dispatch(Command::Help), LoopAction::Continue);
    }
}
