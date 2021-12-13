use std::fs::File;
use std::io::{BufRead, BufReader};

struct PowerReport {
    total: u32,
    column_sums: Vec<u32>,
}

trait ReportParser {
    fn update_report(&mut self, reading: &str);
}

impl PowerReport {
    fn get_gamma_rate(&self) -> String {
        let mut gamma_rate = String::new();

        for &value in self.column_sums.iter() {
            if value >= (self.total as f32 / 2.0).ceil() as u32 {
                gamma_rate.push_str("1");
            } else {
                gamma_rate.push_str("0");
            }
        }
        gamma_rate
    }

    fn get_epsilon_rate(&self) -> String {
        let mut epsilon_rate = String::new();

        for &value in self.column_sums.iter() {
            if value >= (self.total as f32 / 2.0).ceil() as u32 {
                epsilon_rate.push_str("0");
            } else {
                epsilon_rate.push_str("1");
            }
        }
        epsilon_rate
    }

    fn get_power_consumption(&self) -> u32 {
        let gamma_rate = isize::from_str_radix(&self.get_gamma_rate(), 2).unwrap() as u32;
        let epsilon_rate = isize::from_str_radix(&self.get_epsilon_rate(), 2).unwrap() as u32;

        gamma_rate * epsilon_rate
    }
}

impl ReportParser for PowerReport {
    fn update_report(&mut self, reading: &str) {
        self.total += 1;
        if self.column_sums.len() == 0 {
            self.column_sums = vec![0; reading.len() - 1];
        }

        for (i, bite) in reading.as_bytes().iter().enumerate() {
            match bite {
                b'1' => self.column_sums[i] += 1,
                _ => continue,
            }
        }
    }
}

struct LifeSupportReport {
    readings: Vec<String>,
}

impl LifeSupportReport {
    fn get_oxygen_gen_rate(&mut self) -> String {
        for i in 0..4 {
            println!("{:?}", self.readings);
            let filter: char = self.readings[0].chars().nth(i).unwrap();
            self.readings = self
                .readings
                .iter()
                .cloned()
                .filter(|reading| reading.chars().nth(i).unwrap() != filter)
                .collect();
        }
        self.readings[0].clone()
    }
}

fn run<R: BufRead, T: ReportParser>(readings: &mut R, report: &mut T) {
    let mut first_line = String::new();
    readings
        .read_line(&mut first_line)
        .expect("Cannot read line!");
    report.update_report(&first_line);

    for reading in readings.lines() {
        let reading = reading.expect("Cannot read line!");
        report.update_report(&reading);
    }
}

fn task_one() {
    let file = File::open("src/input.txt").expect("Cannot open file!");
    let mut readings = BufReader::new(file);
    let mut report = PowerReport {
        total: 0,
        column_sums: vec![],
    };

    run(&mut readings, &mut report);
    let power_consumption = report.get_power_consumption();
    println!("The power consumption is: {}", power_consumption);
}

fn main() {
    task_one();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(12, [7, 5, 7, 7, 4].to_vec(), r#"10110"#)]
    #[test_case(4, [3, 0, 2, 4].to_vec(), r#"1011"#)]
    #[test_case(5, [3, 1, 2, 4, 5].to_vec(), r#"10011"#)]
    fn test_get_gamma_rate_for_report(total: u32, column_sums: Vec<u32>, expected: &str) {
        let report = PowerReport {
            total: total,
            column_sums: column_sums,
        };
        let actual = report.get_gamma_rate();

        assert_eq!(actual, expected);
    }

    #[test_case(12, [7, 5, 7, 7, 4].to_vec(), r#"01001"#)]
    #[test_case(4, [3, 0, 2, 4].to_vec(), r#"0100"#)]
    #[test_case(5, [3, 1, 2, 4, 5].to_vec(), r#"01100"#)]
    fn test_get_epsilon_rate_for_report(total: u32, column_sums: Vec<u32>, expected: &str) {
        let report = PowerReport {
            total: total,
            column_sums: column_sums,
        };
        let actual = report.get_epsilon_rate();

        assert_eq!(actual, expected);
    }

    #[test_case(12, [7, 5, 7, 7, 4].to_vec(), 22*9)]
    #[test_case(4, [3, 0, 2, 4].to_vec(), 11*4)]
    #[test_case(5, [3, 1, 2, 4, 5].to_vec(), 19*12)]
    fn test_get_power_consumption_for_report(total: u32, column_sums: Vec<u32>, expected: u32) {
        let report = PowerReport {
            total: total,
            column_sums: column_sums,
        };
        let actual = report.get_power_consumption();

        assert_eq!(actual, expected);
    }

    #[test_case([12, 4, 0, 2, 11, 0, 9, 8, 8, 0].to_vec(), "0110100111", [12, 5, 1, 2, 12, 0, 9, 9, 9, 1].to_vec())]
    fn test_update_report(initial_state: Vec<u32>, reading: &str, expected: Vec<u32>) {
        let mut actual = PowerReport {
            total: 2,
            column_sums: initial_state,
        };
        actual.update_report(reading);

        assert_eq!(actual.total, 3, "The total value is not equal");
        for (actual_bit, expected_bit) in actual.column_sums.iter().zip(expected.iter()) {
            assert_eq!(actual_bit, expected_bit, "The column sum doesn't match");
        }
    }

    #[test]
    fn test_run() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        let mut report = PowerReport {
            total: 0,
            column_sums: vec![],
        };

        run(&mut input.as_bytes(), &mut report);
        let actual = report.get_power_consumption();
        println!("{} {}", report.get_gamma_rate(), report.get_epsilon_rate());
        assert_eq!(actual, 198);
    }

    #[test]
    fn test_get_oxygen_gen_rate() {
        let mut report = LifeSupportReport {
            readings: vec![
                "00100".to_string(),
                "11110".to_string(),
                "10110".to_string(),
                "10111".to_string(),
                "10101".to_string(),
                "01111".to_string(),
                "00111".to_string(),
                "11100".to_string(),
                "10000".to_string(),
                "11001".to_string(),
                "00010".to_string(),
                "01010".to_string(),
            ],
        };
        let expected = "10111".to_string();
        let actual = report.get_oxygen_gen_rate();

        assert_eq!(actual, expected);
    }
}
