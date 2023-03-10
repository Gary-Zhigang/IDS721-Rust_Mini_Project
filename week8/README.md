# Rust Crawler

This Rust project is a simple web crawler that retrieves the title and chapter names of a website using the `reqwest` and `select` crates.

## Getting Started

To use this project, you will need to have Rust installed on your computer. You can download Rust from the official website: https://www.rust-lang.org/tools/install.

Once you have Rust installed, you can clone this repository to your local machine:
```
git clone https://github.com/Gary-Zhigang/IDS721-Rust_Mini_Project.git
```
Then, navigate to the project directory:
```
cd week8/crawler
```
## Usage
To run the Rust crawler, simply execute the following command in your terminal:
```
cargo run
```
This will compile and run the Rust program. The program will retrieve the title and chapter names of the website specified in the code (https://nogibjj.github.io/rust-tutorial/chapter_1.html), and print them to the terminal output.

You can modify the URL to crawl by changing the url variable in the main.rs file.

## Dependencies
This project relies on the following Rust crates:

* reqwest (version 0.11)
* select (version 0.6)  

These dependencies are listed in the Cargo.toml file, and will be automatically downloaded and installed when you run cargo build or cargo run.