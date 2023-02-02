use rand::Rng;

fn main() {
    let dice_roll = rand::thread_rng().gen_range(1, 7);

    match dice_roll {
        1 => println!("
          +-------+
          |       |
          |   *   |
          |       |
          +-------+"),
        2 => println!("
          +-------+
          | *     |
          |       |
          |     * |
          +-------+"),
        3 => println!("
          +-------+
          | *     |
          |   *   |
          |     * |
          +-------+"),
        4 => println!("
          +-------+
          | *   * |
          |       |
          | *   * |
          +-------+"),
        5 => println!("
          +-------+
          | *   * |
          |   *   |
          | *   * |
          +-------+"),
        6 => println!("
          +-------+
          | *   * |
          | *   * |
          | *   * |
          +-------+"),
        _ => println!("Invalid roll"),
    }
}

