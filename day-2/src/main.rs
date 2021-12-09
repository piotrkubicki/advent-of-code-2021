use std::fs::File;
use std::io::{BufRead, BufReader};

const X: usize = 0;
const Y: usize = 1;

fn execute_command(command: &String, position: &mut [u32; 2]) {
    let command: Vec<&str> = command.split(' ').collect();
    if command.len() == 2 {
        let direction = command[0];
        let strength = command[1].parse::<u32>().expect("Cannot parse value!");

        match direction {
            "forward" => position[X] += strength,
            "down" => position[Y] += strength,
            "up" => position[Y] -= strength,
            _ => println!("Unknown command, skipping!")
        }
    } else {
        println!("Incorrect command format!");
    }
}

fn drive<R: BufRead>(commands: &mut R) -> [u32; 2] {
    let mut position: [u32; 2] = [0; 2];

    for command in commands.lines() {
        let command = command.expect("Unable to read line!");
        execute_command(&command, &mut position);
    }

    position
}

fn task_one() {
    let file = File::open("src/input.txt").expect("File cannot be opened!");
    let mut commands = BufReader::new(file);
    let position: [u32; 2] = drive(&mut commands);
    let mut result = 1;

    for value in position {
        result *= value;
    }

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
        let mut position: [u32; 2] = [0; 2];
        let expected: [u32; 2] = [5, 0];

        execute_command(&command, &mut position);
        assert!(position.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_execute_up_command() {
        let command = "up 2".to_string();
        let mut position: [u32; 2] = [2, 12];
        let expected: [u32; 2] = [2, 10];

        execute_command(&command, &mut position);
        assert!(position.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_execute_down_command() {
        let command = "down 3".to_string();
        let mut position: [u32; 2] = [2, 12];
        let expected: [u32; 2] = [2, 15];

        execute_command(&command, &mut position);
        assert!(position.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_drive() {
        let commands = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        let expected: [u32; 2] = [15, 10];

        let position : [u32; 2] = drive(&mut commands.as_bytes());
        assert!(position.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}
