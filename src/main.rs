use std::env;
use std::fs;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    // set up state
    let mut cell: Vec<u8> = vec![0; 30000];
    let mut ptr: usize = 0;

    // read input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide exactly one input file as an argument.");
    }
    let chars: Vec<char> = fs::read(&args[1])
        .unwrap()
        .iter()
        .map(|byte| *byte as char)
        .collect();

    // interpret file
    let mut i: usize = 0;
    while i < chars.len() {
        match chars[i] {
            '+' => {
                cell[ptr] = cell[ptr].wrapping_add(1);
            }
            '-' => {
                cell[ptr] = cell[ptr].wrapping_sub(1);
            }
            '>' => {
                ptr += 1;
            }
            '<' => {
                ptr -= 1;
            }
            '.' => {
                stdout().write(&[cell[ptr]]).unwrap();
            }
            ',' => {
                stdin().read_exact(&mut [cell[ptr]]).unwrap();
            }
            '[' => {
                if cell[ptr] == 0 {
                    while chars[i] != ']' {
                        i += 1;
                    }
                }
            }
            ']' => {
                if cell[ptr] != 0 {
                    while chars[i] != '[' {
                        i -= 1;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    stdout().flush().unwrap();
}
