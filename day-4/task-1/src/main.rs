use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq, Debug)]
enum Field {
    Unmarked(u32),
    Marked(u32),
    None,
}

#[derive(PartialEq, Debug)]
struct Gameboard {
    id: u32,
    rows: u32,
    columns: u32,
    data: Vec<Vec<Field>>,
}

impl Gameboard {
    fn build(id: u32, rows: u32, columns: u32, numbers: &mut Vec<Vec<u32>>) -> Gameboard {
        let mut data: Vec<Vec<Field>> = vec![];
        for values in numbers {
            let mut row: Vec<Field> = vec![];
            for &mut value in values {
                row.push(Field::Unmarked(value));
            }
            data.push(row.to_vec());
        }
        Gameboard{
            id, rows, columns, data,
        }
    }

    fn is_row_all_marked(row: Vec<Field>) -> bool {
        if row.iter().all(|x| match x {
            Field::Marked(_) => true,
            _ => false,
        }) {
            return true;
        } else {
            false
        }
    }
}

fn build_gameboards<T: BufRead>(reader: &mut T, rows: u32, columns: u32) -> Vec<Gameboard> {
    let mut counter = 0;
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    let mut gameboards: Vec<Gameboard> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Cannot read gameboard data, file may be corrupted!");
        if !line.eq("\n") && numbers.len() > 0 {
            gameboards.push(Gameboard::build(counter, rows, columns, &mut numbers));
            counter += 1;
            numbers = Vec::new();
        } else {
            let clean_numbers = parse_gameboard_data(&line);
            if clean_numbers.len() > 0 {
                numbers.push(clean_numbers.to_vec());
            }
        }
    }
    gameboards
}

fn parse_gameboard_data(data: &str) -> Vec<u32> {
    data.split(" ")
        .filter(|x| !x.eq(&""))
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect()
}

fn main() {
    let file = File::open("src/input.txt").expect("File cannot be opened!");
    let rows = 5;
    let columns = 5;
    let mut reader = BufReader::new(file);
    let mut lucky_numbers = String::new();
    reader.read_line(&mut lucky_numbers).expect("Cannot read lucky_numbers, file may be corrupted!");
    let mut gameboards = build_gameboards(&mut reader, rows, columns);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn gameboard_build_returns_valid_gameboard() {
        let columns = 2;
        let rows = 2;
        let id = 1;
        let mut data = vec![vec![1, 2], vec![3, 4]];
        let expected = Gameboard {
            id,
            columns,
            rows,
            data: vec![
                vec![Field::Unmarked(1), Field::Unmarked(2)],
                vec![Field::Unmarked(3), Field::Unmarked(4)],
            ]
        };

        let actual = Gameboard::build(id, rows, columns, &mut data);

        assert_eq!(actual, expected);
    }

    #[test_case("10 11 12 16 18 17" => vec![10, 11, 12, 16, 18, 17])]
    #[test_case(" 9  7 17  6" => vec![9, 7, 17, 6])]
    #[test_case(" 2 11  8\n\n" => vec![2, 11, 8])]
    fn parse_gameboard_data_returns_vector_of_int(input: &str) -> Vec<u32> {
        parse_gameboard_data(input)
    }

    #[test]
    fn build_gameboards_returns_vector_of_gameboards() {
        let mut input = "10  1 12 17\n 9  7 16 19\n\n12 16 15 14\n11 13  8 15\n\n".as_bytes();
        let actual = build_gameboards(&mut input, 2, 2);
        println!("{:?}", actual);

        assert_eq!(actual.len(), 2);
    }

    #[test_case([Field::Marked(10), Field::Marked(1), Field::Marked(2)].to_vec() => true)]
    #[test_case([Field::Marked(10), Field::Unmarked(1), Field::Marked(2)].to_vec() => false)]
    #[test_case([Field::Unmarked(1), Field::Unmarked(11), Field::Unmarked(2)].to_vec() => false)]
    fn is_row_all_marked_returns_expected(input: Vec<Field>) -> bool {
        Gameboard::is_row_all_marked(input)
    }

}
