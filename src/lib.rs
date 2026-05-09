pub mod commands;
pub mod state;
pub mod ui;

pub fn run() {
    println!(
        "[CORP-OS v{}] BOOT SEQUENCE COMPLETE. DAILY QUOTA NOT YET MET.",
        env!("CARGO_PKG_VERSION")
    );
    println!("[COMPANY MAINFRAME] EMPLOYEE TERMINAL ONLINE - WELCOME.");
}
