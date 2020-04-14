use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut pool = HashMap::new();
    let filename = &env::args().collect::<Vec<String>>()[1];
    let file = File::open(filename).unwrap();
    let buffer = io::BufReader::new(file);

    let mut output = String::new();

    for line in buffer.lines() {
        let num = line.unwrap().parse::<u32>().unwrap();
        let mut cached_result = pool.get(&num);
        let prime;

        if cached_result.is_none() {
            prime = is_prime(num);
            pool.insert(num, prime);
            cached_result = Some(&prime);
        }

        output.push_str(&cached_result.unwrap().to_string());
        output.push('\n');
    }

    print!("{}", output)
}

fn is_prime(num: u32) -> u8 {
    if primal::is_prime(num as u64) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_test() {
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

    #[test]
    fn small_test() {
        let numbers = vec![3, 7, 9, 4, 2];
        let expected = vec![1, 1, 0, 0, 1];

        for (i, num) in numbers.iter().enumerate() {
            let result = is_prime(*num);

            assert_eq!(result, expected[i]);
        }
    }
}