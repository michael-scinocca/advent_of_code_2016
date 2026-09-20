use std::collections::VecDeque;

pub fn part1() {
    let path = run_navigation("qljzarfv");

    println!("{}", path);
}

pub fn part2() {}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn translate(&self, movement: char, limit: u32) -> Option<(Self, String)> {
        let (x_delta, y_delta) = match movement {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => (0, 0),
        };

        if (x_delta < 0 && self.x == 0)
            || (y_delta < 0 && self.y == 0)
            || (x_delta > 0 && self.x == limit)
            || (y_delta > 0 && self.y == limit)
        {
            return None;
        }

        let x = self.x.saturating_add_signed(x_delta);
        let y = self.y.saturating_add_signed(y_delta);

        Some((Self { x, y }, movement.to_string()))
    }
}

fn get_next_moves(input: &str, path: &str) -> Vec<char> {
    let hash_data = md5::compute(format!("{input}{path}"));
    let hash = format!("{:x}", hash_data);

    hash[0..4]
        .char_indices()
        .filter_map(|(i, c)| match c {
            'b' | 'c' | 'd' | 'e' | 'f' => match i {
                0 => Some('U'),
                1 => Some('D'),
                2 => Some('L'),
                3 => Some('R'),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn run_navigation(input: &str) -> String {
    let position = Position { x: 0, y: 0 };

    let mut queue = VecDeque::new();

    queue.push_back((position, String::new()));

    loop {
        let Some((position, path)) = queue.pop_front() else {
            break String::new();
        };

        if position.x == 3 && position.y == 3 {
            break path;
        }

        let next_moves: Vec<(Position, String)> = get_next_moves(input, &path)
            .into_iter()
            .filter_map(|m| position.translate(m, 3))
            .collect();

        for next_move in next_moves.iter() {
            let mut move_path = path.clone();
            move_path.push_str(&next_move.1);

            queue.push_back((next_move.0, move_path));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let path = run_navigation("ihgpwlah");

        assert_eq!(path, "DDRRRD");
    }

    #[test]
    fn test2() {
        let path = run_navigation("kglvqrro");

        assert_eq!(path, "DDUDRLRRUDRD");
    }

    #[test]
    fn test3() {
        let path = run_navigation("ulqzkmiv");

        assert_eq!(path, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }
}
