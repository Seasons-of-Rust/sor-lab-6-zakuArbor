use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone)]
enum OPCODE {
    Halt = 99,
    Add = 1,
    Mult = 2,
}

impl PartialEq<i32> for OPCODE {
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other
    }
}

const OUTPUT: i32 = 19690720;

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .next() // Only take the first line
        .unwrap() // Unwrap the option of whether there was a next line
        .unwrap() // Unwrap the string result of the lines
        .split(',') // Split by commas
        .map(|number| {
            number
                .parse() // Parse the number
                .expect("could not parse number") // Panic with a message if it fails
        })
        .collect();

    //let mut numbers = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    'SEARCH: for noun in 1..=99 {
        for verb in 1..=99 {
            let mut list = numbers.clone();
            set_1202(&mut list, noun, verb);
            if run(&mut list) == OUTPUT {
                println!("{}", 100 * noun + verb);
                break 'SEARCH;
            }
        }
    }
}

fn run(list: &mut [i32]) -> i32 {
    let mut i: usize = 0;
    while i < list.len() {
        let opcode = match list.get(i) {
            Some(val) => *val,
            _ => panic!("Unable to retrieve opcode. Crash and burn to oblivion"),
        };
        if opcode == OPCODE::Halt as i32 {
            break;
        }
        let (a, b) = get_operands(list, i);
        if a < 0 || b < 0 {
            panic!("Unable to retrieve operand. Crash and burn to oblivion");
        }
        //println!("Operands: {}, {}", a, b);
        let answer_ref = match get_res_ref(list, i + 3) {
            Some(val) => val,
            _ => panic!(
                "Unable to access write element. Local position {}, Crash and burn to oblivion",
                i
            ),
        };

        if OPCODE::Add.eq(&opcode) {
            *answer_ref = a + b;
        } else if OPCODE::Mult.eq(&opcode) {
            *answer_ref = a * b;
        } else {
            panic!("INVALID OPCODE. Crash and Burn");
        }
        i += 4;
    }
    *list.get(0).unwrap()
}

fn set_1202(list: &mut [i32], noun: i32, verb: i32) {
    *list.get_mut(1).unwrap() = noun;
    *list.get_mut(2).unwrap() = verb;
}

fn get_res_ref(list: &mut [i32], addr: usize) -> Option<&mut i32> {
    let index = match list.get(addr) {
        Some(val) => *val,
        _ => return None,
    };
    //println!("get_res_ref: write index is {}", index);
    list.get_mut(index as usize)
}

/// given a position, will look at the position and traverse to the position to retrieve the value
/// i.e. numbers[numbers[address]]
fn get_val(list: &[i32], addr: usize) -> Option<&i32> {
    match list.get(addr) {
        Some(val) => list.get(*val as usize),
        _ => None,
    }
}

fn get_operands(list: &[i32], base_addr: usize) -> (i32, i32) {
    let a = *(get_val(list, base_addr + 1).unwrap_or(&-1));
    let b = *(get_val(list, base_addr + 2).unwrap_or(&-1));
    (a, b)
}
