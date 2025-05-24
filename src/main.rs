// ARGS Parser
use clap::Parser;
use std::io::{self, Write}; // Import Write for writeln! and io::Result
use std::process::{Command, Stdio}; // Import Command and Stdio for child processes

#[derive(Parser, Debug)]
#[clap(author = "Maxwell Rupp", version, about)]
/// Application configuration
struct Args {
    /// a REQUIRED message to be passed as an argument
    #[arg()]
    message: Option<String>,
}

// Non-TUI Stuff
fn main() {
    let args = Args::parse();
    let message = match args.message {
        Some(msg) => msg,
        None => "No message provided".to_string(),
    };
    printing_with_lolcat(message).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
}

fn printing_with_lolcat(message: String) -> io::Result<()> {
    // --- LOLCAT INTEGRATION START ---

    // Get the path to the lolcat executable from the environment variable.
    // This variable is set by the Nix flake's `postInstall` phase using `wrapProgram`.
    let lolcat_path = match std::env::var("LOLCAT_PATH") {
        Ok(path) => path,
        Err(_) => {
            // Fallback or error if LOLCAT_PATH is not set (e.g., when run outside Nix)
            eprintln!("Error: LOLCAT_PATH environment variable not set.");
            eprintln!("Please ensure this application is run within its Nix environment (e.g., `nix run .`).");
            std::process::exit(1); // Exit if lolcat path isn't found
        }
    };

    // 1. Spawn the `lolcat` child process.
    //    - `stdin(Stdio::piped())`: We'll write the `message` to lolcat's input.
    //    - `stdout(Stdio::inherit())`: lolcat's output (the rainbow-colored text)
    //      should go directly to the user's terminal.
    //    - `stderr(Stdio::inherit())`: Any errors from lolcat will also go
    //      directly to the user's terminal.
    let mut child = Command::new(&lolcat_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?; // Use `?` to propagate any spawning errors

    // 2. Get the stdin handle of the child process.
    //    `take()` is used because `stdin` is an `Option<ChildStdin>`.
    let mut stdin = child.stdin.take().expect("Failed to open stdin for lolcat");

    // 3. Write the message to lolcat's stdin.
    writeln!(stdin, "{}", message)?; // Use writeln! to ensure a newline

    // 4. Crucially, close stdin to signal EOF to lolcat.
    //    If stdin is not closed, lolcat will keep waiting for more input
    //    and won't process the buffered text or exit.
    drop(stdin);

    // 5. Wait for the lolcat process to finish and get its exit status.
    let status = child.wait()?;

    // 6. Check if lolcat executed successfully.
    if !status.success() {
        eprintln!("lolcat exited with an error: {:?}", status);
        // Optionally, return an error from main if lolcat failed
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("lolcat failed with status: {:?}", status),
        ));
    }

    // --- LOLCAT INTEGRATION END ---

    Ok(()) // Indicate successful execution
}
