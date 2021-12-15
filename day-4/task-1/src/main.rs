use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_data<B: BufRead>(reader: &mut B) -> (Vec<u32>, Vec<[u32; 25]>) {
    let mut winning_numbers: Vec<u32> = vec![];
    let mut gameboards: Vec<[u32; 25]> = vec![];

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
        let test_input = "12,8,10,11,5,7,9\n\n10 11,  9, 19,  4\n 9 11 4  5 23\n\n21 11 32 18 12\n";
        let (actual_numbers, actual_gameboards) = read_data(&mut test_input.as_bytes());

        assert_eq!(actual_numbers, vec![12, 8, 10, 11, 5, 7, 9]);
        assert_eq!(
            actual_gameboards,
            vec![[10, 11, 9, 19, 4, 9, 11, 4, 5, 23], [21, 11, 32, 18, 12]]
        );
    }
}
