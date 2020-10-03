use std::env;

use hr::hr;

fn main() {
    let args: Vec<String> = env::args().collect();
    hr(args)
}
