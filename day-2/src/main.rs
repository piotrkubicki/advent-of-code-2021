use std::fs::File;
use std::io::{BufRead, BufReader};

struct Position {
    x_pos: u32,
    y_pos: u32,
}

struct PositionAim {
    x_pos: u32,
    y_pos: u32,
    aim: u32,
}

trait Submarine {
    fn change_position(&mut self, direction: &str, strength: u32);
    fn calculate_position(&self) -> u32;
}

impl Submarine for Position {
    fn change_position(&mut self, direction: &str, strength: u32) {
        match direction {
            "forward" => self.x_pos += strength,
            "down" => self.y_pos += strength,
            "up" => self.y_pos -= strength,
            _ => println!("Unknown command, skipping!")
        }
    }

    fn calculate_position(&self) -> u32 {
        self.x_pos * self.y_pos
    }
}

impl Submarine for PositionAim {
    fn change_position(&mut self, direction: &str, strength: u32) {
        match direction {
            "forward" => {
                self.x_pos += strength;
                self.y_pos += self.aim * strength;
            },
            "down" => self.aim += strength,
            "up" => self.aim -= strength,
            _ => println!("Unknown command, skipping!")
        }
    }

    fn calculate_position(&self) -> u32 {
        self.x_pos *self.y_pos
    }
}

fn parse_command(command: &str) -> (&str, &str) {
    for (i, &word) in command.as_bytes().iter().enumerate() {
        if word == b' ' {
            return (&command[..i], &command[i+1..])
        }
    }

    (&"", &"")
}

fn execute_command<S: Submarine>(command: &str, submarine: &mut S) {
    let (direction, strength) = parse_command(command);
    if direction != "" && strength != "" {
        let strength = strength.parse::<u32>().expect("Cannot parse value!");
        submarine.change_position(direction, strength);
    } else {
        println!("Invalid command format {}, skipping!", command);
    }
}

fn drive<R: BufRead, S: Submarine>(commands: &mut R, submarine: &mut S) {
    for command in commands.lines() {
        let command = command.expect("Unable to read line!");
        execute_command(&command, submarine);
    }
}

fn task_one() {
    let file = File::open("src/input.txt").expect("File cannot be opened!");
    let mut commands = BufReader::new(file);
    let mut submarine: Position = Position{ x_pos: 0, y_pos: 0 };
    drive(&mut commands, &mut submarine);
    let result = submarine.calculate_position();

    println!("Total: {}", result);
}

fn task_two() {
    let file = File::open("src/input.txt").expect("File cannot be opened!");
    let mut commands = BufReader::new(file);
    let mut submarine: PositionAim = PositionAim{ x_pos: 0, y_pos: 0, aim: 0 };
    drive(&mut commands, &mut submarine);
    let result = submarine.calculate_position();

    println!("Total: {}", result);
}

fn main() {
    task_one();
    task_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_return_correct_tuple() {
        let command = "up 10".to_string();
        let (direction, strength) = parse_command(&command);

        assert_eq!(direction, "up", "direction is not equal up");
        assert_eq!(strength, "10", "strength is not equal 10");
    }

    #[test]
    fn test_parse_command_return_empty_tuple() {
        let command = "down10".to_string();
        let (direction, strength) = parse_command(&command);

        assert_eq!(direction, "", "direction is not empty");
        assert_eq!(strength, "", "strength is not empty");
    }

    #[test]
    fn test_execute_forward_command_for_position() {
        let command = "forward 5".to_string();
        let mut actual: Position = Position{ x_pos: 0, y_pos: 0};
        let expected: Position = Position{ x_pos: 5, y_pos: 0};

        execute_command(&command, &mut actual);
        assert_eq!(actual.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(actual.y_pos, expected.y_pos, "y_pos are not equal");
    }

    #[test]
    fn test_execute_up_command_for_position() {
        let command = "up 2".to_string();
        let mut actual: Position = Position{ x_pos: 2, y_pos: 12 };
        let expected: Position = Position{ x_pos: 2, y_pos: 10 };

        execute_command(&command, &mut actual);
        assert_eq!(actual.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(actual.y_pos, expected.y_pos, "y_pos are not equal");
    }

    #[test]
    fn test_execute_down_command_for_position() {
        let command = "down 3".to_string();
        let mut actual: Position = Position{ x_pos: 2, y_pos: 12 };
        let expected: Position = Position{ x_pos: 2, y_pos: 15};

        execute_command(&command, &mut actual);
        assert_eq!(actual.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(actual.y_pos, expected.y_pos, "y_pos are not equal");
    }

    #[test]
    fn test_execute_forward_command_for_positionaim() {
        let command = "forward 5".to_string();
        let mut actual: PositionAim = PositionAim{ x_pos: 0, y_pos: 10, aim: 2 };
        let expected: PositionAim = PositionAim{ x_pos: 5, y_pos: 20, aim: 2 };

        execute_command(&command, &mut actual);
        assert_eq!(actual.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(actual.y_pos, expected.y_pos, "y_pos are not equal");
        assert_eq!(actual.aim, expected.aim, "aim are not equal");
    }

    #[test]
    fn test_execute_up_command_for_positionaim() {
        let command = "up 2".to_string();
        let mut actual: PositionAim = PositionAim{ x_pos: 2, y_pos: 12, aim: 3 };
        let expected: PositionAim = PositionAim{ x_pos: 2, y_pos: 12, aim: 1 };

        execute_command(&command, &mut actual);
        assert_eq!(actual.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(actual.y_pos, expected.y_pos, "y_pos are not equal");
        assert_eq!(actual.aim, expected.aim, "aim are not equal");
    }

    #[test]
    fn test_execute_down_command_for_positionaim() {
        let command = "down 3".to_string();
        let mut actual: PositionAim = PositionAim{ x_pos: 2, y_pos: 12, aim: 8 };
        let expected: PositionAim = PositionAim{ x_pos: 2, y_pos: 12, aim: 11 };

        execute_command(&command, &mut actual);
        assert_eq!(actual.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(actual.y_pos, expected.y_pos, "y_pos are not equal");
        assert_eq!(actual.aim, expected.aim, "aim are not equal");
    }

    #[test]
    fn test_drive() {
        let commands = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        let expected: Position = Position{ x_pos: 15, y_pos: 10 };

        let mut position: Position = Position{ x_pos: 0, y_pos: 0 };
        drive(&mut commands.as_bytes(), &mut position);
        assert_eq!(position.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(position.y_pos, expected.y_pos, "y_pos are not equal");
    }
}
