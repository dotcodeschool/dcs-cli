use colored::{ColoredString, Colorize};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

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

fn test_command() -> Result<SubmissionResponse, String> {
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

    let user_project_dir = "../dcs-rsm";
    let target_dir = "/tmp/dcs-rsm-target";
    let target_dir_exists = Path::new(target_dir).exists();
    if !target_dir_exists {
        println!("{} {}", "[build]".yellow(), "Starting build...".blue());
        println!(
            "{} {}",
            "[build]".yellow(),
            "If you don't see logs for 60s+, please contact us at batman@dotcodeschool.com".blue()
        );

        let build_output = Command::new("cargo")
            .arg("build")
            // .arg("--color=always")
            .arg("--release")
            .arg("--target-dir")
            .arg(target_dir)
            .current_dir(user_project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match build_output {
            Ok(mut build_log) => {
                let stdout = build_log.stdout.take().expect("Failed to capture stdout");
                let stderr = build_log.stderr.take().expect("Failed to capture stderr");

                BufReader::new(stdout).lines().for_each(|line| {
                    println!("{} > {}", "[build]".yellow().to_string(), line.unwrap());
                });

                BufReader::new(stderr).lines().for_each(|line| {
                    println!("{} > {}", "[build]".yellow().to_string(), line.unwrap());
                });

                if build_log.wait().unwrap().success() {
                    println!("{} {}", "[build]".yellow(), "Build successful.".green());
                } else {
                    return Err(format!("{} {}\n{} {}", "[build]".yellow(), "Build failed. Please check your code and try again.".red(), "[build]".yellow(), "If you think this is an error on our side, please let us know at batman@dotcodeschool.com.".red()));
                }
            }
            Err(e) => {
                return Err(format!("\n{}\n>\t{}\n\n{}", "Failed to build project:".red(), e, "If you think this is an error on our side, please let us know at batman@dotcodeschool.com.".red()));
            }
        }
    }

    println!("\nRunning tests. Logs should appear shortly...\n");

    // run tests
    //
    // logs from the test process should be printed like
    // [compile]   Compiling rust-state-machine v0.1.0 (/app)
    // [compile]   Finished release [optimized] target(s) in 0.33s
    // [compile] Compilation successful.
    //
    // [tester::rsm_l2c3] Running tests for Lesson 2 - Chapter 3 (Creating a Balances Pallet)
    // [tester::rsm_l2c3] $ cargo run --quiet --release -- "$@"
    println!(
        "{} {}",
        "[tester::rsm_l2c3]".yellow(),
        "Running tests for Lesson 2 - Chapter 3 (Creating a Balances Pallet)".blue()
    );
    println!("{} {}", "[tester::rsm_l2c3]".yellow(), "$ cargo run".blue());
    let mut compile_output = Command::new("cargo")
        .arg("run")
        // .arg("--color=always")
        .arg("--quiet")
        .arg("--release")
        .arg("--target-dir")
        .arg("/tmp/dcs-rsm-target")
        .current_dir(user_project_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match compile_output {
        Ok(mut compile_log) => {
            let stdout = compile_log.stdout.take().expect("Failed to capture stdout");
            let stderr = compile_log.stderr.take().expect("Failed to capture stderr");

            BufReader::new(stdout).lines().for_each(|line| {
                println!(
                    "{} {}",
                    "[your_program]".yellow().to_string(),
                    line.unwrap()
                );
            });

            BufReader::new(stderr).lines().for_each(|line| {
                println!(
                    "{} {}",
                    "[your_program]".yellow().to_string(),
                    line.unwrap()
                );
            });

            let joined_message = response
                .on_failure_messages
                .iter()
                .map(|message| message.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            Err(joined_message)
        }
        Err(_) => {
            return Err("Tests failed. Please check your code and try again."
                .red()
                .to_string());
        }
    }
}

fn main() {
    println!("");
    println!("Initiating test run...");
    let result = test_command();
    match result {
        Ok(response) => {
            response.print_on_success_messages();
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
