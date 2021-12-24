use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq, Debug)]
enum Field {
    Unmarked(u32),
    Marked(u32),
    None,
}

#[derive(PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
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

    fn is_row_all_marked(&self, row_index: usize) -> bool {
        match self.data.get(row_index) {
            Some(row) => {
                !row.iter().any(|x| match x {
                    Field::Unmarked(_) => true,
                    _ => false,
                })
            },
            _ => false,
        }
    }

    fn is_column_all_marked(&self, column_index: usize) -> bool {
        !self.data.iter().any(|row| match row.get(column_index) {
            Some(field) => match field {
                Field::Unmarked(_) => true,
                _ => false,
            },
            _ => false,
        })
    }

    fn check_number(&mut self, number: u32) -> Option<Position> {
        for (i, row) in self.data.iter_mut().enumerate() {
            for (j, field) in row.iter_mut().enumerate() {
                match field {
                    Field::Unmarked(value) => {
                        if *value == number {
                            *field = Field::Marked(*value);
                            return Option::Some(Position{x: i, y: j});
                        }
                    },
                    _ => continue,
                }
            }
        }

        Option::None
    }

    fn sum_unmarked(&self) -> u32 {
        self.data.iter()
            .flatten()
            .map(|field| match field {
                Field::Unmarked(value) => value,
                _ => &0,
            })
            .sum()
    }
}

fn build_gameboards<T: BufRead>(reader: &mut T, rows: u32, columns: u32) -> Vec<Gameboard> {
    let mut counter = 0;
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    let mut gameboards: Vec<Gameboard> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Cannot read gameboard data, file may be corrupted!");
        if line.eq("") && numbers.len() > 0 {
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
    let lucky_numbers = lucky_numbers
        .split(",")
        .map(|x| x.trim().parse::<u32>().unwrap());
    let mut gameboards = build_gameboards(&mut reader, rows, columns);

    'main: for lucky_number in lucky_numbers {
        for gameboard in gameboards.iter_mut() {
            match gameboard.check_number(lucky_number) {
                Some(position) => {
                    if gameboard.is_row_all_marked(position.x) {
                        for row in gameboard.data.iter() {
                            println!("{:?}", row);
                        }
                        let res = gameboard.sum_unmarked() * lucky_number;
                        println!("The final result is {}", res);
                        break 'main;
                    }
                    if gameboard.is_column_all_marked(position.y) {
                        for row in gameboard.data.iter() {
                            println!("{:?}", row);
                        }
                        let res = gameboard.sum_unmarked() * lucky_number;
                        println!("The final result is {}", res);
                        break 'main;
                    }

                },
                _ => continue,
            };
        }
    }
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
        let mut input = "10  1\n12 17\n\n 9  7\n16 19\n\n".as_bytes();
        let actual = build_gameboards(&mut input, 2, 2);
        let expected = vec![
            Gameboard{
                id: 0,
                rows: 2,
                columns: 2,
                data: vec![
                    vec![Field::Unmarked(10), Field::Unmarked(1)],
                    vec![Field::Unmarked(12), Field::Unmarked(17)],
                ],
            },
            Gameboard{
                id: 1,
                rows: 2,
                columns: 2,
                data: vec![
                    vec![Field::Unmarked(9), Field::Unmarked(7)],
                    vec![Field::Unmarked(16), Field::Unmarked(19)],
                ],
            },
        ];

        assert_eq!(actual.len(), 2);
        assert_eq!(actual, expected);
    }

    #[test_case(Gameboard{ id: 1, rows: 1, columns: 1, data: vec![vec![Field::Marked(10), Field::Marked(1), Field::Marked(2)]] } => true)]
    #[test_case(Gameboard{ id: 1, rows: 1, columns: 1, data: vec![vec![Field::Marked(10), Field::Unmarked(1), Field::Marked(2)]] } => false)]
    #[test_case(Gameboard{ id: 1, rows: 1, columns: 1, data: vec![vec![Field::Unmarked(1), Field::Unmarked(11), Field::Unmarked(2)]] } => false)]
    fn is_row_all_marked_returns_expected(gameboard: Gameboard) -> bool {
        gameboard.is_row_all_marked(0)
    }

    #[test_case(Gameboard{ id: 1, rows: 1, columns: 1, data: vec![vec![Field::Marked(10), Field::Marked(1)], vec![Field::Marked(12), Field::Unmarked(8)]] }, 1 => false)]
    #[test_case(Gameboard{ id: 1, rows: 1, columns: 1, data: vec![vec![Field::Marked(10), Field::Marked(1)], vec![Field::Marked(12), Field::Unmarked(8)]] }, 0 => true)]
    fn is_column_all_marked_returns_expected(gameboard: Gameboard, column_index: usize) -> bool {
        gameboard.is_column_all_marked(column_index)
    }

    #[test]
    fn check_number_returns_expected() {
        let mut gameboard = Gameboard {
            id: 0,
            rows: 2,
            columns: 2,
            data: vec![
                vec![Field::Unmarked(2), Field::Unmarked(1)],
                vec![Field::Unmarked(10), Field::Marked(4)],
            ],
        };

        assert_eq!(gameboard.check_number(1), Some(Position{x: 0, y: 1}));
        let actual = match gameboard.data.get(0) {
            Some(field) => match field.get(1) {
                Some(field) => field,
                _ => &Field::None,
            },
            _ => &Field::None,
        };
        assert_eq!(actual, &Field::Marked(1));
        assert_eq!(gameboard.check_number(2), Some(Position{x: 0, y: 0}));
        assert_eq!(gameboard.check_number(10), Some(Position{x: 1, y: 0}));
        assert_eq!(gameboard.check_number(4), None);
    }

    #[test]
    fn sum_unmarked_return_expected() {
        let gameboard = Gameboard{
            id: 0,
            rows: 3,
            columns: 3,
            data: vec![
                vec![Field::Unmarked(1), Field::Unmarked(2), Field::Marked(3)],
                vec![Field::Marked(4), Field::Marked(5), Field::Marked(6)],
                vec![Field::Marked(7), Field::Unmarked(8), Field::Unmarked(9)],
            ],
        };
        let expected = 20;
        let actual = gameboard.sum_unmarked();

        assert_eq!(actual, expected);
    }
}
