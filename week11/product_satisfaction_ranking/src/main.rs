use calamine::{open_workbook, Reader, Xlsx};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Product {
    id: u32,
    sales: u32,
    review_score: f64,
    satisfaction_score: f64,
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.satisfaction_score.partial_cmp(&other.satisfaction_score)
    }
}

impl Eq for Product {}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn calculate_satisfaction_score(sales: u32, review_score: f64) -> f64 {
    let sales_weight = 0.6;
    let review_weight = 0.4;
    let normalized_sales = sales as f64 / 10000.0;
    let weighted_sales = normalized_sales * sales_weight;
    let weighted_review_score = review_score * review_weight;
    let satisfaction_score = (weighted_sales + weighted_review_score) * 10.0;
    satisfaction_score
}

fn read_products_from_excel<P: AsRef<Path>>(path: P) -> Vec<(u32, u32, f64)> {
    let mut products = Vec::new();
    let mut excel: Xlsx<_> = open_workbook(path).expect("Cannot open the Excel file");
    let range = excel
        .worksheet_range("Sheet1")
        .expect("Cannot find 'Sheet1'")
        .expect("Cannot read data from 'Sheet1'");

    for row in range.rows().skip(1) {
        let id = row[0].get_float().expect("Invalid product ID") as u32;
        let sales = row[1].get_float().expect("Invalid sales number") as u32;
        let review_score = row[2].get_float().expect("Invalid review score");
        products.push((id, sales, review_score));
    }

    products
}

fn main() {
    let input_file = "products.xlsx";
    let products = read_products_from_excel(input_file);

    let mut product_heap = BinaryHeap::new();
    for (id, sales, review_score) in products {
        let satisfaction_score = calculate_satisfaction_score(sales, review_score / 100.0);
        product_heap.push(Product {
            id,
            sales,
            review_score,
            satisfaction_score,
        });
    }

    println!("Product ranking by satisfaction score:");
    while let Some(product) = product_heap.pop() {
        println!(
            "ID: {}, Sales: {}, Review score: {:.2}%, Satisfaction score: {:.2}/10",
            product.id, product.sales, product.review_score, product.satisfaction_score
        );
    }
}
