use clap::Parser;
use grrs::utils::execute::cli_execute;
use serde_json::Value;

// Imports
use grrs::utils::cli_helpers::prompt_selecter;
use grrs::utils::package_json::package_json_handler;
use grrs::utils::runner::{file_picker, file_reader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();

    // Picks the file based on the path
    let v_path = file_picker(args.path.to_str().unwrap()).unwrap();

    // Reads the file and returns the data
    let file_data = file_reader(&v_path);

    // Parses the data and returns the scripts object
    let scripts = match package_json_handler(&file_data) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if scripts == Value::Null {
        println!("No scripts found.");
        return;
    }

    let selected = match prompt_selecter(scripts) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("🔺 Error: {}", e);
            return;
        }
    };

    println!("⚙ Executing {} 🚀", selected);

    if let Err(e) = cli_execute(&selected) {
        eprintln!("Error: {}", e);
    }
}
