use std::fs::File;
use std::io::{BufRead, BufReader};

struct Position {
    x_pos: u32,
    y_pos: u32,
}

trait Driver {
    fn move_submarine(&mut self, direction: &str, strength: u32);
}

impl Driver for Position {
    fn move_submarine(&mut self, direction: &str, strength: u32) {
        match direction {
            "forward" => self.x_pos += strength,
            "down" => self.y_pos += strength,
            "up" => self.y_pos -= strength,
            _ => println!("Unknown command, skipping!")
        }
    }
}

fn execute_command<T: Driver>(command: &String, position: &mut T) {
    let command: Vec<&str> = command.split(' ').collect();
    if command.len() == 2 {
        let direction = command[0];
        let strength = command[1].parse::<u32>().expect("Cannot parse value!");
        position.move_submarine(direction, strength);
    } else {
        println!("Incorrect command format!");
    }
}

fn drive<R: BufRead>(commands: &mut R) -> Position {
    let mut position: Position = Position { x_pos: 0, y_pos: 0};

    for command in commands.lines() {
        let command = command.expect("Unable to read line!");
        execute_command(&command, &mut position);
    }

    position
}

fn task_one() {
    let file = File::open("src/input.txt").expect("File cannot be opened!");
    let mut commands = BufReader::new(file);
    let position: Position = drive(&mut commands);
    let result = position.x_pos * position.y_pos;

    println!("Total: {}", result);
}

fn main() {
    task_one();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_forward_command() {
        let command = "forward 5".to_string();
        let mut position: Position = Position{ x_pos: 0, y_pos: 0};
        let expected: Position = Position{ x_pos: 5, y_pos: 0};

        execute_command(&command, &mut position);
        assert_eq!(position.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(position.y_pos, expected.y_pos, "y_pos are not equal");
    }

    #[test]
    fn test_execute_up_command() {
        let command = "up 2".to_string();
        let mut position: Position = Position{ x_pos: 2, y_pos: 12 };
        let expected: Position = Position{ x_pos: 2, y_pos: 10 };

        execute_command(&command, &mut position);
        assert_eq!(position.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(position.y_pos, expected.y_pos, "y_pos are not equal");
    }

    #[test]
    fn test_execute_down_command() {
        let command = "down 3".to_string();
        let mut position: Position = Position{ x_pos: 2, y_pos: 12 };
        let expected: Position = Position{ x_pos: 2, y_pos: 15};

        execute_command(&command, &mut position);
        assert_eq!(position.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(position.y_pos, expected.y_pos, "y_pos are not equal");
    }

    #[test]
    fn test_drive() {
        let commands = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        let expected: Position = Position{ x_pos: 15, y_pos: 10 };

        let position: Position = drive(&mut commands.as_bytes());
        assert_eq!(position.x_pos, expected.x_pos, "x_pos are not equal");
        assert_eq!(position.y_pos, expected.y_pos, "y_pos are not equal");
    }
}
