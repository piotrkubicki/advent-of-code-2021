struct Report {
    total: u32,
    column_sums: Vec<u32>,
}

trait ReportParser {
    fn get_gamma_rate(&self) -> String;
    fn get_epsilon_rate(&self) -> String;
    fn get_power_consumption(&self) -> u32;
}

impl ReportParser for Report {
    fn get_gamma_rate(&self) -> String {
        let mut gamma_rate = String::new();

        for &value in self.column_sums.iter() {
            if value >= (self.total as f32 / 2.0).ceil() as u32 {
                gamma_rate.push_str(&"1");
            } else {
                gamma_rate.push_str(&"0");
            }
        }
        gamma_rate
    }

    fn get_epsilon_rate(&self) -> String {
        let mut epsilon_rate = String::new();

        for &value in self.column_sums.iter() {
            if value >= (self.total as f32 / 2.0).ceil() as u32 {
                epsilon_rate.push_str(&"0");
            } else {
                epsilon_rate.push_str(&"1");
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

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(12, [7, 5, 7, 7, 4].to_vec(), r#"10110"#)]
    #[test_case(4, [3, 0, 2, 4].to_vec(), r#"1011"#)]
    #[test_case(5, [3, 1, 2, 4, 5].to_vec(), r#"10011"#)]
    fn test_get_gamma_rate_for_report(total: u32, column_sums: Vec<u32>, expected: &str) {
        let report = Report {
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
        let report = Report {
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
        let report = Report {
            total: total,
            column_sums: column_sums,
        };
        let actual = report.get_power_consumption();

        assert_eq!(actual, expected);
    }
}
