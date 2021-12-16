use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_data<B: BufRead>(reader: &mut B) -> (Vec<u32>, Vec<[u32; 25]>) {
    let mut winning_numbers = String::new();
    let mut gameboards: Vec<[u32; 25]> = vec![];

    reader
        .read_line(&mut winning_numbers)
        .expect("Corrupted file");
    let winning_numbers = winning_numbers
        .split(",")
        .map(|x| x.trim().replace(",", ""))
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut counter = 0;

    for line in reader.lines() {
        let line = line.expect("Corrupted file!");
        println!("the line is {} dd", line);
        if line.eq("\n") {
            println!("empty line");
            counter += 1;
            gameboards[counter] = [0; 25];
            println!("dd{:?}", gameboards);
        } else {
            for (i, number) in line.split(" ").enumerate() {
                println!("{}", number);
                if !number.eq("\n") {
                    let number = match number.trim().parse() {
                        Ok(number) => number,
                        _ => continue,
                    };
                    gameboards[counter][i] = number;
                }
            }
        }
    }
    (winning_numbers, gameboards)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_data() {
        let test_input = "12,8,10,11,5,7,9\n\n10 11  9 19  4\n 9 11 4  5 23\n21 11 32 18 12\n22 19 11 12  4\n23 32 12 11  5\n\n10 11  9 19  4\n 8 11 4  5 23\n21 11 32 18 12\n22 19 11 12  4\n23 32 12 11  5";
        let (actual_numbers, actual_gameboards) = read_data(&mut test_input.as_bytes());
        let expected = vec![
            [
                10, 11, 9, 19, 4, 9, 11, 4, 5, 23, 21, 11, 32, 18, 12, 22, 19, 11, 12, 4, 23, 32,
                12, 11, 5,
            ],
            [
                10, 11, 9, 19, 4, 8, 11, 4, 5, 23, 21, 11, 32, 18, 12, 22, 19, 11, 12, 4, 23, 32,
                12, 11, 5,
            ],
        ];

        assert_eq!(
            actual_numbers,
            vec![12, 8, 10, 11, 5, 7, 9],
            "Winning numbers are not as expected!"
        );
        assert_eq!(
            actual_gameboards, expected,
            "Gameboards are not as expected!"
        );
    }
}
