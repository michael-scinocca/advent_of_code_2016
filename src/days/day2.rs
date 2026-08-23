use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("data/day2.txt").unwrap();

    println!("{}", get_code(&input).concat());
}

pub fn part2() {
    let input = read_to_string("data/day2.txt").unwrap();

    println!("{}", get_code_2(&input).concat());
}

fn get_code(input: &str) -> Vec<String> {
    let mut result = Vec::new();

    let keypad = [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let mut position = (1, 1);

    let lines = input.lines().filter(|l| !l.is_empty()).map(|l| l.trim());

    for line in lines {
        for char in line.chars() {
            match char {
                'L' if position.0 > 0 => position.0 -= 1,
                'R' if position.0 < 2 => position.0 += 1,
                'U' if position.1 > 0 => position.1 -= 1,
                'D' if position.1 < 2 => position.1 += 1,
                _ => (),
            };
        }

        result.push(keypad[position.1][position.0].to_string());
    }

    result
}

fn get_code_2(input: &str) -> Vec<String> {
    let mut result = Vec::new();

    let keypad = [
        vec!['X', 'X', '1', 'X', 'X'],
        vec!['X', '2', '3', '4', 'X'],
        vec!['5', '6', '7', '8', '9'],
        vec!['X', 'A', 'B', 'C', 'X'],
        vec!['X', 'X', 'D', 'X', 'X'],
    ];

    let mut position = (0, 2);

    let lines = input.lines().filter(|l| !l.is_empty()).map(|l| l.trim());

    for line in lines {
        for char in line.chars() {
            match char {
                'L' if position.0 > 0 && keypad[position.1][position.0 - 1] != 'X' => {
                    position.0 -= 1
                }
                'R' if position.0 < 4 && keypad[position.1][position.0 + 1] != 'X' => {
                    position.0 += 1
                }
                'U' if position.1 > 0 && keypad[position.1 - 1][position.0] != 'X' => {
                    position.1 -= 1
                }
                'D' if position.1 < 4 && keypad[position.1 + 1][position.0] != 'X' => {
                    position.1 += 1
                }
                _ => (),
            };
        }

        result.push(keypad[position.1][position.0].to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = r"
            ULL
            RRDDD
            LURDL
            UUUUD";

        assert_eq!("1985", get_code(input).concat());
    }

    #[test]
    fn test_2() {
        let input = r"
            ULL
            RRDDD
            LURDL
            UUUUD";

        assert_eq!("5DB3", get_code_2(input).concat());
    }
}
