use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).unwrap();
    let buffer = io::BufReader::new(file);

    for line in buffer.lines() {
        let num = line.unwrap().parse().unwrap();
        println!("{}", is_prime(num));
    }
}

fn is_prime(num: u64) -> u8 {
    if primal::is_prime(num) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = File::open("input.txt").unwrap();
        let input_buffer = io::BufReader::new(file);

        let file = File::open("expected.txt").unwrap();
        let expected_buffer = io::BufReader::new(file);
        let mut expected_lines = expected_buffer.lines();

        for line in input_buffer.lines() {
            let num = line.unwrap().parse().unwrap();
            let result = is_prime(num);

            assert_eq!(result, expected_lines.next().unwrap().unwrap().parse().unwrap());
        }
    }
}