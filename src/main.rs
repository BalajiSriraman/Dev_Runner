use clap::Parser;
use serde_json::Value;

// Imports
use dev_runner::utils::cli_helpers::prompt_selecter;
use dev_runner::utils::execute::cli_execute;
use dev_runner::utils::package_json::package_json_handler;
use dev_runner::utils::runner::{file_picker, file_reader};

#[derive(Parser)]
struct Cli {
    /// The path to the package.json file [default: root(.)]
    #[clap(default_value = ".")]
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

    let npm_command = "npm run".to_string() + " " + &selected;

    cli_execute(&npm_command)
}
