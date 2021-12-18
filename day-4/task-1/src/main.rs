use std::fs::File;
use std::io::{BufRead, BufReader};

struct Gameboard {
    x: u32,
    y: u32,
    values: Vec<u32>,
}

impl Gameboard {
    fn build<B: BufRead>(reader: &mut B, x: u32, y: u32) -> Gameboard {
        let mut values: Vec<u32> = vec![];
        let total_values: u32 = x * y;

        for line in reader.lines() {
            let line = line.expect("Corrupted file!");
            if !line.eq("\n") {
                let mut parsed_values = line
                    .split(" ")
                    .filter(|value| !value.eq(&""))
                    .map(|value| value.trim().parse::<u32>().unwrap())
                    .collect();
                values.append(&mut parsed_values);
            }

            if values.len() as u32 == total_values {
                break;
            }
        }

        Gameboard {
            x: x,
            y: y,
            values: vec![],
        }
    }
}

fn read_lucky_numbers<B: BufRead>(reader: &mut B) -> Vec<u32> {
    let mut lucky_numbers = String::new();

    reader
        .read_line(&mut lucky_numbers)
        .expect("Corrupted file");

    let lucky_numbers = lucky_numbers
        .split(",")
        .map(|x| x.trim().replace(",", ""))
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    lucky_numbers
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("12,8,11,6,7,9", vec![12, 8, 11, 6, 7, 9])]
    #[test_case("10, 11, 2, 5, 1", vec![10, 11, 2, 5, 1])]
    fn read_lucky_numbers_return_vector(test_input: &str, expected: Vec<u32>) {
        let actual = read_lucky_numbers(&mut test_input.as_bytes());

        for (actual, expected) in actual.iter().zip(expected.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn build_gameboard() {
        let test_input = "10 11  9 19  4\n 9 11 4  5 23\n21 11 32 18 12\n22 19 11 12  4\n23 32 12 11  5\n\n10 11  9 19  4\n 8 11 4  5 23\n21 11 32 18 12\n22 19 11 12  4\n23 32 12 11  5";
        let actual = Gameboard::build(&mut test_input.as_bytes(), 5, 5);
        let expected = vec![
            10, 11, 9, 19, 4, 9, 11, 4, 5, 23, 21, 11, 32, 18, 12, 22, 19, 11, 12, 4, 23, 32, 12,
            11, 5,
        ];

        assert_eq!(actual.y, 5);
        assert_eq!(actual.x, 5);

        for (actual, expected) in actual.values.iter().zip(expected.iter()) {
            assert_eq!(actual, expected);
        }
    }
}
