use argh::FromArgs;
use std::process::{Command, Stdio};

const CURRENT_FLAKE_URL: &str = ".";

/// Absolute path to the devour-flake executable
///
/// We expect this environment to be set in Nix build and shell.
const DEVOUR_FLAKE: &str = env!("DEVOUR_FLAKE");

#[derive(FromArgs, Debug)]
/// Application configuration
struct Config {
    /// whether to be verbose
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// flake URL or github URL
    #[argh(positional, default = "CURRENT_FLAKE_URL.to_string()")]
    url: String,
}

type AppError = Box<dyn std::error::Error>;
type AppResult<T> = Result<T, AppError>;

fn main() -> AppResult<()> {
    let cfg = argh::from_env::<Config>();
    if cfg.verbose {
        println!("DEBUG {cfg:?}");
    }
    // TODO: Handle github urls, in particular PR urls
    println!("Running nixci on {}", cfg.url.to_string());
    // TODO: Run `nix flake lock --no-update-lock-file` and report on it.
    let output = Command::new(DEVOUR_FLAKE)
        .arg(cfg.url)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    if output.status.success() {
        // TODO: Strip devour-flake's follows-logging output
        let raw_output = String::from_utf8(output.stdout)?;
        let outs = raw_output.split_ascii_whitespace();
        if outs.clone().count() == 0 {
            println!("ERROR: No outputs produced by devour-flake");
            std::process::exit(1);
        } else {
            outs.for_each(|out| println!("out: {}", out));
            Ok(())
        }
    } else {
        println!("ERROR: devour-flake failed");
        std::process::exit(output.status.code().unwrap_or(1));
    }
}
