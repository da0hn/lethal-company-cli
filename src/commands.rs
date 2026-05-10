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
