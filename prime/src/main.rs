#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::env;
use std::fs::File;
use std::io::Read;
use fxhash::FxHashMap;

fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let mut file = File::open(filename).unwrap();
    let mut lines = String::new();
    file.read_to_string(&mut lines).unwrap();

    let mut pool = FxHashMap::default();
    let mut output = String::new();

    let mut num;
    let mut cached_result;
    let mut prime;
    for line in lines.split_whitespace() {
        num = line.parse::<u32>().unwrap();
        cached_result = pool.get(&num);

        if cached_result.is_some() {
            output.push(*cached_result.unwrap());
        } else {
            prime = is_prime(num);
            pool.insert(num, prime);
            output.push(prime);
        }

        output.push('\n');
    }

    print!("{}", output)
}

fn is_prime(num: u32) -> char {
    if primal::is_prime(num as u64) {
        '1'
    } else {
        '0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_test() {
        let mut file = File::open("input.txt").unwrap();
        let mut input_lines = String::new();
        file.read_to_string(&mut input_lines).unwrap();

        let mut file = File::open("expected.txt").unwrap();
        let mut expected_lines = String::new();
        file.read_to_string(&mut expected_lines).unwrap();
        let mut expected_lines = expected_lines.split_whitespace();

        for line in input_lines.split_whitespace() {
            let num = line.parse().unwrap();
            let result = is_prime(num);

            assert_eq!(result, expected_lines.next().unwrap().parse().unwrap());
        }
    }

    #[test]
    fn small_test() {
        let numbers = vec![3, 7, 9, 4, 2];
        let expected = vec!['1', '1', '0', '0', '1'];

        for (i, num) in numbers.iter().enumerate() {
            let result = is_prime(*num);

            assert_eq!(result, expected[i]);
        }
    }
}