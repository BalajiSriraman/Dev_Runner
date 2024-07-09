# Package Script Runner - **Dev_Runner**

A command-line interface (CLI) tool built in Rust that detects `package.json` files and provides an interactive prompt to run npm scripts.

## Installation

```bash
curl -s https://storage.googleapis.com/dev_runner/install.sh | bash
```
## Usage

The default alias for this tool is devr. To get more information about its usage, run:  `devr --help`

## Features

- Automatically detects `package.json` in the current directory
- Lists all available npm scripts
- Provides an interactive prompt for selecting and running scripts
- Easy-to-use interface
  
## Use Case and Compatibility
This tool fits my specific use case and has been tested to work on my machine and few containers. However, please note that it has not been extensively tested on all developer workloads or environments. Your mileage may vary depending on your specific setup and requirements.

## Issues and Support
If you encounter any issues or have suggestions for improvements, please don't hesitate to raise an issue on the project's repository. I'm happy to help you set up and resolve any problems you might face.

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

- This project was inspired by the need for a quick and easy way to run npm scripts from the command line.
- Built with Rust 🦀
