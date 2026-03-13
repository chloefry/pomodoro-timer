// ============================================================
// Pomodoro Timer
// ============================================================

// --- Enums ---

/// Match on it throughout code to decide durations, display messages, etc. 
enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

// --- Structs ---

/// Struct will hold all timer settings. Pass a reference to the struct through functions.
struct Config {
    work_duration: u64,
    short_break: u64,
    long_break: u64,
    cycles: u32,
}

// --- Core Timer Logic ---

/// Counts down from the given number of seconds, updating the display each tick.
fn run_timer(duration: u64) {
    todo!()
}

/// Converts a raw number of seconds into a formatted "MM:SS" string.
fn format_time(seconds: u64) -> String {
    todo!()
}

// --- Session Management ---

/// Runs a single session (work, short break, or long break) based on the session type.
fn run_session(session_type: SessionType, config: &Config) {
    todo!()
}

/// Runs a full pomodoro cycle: work → break → work → break → long break.
fn run_pomodoro_cycle(config: &Config) {
    todo!()
}

// --- User Interaction ---

/// Parses command-line arguments and returns a Config.
fn parse_args() -> Config {
    todo!()
}

/// Prompts the user to continue after a session. Returns true if they want to keep going.
fn prompt_continue() -> bool {
    todo!()
}

// --- Notifications ---

/// Prints a message to the terminal and optionally plays a sound alert.
fn notify(message: &str) {
    todo!()
}

// --- Entry Point ---

fn main() {
    let config = parse_args();
    run_pomodoro_cycle(&config);
}

