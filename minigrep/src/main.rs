use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    for ar in arguments{
        println!("{}", ar);
    }
}
