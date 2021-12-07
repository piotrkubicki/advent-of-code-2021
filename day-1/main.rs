use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_depth_increase<R: BufRead>(readings: &mut R) -> i32 {
    let mut counter = 0;
    let mut previous = 0;

    for line in readings.lines() {
        let reading = line.expect("Unable to read line").parse::<i32>().unwrap();
        if previous == 0 {
            previous = reading;
        }
        if previous < reading {
            counter += 1;
        }
        previous = reading;
    }

    return counter;
}

fn task_one() {
    let file = File::open("readings.txt").expect("Unable to open file");
    let mut readings = BufReader::new(file);
    let total = count_depth_increase(&mut readings);

    println!("Total depth increase: {}", total);
}

fn main() {
    task_one();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_depth_increase() {
        let test_input = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        let actual = count_depth_increase(&mut test_input.as_bytes());
        assert_eq!(7, actual);
    }
}
