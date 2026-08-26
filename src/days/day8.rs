use std::fmt::Display;
use std::fs::read_to_string;

struct Screen {
    screen_width: usize,
    lines: Vec<u64>,
}

impl Screen {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Screen {
            screen_width,
            lines: vec![0; screen_height],
        }
    }

    pub fn rect(&mut self, width: usize, height: usize) {
        for line in 0..height {
            self.lines[line] |= ((1 << width) - 1) << (self.screen_width - width);
        }
    }

    pub fn rotate_row(&mut self, row: usize, rotation: usize) {
        let saved = self.lines[row] & ((1 << rotation) - 1);

        self.lines[row] = (self.lines[row] >> rotation) | (saved << (self.screen_width - rotation));
    }

    pub fn rotate_col(&mut self, col: usize, rotation: usize) {
        let on_mask = 1 << (self.screen_width - col - 1);

        for _ in 0..rotation {
            let row_last = self.lines[self.lines.len() - 1] & on_mask;

            for row in (0..self.lines.len()).rev() {
                let above_row = if row == 0 {
                    row_last
                } else {
                    self.lines[row - 1]
                };

                if above_row & on_mask == on_mask {
                    self.lines[row] |= on_mask;
                } else {
                    self.lines[row] &= !on_mask;
                }
            }
        }
    }

    pub fn process_input(&mut self, input: &str) {
        for mut line in input.lines().map(|x| x.trim()) {
            if line.starts_with("rect") {
                line = line.trim_start_matches("rect").trim();

                let mut parts = line.split('x');

                self.rect(
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                );
            }

            if line.starts_with("rotate row") {
                line = line.trim_start_matches("rotate row y=").trim();

                let mut parts = line.split("by");

                self.rotate_row(
                    parts.next().unwrap().trim().parse().unwrap(),
                    parts.next().unwrap().trim().parse().unwrap(),
                );
            }

            if line.starts_with("rotate column") {
                line = line.trim_start_matches("rotate column x=").trim();

                let mut parts = line.split("by");

                self.rotate_col(
                    parts.next().unwrap().trim().parse().unwrap(),
                    parts.next().unwrap().trim().parse().unwrap(),
                );
            }
        }
    }

    pub fn get_on_count(&self) -> u32 {
        let mut on_count = 0;

        for line in &self.lines {
            for i in 0..self.screen_width {
                let mask = 1 << i;

                if mask & line == mask {
                    on_count += 1;
                }
            }
        }

        on_count
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{:0width$b}", line, width = self.screen_width)?
        }

        writeln!(f)?;

        for line in &self.lines {
            for i in (0..self.screen_width).rev() {
                let mask = 1 << i;

                if mask & line == mask {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }

            writeln!(f)?
        }

        std::fmt::Result::Ok(())
    }
}

pub fn part1() {
    let mut screen = Screen::new(50, 6);

    let input = read_to_string("data/day8.txt").unwrap();

    screen.process_input(&input);

    println!("{}", screen.get_on_count());
}

pub fn part2() {
    let mut screen = Screen::new(50, 6);

    let input = read_to_string("data/day8.txt").unwrap();

    screen.process_input(&input);

    println!("{}", screen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut screen = Screen::new(7, 3);

        screen.rect(3, 2);

        assert_eq!(0b1110000, screen.lines[0]);
        assert_eq!(0b1110000, screen.lines[1]);
        assert_eq!(0b0000000, screen.lines[2]);

        screen.rotate_col(1, 1);

        assert_eq!(0b1010000, screen.lines[0]);
        assert_eq!(0b1110000, screen.lines[1]);
        assert_eq!(0b0100000, screen.lines[2]);

        screen.rotate_row(0, 4);

        assert_eq!(0b0000101, screen.lines[0]);
        assert_eq!(0b1110000, screen.lines[1]);
        assert_eq!(0b0100000, screen.lines[2]);

        screen.rotate_col(1, 1);

        assert_eq!(0b0100101, screen.lines[0]);
        assert_eq!(0b1010000, screen.lines[1]);
        assert_eq!(0b0100000, screen.lines[2]);

        assert_eq!(6, screen.get_on_count());
    }

    #[test]
    fn test_2() {
        let mut screen = Screen::new(7, 3);

        screen.process_input(
            r"
            rect 3x2
            rotate column x=1 by 1
            rotate row y=0 by 4
            rotate column x=1 by 1",
        );

        assert_eq!(0b0100101, screen.lines[0]);
        assert_eq!(0b1010000, screen.lines[1]);
        assert_eq!(0b0100000, screen.lines[2]);

        assert_eq!(6, screen.get_on_count());
    }
}
