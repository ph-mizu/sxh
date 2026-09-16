use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("this is mizu from laptop ts:100.64.0.0");
    println!("{args:?}");
}