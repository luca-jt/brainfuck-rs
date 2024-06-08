use std::env;
use std::fs;
use std::io::{Read, stdin, stdout, Write};

const CELL_LENGTH: usize = 30000;

fn main() {
    // set up state
    let mut cell: Vec<u8> = vec![0; CELL_LENGTH];
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

    let mut io_lock = stdout().lock();

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
                ptr = (ptr + 1) % CELL_LENGTH;
            }
            '<' => {
                if ptr == 0 {
                    ptr = CELL_LENGTH - 1;
                } else {
                    ptr -= 1;
                }
            }
            '.' => {
                write!(io_lock, "{}", cell[ptr] as char).unwrap();
            }
            ',' => {
                stdin().read_exact(&mut [cell[ptr]]).unwrap();
            }
            '[' => {
                if cell[ptr] == 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        i += 1;
                        if chars[i] == '[' {
                            loop_count += 1;
                        } else if chars[i] == ']' {
                            loop_count -= 1;
                        }
                    }
                }
            }
            ']' => {
                if cell[ptr] != 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        i -= 1;
                        if chars[i] == '[' {
                            loop_count -= 1;
                        } else if chars[i] == ']' {
                            loop_count += 1;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    io_lock.flush().unwrap();
}
