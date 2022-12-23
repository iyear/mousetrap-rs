use std::io;

fn main() {
    println!("started_by_explorer: {}", mousetrap::started_by_explorer());

    println!("Press Enter to exit...");

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}
