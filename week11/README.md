# Product Satisfaction Ranking

This Rust project calculates and ranks products based on their satisfaction scores. The input is an Excel file containing product data, including sales and review scores. The output is a ranked list of products with their satisfaction scores on a scale of 0 to 10.


## Features

* Read product data from an Excel file
* Calculate satisfaction scores based on weighted sales and review scores
* Rank products by satisfaction score
* Print ranked product list with satisfaction scores

## Requirements

- Rust 1.54.0 or higher
- Cargo
- An Excel file containing product data (see the "Input Excel File Format" section below)

## Input Excel File Format

The input Excel file should have the following structure:
```
|  id  | sales | review_score |
|------|-------|--------------|
|   1  |  5000 |     80       |
|   2  |  3000 |     90       |
|   3  |  1000 |     75       |
|   4  |  7000 |     85       |
|   5  |  2000 |     65       |
```
- The first column (id) contains the product IDs as integers
- The second column (sales) contains the sales numbers as integers
- The third column (review_score) contains the review scores as floating-point numbers in the range of 0 to 100  
Make sure to name the sheet containing the data "products.xlsx".
## Usage

1. Clone the repository or download the source code
2. Place your input Excel file (e.g., products.xlsx) in the project directory
3. Open a terminal/command prompt and navigate to the project directory
4. Run the following command:
```
cargo run
```
The program will read the product data from the Excel file, calculate the satisfaction scores, rank the products, and print the ranked product list with satisfaction scores.

