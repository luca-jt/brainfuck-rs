use std::env;
use std::fs;

static ARRAY: Vec<u8> = vec![0; 30000];

fn main() {
    // read input
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        panic!("provide exactly one arg");
    }

    let file: Vec<char> = fs::read(&args[0])
        .unwrap()
        .iter()
        .map(|byte| *byte as char)
        .collect();

    let pointer: usize = 0;

    // interpret file
}
