pub fn part1() {
    let instructions = process_instructions(&std::fs::read_to_string("data/day21.txt").unwrap());

    let mut input: Vec<_> = "abcdefgh".chars().collect();

    for operation in instructions {
        transform(&mut input, operation);
    }

    println!("{}", input.into_iter().collect::<String>());
}

pub fn part2() {
    let instructions =
        process_instructions_reverse(&std::fs::read_to_string("data/day21.txt").unwrap());

    let mut input: Vec<_> = "fbgdceah".chars().collect();

    for operation in instructions {
        transform(&mut input, operation);
    }

    println!("{}", input.into_iter().collect::<String>());
}

enum Operation {
    SwapPosition {
        position_x: usize,
        position_y: usize,
    },
    SwapLetter {
        letter_x: char,
        letter_y: char,
    },
    Rotate {
        rotation: i32,
    },
    RotateLetter {
        letter: char,
    },
    RotateLetterReverse {
        letter: char,
    },
    Reverse {
        position_x: usize,
        position_y: usize,
    },
    Move {
        position_x: usize,
        position_y: usize,
    },
}

fn process_instructions(instructions: &str) -> Vec<Operation> {
    let mut operations = Vec::new();

    for line in instructions
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
    {
        if let Some(line) = line.strip_prefix("swap position ") {
            let mut parts = line.split(" ");

            let position_x = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            parts.next().unwrap();
            let position_y = parts.next().unwrap().parse::<usize>().unwrap();

            operations.push(Operation::SwapPosition {
                position_x,
                position_y,
            })
        }

        if let Some(line) = line.strip_prefix("swap letter ") {
            let mut parts = line.split(" ");

            let letter_x = parts.next().unwrap().chars().next().unwrap();
            parts.next().unwrap();
            parts.next().unwrap();
            let letter_y = parts.next().unwrap().chars().next().unwrap();

            operations.push(Operation::SwapLetter { letter_x, letter_y })
        }

        if let Some(line) = line.strip_prefix("reverse positions ") {
            let mut parts = line.split(" ");

            let position_x = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            let position_y = parts.next().unwrap().parse::<usize>().unwrap();

            operations.push(Operation::Reverse {
                position_x,
                position_y,
            });
        }

        if let Some(line) = line.strip_prefix("rotate left ") {
            let mut parts = line.split(" ");

            let rotation = -parts.next().unwrap().parse::<i32>().unwrap();

            operations.push(Operation::Rotate { rotation });
        }

        if let Some(line) = line.strip_prefix("rotate right ") {
            let mut parts = line.split(" ");

            let rotation = parts.next().unwrap().parse::<i32>().unwrap();

            operations.push(Operation::Rotate { rotation });
        }

        if let Some(line) = line.strip_prefix("move position ") {
            let mut parts = line.split(" ");

            let position_x = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            parts.next().unwrap();
            let position_y = parts.next().unwrap().parse::<usize>().unwrap();

            operations.push(Operation::Move {
                position_x,
                position_y,
            })
        }

        if let Some(line) = line.strip_prefix("rotate based on position of letter ") {
            let mut parts = line.split(" ");

            let letter = parts.next().unwrap().chars().next().unwrap();

            operations.push(Operation::RotateLetter { letter })
        }
    }

    operations
}

fn process_instructions_reverse(instructions: &str) -> Vec<Operation> {
    let mut operations = Vec::new();

    for line in instructions
        .trim()
        .lines()
        .rev()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
    {
        if let Some(line) = line.strip_prefix("swap position ") {
            let mut parts = line.split(" ");

            let position_x = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            parts.next().unwrap();
            let position_y = parts.next().unwrap().parse::<usize>().unwrap();

            operations.push(Operation::SwapPosition {
                position_x,
                position_y,
            })
        }

        if let Some(line) = line.strip_prefix("swap letter ") {
            let mut parts = line.split(" ");

            let letter_x = parts.next().unwrap().chars().next().unwrap();
            parts.next().unwrap();
            parts.next().unwrap();
            let letter_y = parts.next().unwrap().chars().next().unwrap();

            operations.push(Operation::SwapLetter { letter_x, letter_y })
        }

        if let Some(line) = line.strip_prefix("reverse positions ") {
            let mut parts = line.split(" ");

            let position_x = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            let position_y = parts.next().unwrap().parse::<usize>().unwrap();

            operations.push(Operation::Reverse {
                position_x,
                position_y,
            });
        }

        if let Some(line) = line.strip_prefix("rotate left ") {
            let mut parts = line.split(" ");

            let rotation = parts.next().unwrap().parse::<i32>().unwrap();

            operations.push(Operation::Rotate { rotation });
        }

        if let Some(line) = line.strip_prefix("rotate right ") {
            let mut parts = line.split(" ");

            let rotation = -parts.next().unwrap().parse::<i32>().unwrap();

            operations.push(Operation::Rotate { rotation });
        }

        if let Some(line) = line.strip_prefix("move position ") {
            let mut parts = line.split(" ");

            let position_y = parts.next().unwrap().parse::<usize>().unwrap();
            parts.next().unwrap();
            parts.next().unwrap();
            let position_x = parts.next().unwrap().parse::<usize>().unwrap();

            operations.push(Operation::Move {
                position_x,
                position_y,
            })
        }

        if let Some(line) = line.strip_prefix("rotate based on position of letter ") {
            let mut parts = line.split(" ");

            let letter = parts.next().unwrap().chars().next().unwrap();

            operations.push(Operation::RotateLetterReverse { letter })
        }
    }

    operations
}

fn transform(input: &mut Vec<char>, operation: Operation) {
    match operation {
        Operation::SwapPosition {
            position_x,
            position_y,
        } => {
            input.swap(position_x, position_y);
        }
        Operation::SwapLetter { letter_x, letter_y } => {
            let position_x = input.iter().position(|x| *x == letter_x).unwrap();
            let position_y = input.iter().position(|x| *x == letter_y).unwrap();

            input.swap(position_x, position_y);
        }
        Operation::Rotate { rotation } => {
            rotate(input, rotation);
        }
        Operation::RotateLetter { letter } => {
            let mut rotation = input.iter().position(|x| *x == letter).unwrap() as i32;

            if rotation >= 4 {
                rotation += 1;
            }

            rotation += 1;

            rotate(input, rotation);
        }
        Operation::RotateLetterReverse { letter } => {
            let position = input.iter().position(|x| *x == letter).unwrap();

            let rotation = match position {
                0 => -9,
                1 => -1,
                2 => -6,
                3 => -2,
                4 => -7,
                5 => -3,
                6 => -8,
                7 => -4,
                _ => 0,
            };

            rotate(input, rotation);
        }
        Operation::Reverse {
            position_x,
            position_y,
        } => {
            let mut reversed = input[position_x..=position_y].to_vec();
            reversed.reverse();

            let result = [
                &input[0..position_x],
                &reversed[..],
                &input[position_y + 1..],
            ]
            .concat();

            *input = result;
        }
        Operation::Move {
            position_x,
            position_y,
        } => {
            let char = input.remove(position_x);
            input.insert(position_y, char);
        }
    }
}

fn rotate(input: &mut Vec<char>, rotation: i32) {
    for _ in 0..rotation.abs() {
        if rotation > 0 {
            if let Some(val) = input.pop() {
                input.insert(0, val);
            }
        } else {
            input.push(input[0]);
            input.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_position() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(
            &mut input,
            Operation::SwapPosition {
                position_x: 1,
                position_y: 4,
            },
        );

        assert_eq!(&input, &['a', 'e', 'c', 'd', 'b']);
    }

    #[test]
    fn test_swap_letter() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(
            &mut input,
            Operation::SwapLetter {
                letter_x: 'b',
                letter_y: 'e',
            },
        );

        assert_eq!(&input, &['a', 'e', 'c', 'd', 'b']);
    }

    #[test]
    fn test_rotate_right() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(&mut input, Operation::Rotate { rotation: 1 });

        assert_eq!(input, "eabcd".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_rotate_left() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(&mut input, Operation::Rotate { rotation: -1 });

        assert_eq!(input, "bcdea".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_rotate_letter() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(&mut input, Operation::RotateLetter { letter: 'b' });

        assert_eq!(input, "deabc".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_reverse() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(
            &mut input,
            Operation::Reverse {
                position_x: 1,
                position_y: 3,
            },
        );

        assert_eq!(input, "adcbe".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_reverse_to_end() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(
            &mut input,
            Operation::Reverse {
                position_x: 1,
                position_y: 4,
            },
        );

        assert_eq!(input, "aedcb".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_move() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(
            &mut input,
            Operation::Move {
                position_x: 1,
                position_y: 3,
            },
        );

        assert_eq!(input, "acdbe".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_move_after() {
        let mut input: Vec<_> = "abcde".chars().collect();

        transform(
            &mut input,
            Operation::Move {
                position_x: 2,
                position_y: 1,
            },
        );

        assert_eq!(input, "acbde".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_instructions() {
        let instructions = process_instructions(
            r"
            swap position 4 with position 0
            swap letter d with letter b
            reverse positions 0 through 4
            rotate left 1 step,
            move position 1 to position 4
            move position 3 to position 0
            rotate based on position of letter b
            rotate based on position of letter d",
        );

        let mut input: Vec<_> = "abcde".chars().collect();

        for operation in instructions {
            transform(&mut input, operation);
        }

        assert_eq!(input, "decab".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_instructions_reverse() {
        let instructions_text = r"
            swap position 4 with position 0
            swap letter d with letter b
            reverse positions 0 through 4
            rotate left 1 step
            move position 1 to position 4
            move position 3 to position 0
            rotate based on position of letter b
            rotate based on position of letter d";

        let instructions = process_instructions(instructions_text);

        let instructions_reverse = process_instructions_reverse(instructions_text);

        let mut input: Vec<_> = "abcde".chars().collect();

        for operation in instructions {
            transform(&mut input, operation);
        }

        for operation in instructions_reverse {
            transform(&mut input, operation);
        }

        assert_eq!(input, "abcde".chars().collect::<Vec<_>>());
    }
}
