use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

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

    counter
}

fn task_one() {
    let file = File::open("readings.txt").expect("Unable to open file");
    let mut readings = BufReader::new(file);
    let total = count_depth_increase(&mut readings);

    println!("Total depth increase: {}", total);
}

fn count_sum_depth_increase<R: BufRead>(readings: &mut R) -> i32 {
    const WINDOW_SIZE: usize = 3;
    let mut readings_arr: [i32; WINDOW_SIZE] = [0; WINDOW_SIZE];
    let mut counter = 0;
    let mut previous_sum = 0;

    for line in readings.lines().enumerate() {
        let (i, line) = line;
        let reading = line.expect("Unable to read line").parse::<i32>().unwrap();

        match (i+1).cmp(&WINDOW_SIZE) {
            Ordering::Less => readings_arr[i] = reading,
            _ => {
                let current_sum = readings_arr.iter().sum();
                if previous_sum == 0 {
                    previous_sum = current_sum;
                }
                if previous_sum < current_sum {
                    counter += 1;
                }
                previous_sum = current_sum;
                readings_arr[i%WINDOW_SIZE] = reading;
            }
        }
    }

    counter
}

fn task_two() {
    let file = File::open("readings.txt").expect("Unable to open file");
    let mut readings = BufReader::new(file);
    let total = count_sum_depth_increase(&mut readings);

    println!("Total sum depth increase: {}", total);
}

fn main() {
    task_one();
    task_two();
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

    #[test]
    fn test_count_sum_depth_increase() {
        let test_input = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        let actual = count_sum_depth_increase(&mut test_input.as_bytes());
        assert_eq!(5, actual);

        let test_input = String::from("199\n200\n208\n200\n100\n228\n240\n269\n260\n263\n");
        let actual = count_sum_depth_increase(&mut test_input.as_bytes());
        assert_eq!(6, actual);
    }
}
