use colored::{ColoredString, Colorize};
use std::process::Command;
struct SubmissionResponse {
    on_init_messages: Vec<ColoredString>,
    on_failure_messages: Vec<ColoredString>,
    on_success_messages: Vec<ColoredString>,
}

impl SubmissionResponse {
    fn new() -> SubmissionResponse {
        SubmissionResponse {
            on_init_messages: vec![
                "".into(),
                "Welcome back! The initial build may take some time but the future builds will be faster ⚡️".green(),
                "".into(),
            ],
            on_failure_messages: vec![
                "".into(),
                "Test failed.".red(),
                "".into(),
            "Please view our article on debugging test failures: https://dotcodeschool.com/docs/debugging-test-failures".into(),
            ],
            on_success_messages: vec![
                "All tests passed.".green(),
                "".into(),
                "Great job! 🎉".into(),
                "".into(),
                "Move to the next step: https://dotcodeschool.com/courses/rust-state-machine".into()
            ],
        }
    }

    fn print_on_failure_messages(&self) {
        for message in &self.on_failure_messages {
            println!("{}", message);
        }
    }

    fn print_on_init_messages(&self) {
        for message in &self.on_init_messages {
            println!("{}", message);
        }
    }

    fn print_on_success_messages(&self) {
        for message in &self.on_success_messages {
            println!("{}", message);
        }
    }
}

fn test_command() -> Result<SubmissionResponse, SubmissionResponse> {
    let response = SubmissionResponse::new();
    response.print_on_init_messages();
    
    // TODO: build user's code
    // 
    // logs from the build process should be printed like
    // 
    // [build] Starting build...
    // [build] If you don't see logs for 60s+, please contact us at batman@dotcodeschool.com
    // [build] >  Updating crates.io index
    // [build] >  Downloading crates
    // [build] >  Downloaded proc-macro2 v1.0.24
    // ...
    // [build] >  Compiling proc-macro2 v1.0.24
    // ...
    // [build] Build successful.
    
    println!("Running tests. Logs should appear shortly...");
    
    // run tests
    // 
    // logs from the test process should be printed like
    // [compile]   Compiling rust-state-machine v0.1.0 (/app)
    // [compile]   Finished release [optimized] target(s) in 0.33s
    // [compile] Compilation successful.
    // 
    // [tester::rsm_l2c3] Running tests for Lesson 2 - Chapter 3 (Creating a Balances Pallet)
    // [tester::rsm_l2c3] $ cargo run --quiet --release -- "$@"
    
    Err(response)
}

fn main() {
    println!("Initiating test run...");
    let result = test_command();
    match result {
        Ok(response) => {
            response.print_on_success_messages();
        }
        Err(e) => {
            e.print_on_failure_messages();
        }
    }
}
