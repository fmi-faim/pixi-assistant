use clap::{Parser, Subcommand};
use fs2::available_space;
use serde::Deserialize;
use std::path::Path;
use std::process::{Command, exit};

const BYTES_PER_GB: f64 = 1024.0 * 1024.0 * 1024.0;

#[derive(Parser)]
#[command(name = "pixi-assistant")]
#[command(about = "Helper tool for pixi operations", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check if sufficient space is available in pixi cache
    Check {
        /// Minimum required space in GB
        #[arg(long)]
        gb: f64,
    },
}

#[derive(Deserialize)]
struct PixiInfo {
    cache_dir: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { gb } => check_space(gb),
    }
}

fn check_space(min_gb: f64) {
    // Get pixi info
    let output = Command::new("pixi")
        .args(["info", "--json"])
        .output()
        .unwrap_or_else(|e| {
            eprintln!("Error: Failed to execute pixi info: {e}");
            exit(1);
        });

    if !output.status.success() {
        eprintln!("Error: Failed to run 'pixi info --json'");
        exit(1);
    }

    // Parse JSON
    let info: PixiInfo =
        serde_json::from_slice(&output.stdout).expect("Failed to parse pixi info JSON");

    let cache_path = Path::new(&info.cache_dir);

    // Retrieve the available disk space for the cache path,
    // exiting the process with an error message if the space cannot be determined.
    let available_bytes = available_space(cache_path).unwrap_or_else(|e| {
        eprintln!(
            "Error: Failed to get available space for {}: {}",
            cache_path.display(),
            e
        );
        exit(1);
    });

    let available_gb = available_bytes as f64 / BYTES_PER_GB;

    if available_gb >= min_gb {
        println!(
            "✓ Cache directory {} has sufficient space available ({:.2} GB)",
            info.cache_dir, available_gb
        );
        exit(0);
    } else {
        eprintln!(
            "✗ Cache directory {} has insufficient space ({:.2} GB), please set PIXI_CACHE_DIR to a location with at least {:.2} GB free.",
            info.cache_dir, available_gb, min_gb
        );
        exit(1);
    }
}
