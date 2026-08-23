use std::{collections::HashSet, fs::read_to_string};

enum Look {
    Up,
    Right,
    Down,
    Left,
}

struct Position {
    look: Look,
    x: i32,
    y: i32,
}

impl Position {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
pub fn part1() {
    let input = read_to_string("data/day1.txt").unwrap();

    let position = process_input_1(&input);

    println!("{}", position.distance());
}

pub fn part2() {
    let input = read_to_string("data/day1.txt").unwrap();

    let position = process_input_2(&input);

    println!("{}", position.distance());
}

fn process_input_1(input: &str) -> Position {
    let mut position = Position {
        look: Look::Up,
        x: 0,
        y: 0,
    };

    let parts = input.split(',').map(|p| p.trim());

    for part in parts {
        let blocks = part[1..].parse::<i32>().unwrap();

        if part.starts_with('R') {
            position.look = match &position.look {
                Look::Up => Look::Right,
                Look::Right => Look::Down,
                Look::Down => Look::Left,
                Look::Left => Look::Up,
            }
        } else if part.starts_with('L') {
            position.look = match &position.look {
                Look::Up => Look::Left,
                Look::Left => Look::Down,
                Look::Down => Look::Right,
                Look::Right => Look::Up,
            }
        }

        match &position.look {
            Look::Up => position.y += blocks,
            Look::Right => position.x += blocks,
            Look::Down => position.y -= blocks,
            Look::Left => position.x -= blocks,
        }
    }

    position
}

fn process_input_2(input: &str) -> Position {
    let mut visited = HashSet::new();

    let mut position = Position {
        look: Look::Up,
        x: 0,
        y: 0,
    };

    visited.insert((position.x, position.y));

    let parts = input.split(',').map(|p| p.trim());

    'main: for part in parts {
        let blocks = part[1..].parse::<i32>().unwrap();

        if part.starts_with('R') {
            position.look = match &position.look {
                Look::Up => Look::Right,
                Look::Right => Look::Down,
                Look::Down => Look::Left,
                Look::Left => Look::Up,
            }
        } else if part.starts_with('L') {
            position.look = match &position.look {
                Look::Up => Look::Left,
                Look::Left => Look::Down,
                Look::Down => Look::Right,
                Look::Right => Look::Up,
            }
        }

        for _ in 0..blocks {
            match &position.look {
                Look::Up => position.y += 1,
                Look::Right => position.x += 1,
                Look::Down => position.y -= 1,
                Look::Left => position.x -= 1,
            }

            if visited.contains(&(position.x, position.y)) {
                break 'main;
            }

            visited.insert((position.x, position.y));
        }
    }

    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let position = process_input_1("R2, L3");
        assert_eq!(5, position.distance());
    }

    #[test]
    fn test_2() {
        let position = process_input_1("R2, R2, R2");
        assert_eq!(2, position.distance());
    }

    #[test]
    fn test_3() {
        let position = process_input_1("R5, L5, R5, R3");
        assert_eq!(12, position.distance());
    }

    #[test]
    fn test_4() {
        let position = process_input_2("R8, R4, R4, R8");
        assert_eq!(4, position.distance());
    }
}
