# Random News Headline Generator

This is a simple Rust program that fetches the top news headlines from the BBC News website and selects a random headline to display.

## Requirements

- Rust (version 1.56 or later)
- Cargo (the Rust package manager)

## Installation

To install the program, clone this repository and run the following command in the project directory:

```
cargo build --release
```
This will compile the program and create an executable binary in the target/release directory.

## Usage

To run the program, navigate to the project directory and run the following command:

```
cargo run
```
The program will make a GET request to the BBC News website and retrieve the HTML for the top news section. It will then use the select crate to find all elements with the "gs-c-promo-heading__title" class, which correspond to the top news headlines. The program will choose a random headline from the list and print its text and URL to the console.

