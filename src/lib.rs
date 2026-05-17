use commands::{LoopAction, dispatch, parse};
use state::GameState;
use std::io::{self, BufRead, Write};

pub mod commands;
pub mod state;
pub mod ui;

const COMPANY_NAME: &str = "THE CORP";

pub fn run() {
    let version = env!("CARGO_PKG_VERSION");
    println!("[{COMPANY_NAME}] INITIATING BOOT SEQUENCE...");
    println!("[{COMPANY_NAME}] BOOT SEQUENCE COMPLETE. DAILY QUOTA NOT YET MET.");
    println!("[MAINFRAME v{version}] EMPLOYEE TERMINAL ONLINE - WELCOME.");
    println!("[MAINFRAME v{version}] TYPE HELP FOR AVAILABLE COMMANDS");
    println!("{}", "-".repeat(60));

    let stdin = io::stdin();
    let mut input = String::new();

    let mut state = GameState::new();

    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("[ERROR] Failed to flush stdout");

        input.clear();

        stdin
            .lock()
            .read_line(&mut input)
            .expect("[ERROR] Failed to read stdin");

        if let LoopAction::Exit = dispatch(parse(input.trim()), &mut state) {
            break;
        }
    }
}
