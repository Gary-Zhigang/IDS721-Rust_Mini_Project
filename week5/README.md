# Rust Keyword Extractor

This is a simple program written in Rust that extracts the most frequent words from a paragraph of text. It uses a basic word frequency algorithm to determine which words are most important.

## Usage

To use the program, run it from the command line and enter a paragraph of text when prompted. The program will then return a list of the five most frequently occurring words in the paragraph.

## Example

```bash
$ cargo run
Enter a paragraph:
The quick brown fox jumped over the lazy dog.
Keywords: ["the", "over", "jumped", "fox", "lazy"]
```

## Dependencies

This program uses the std library as well as the HashMap and io modules.

