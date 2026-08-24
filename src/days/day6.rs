use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("data/day6.txt").unwrap();

    let code = get_code(&input, 1);

    println!("{code}");
}

pub fn part2() {
    let input = read_to_string("data/day6.txt").unwrap();

    let code = get_code(&input, 2);

    println!("{code}");
}

fn get_code(input: &str, version: u32) -> String {
    let mut code = String::new();

    let code_length = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .next()
        .unwrap()
        .len();

    let mut frequency = Vec::new();

    for _ in 0..code_length {
        frequency.push(HashMap::new());
    }

    for line in input.lines().filter(|l| !l.is_empty()).map(|l| l.trim()) {
        let line_chars = line.chars();

        for (index, char) in line_chars.enumerate() {
            frequency[index]
                .entry(char)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    for position in frequency {
        let pos_list: Vec<_> = position.iter().map(|x| (x.0, x.1)).collect();
        let max_for_position = if version == 1 {
            pos_list.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0
        } else {
            pos_list.iter().max_by(|a, b| b.1.cmp(a.1)).unwrap().0
        };

        code.push(*max_for_position);
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = r"
            eedadn
            drvtee
            eandsr
            raavrd
            atevrs
            tsrnev
            sdttsa
            rasrtv
            nssdts
            ntnada
            svetve
            tesnvt
            vntsnd
            vrdear
            dvrsen
            enarar";

        assert_eq!("easter", get_code(input, 1));
    }

    #[test]
    fn test_2() {
        let input = r"
            eedadn
            drvtee
            eandsr
            raavrd
            atevrs
            tsrnev
            sdttsa
            rasrtv
            nssdts
            ntnada
            svetve
            tesnvt
            vntsnd
            vrdear
            dvrsen
            enarar";

        assert_eq!("advent", get_code(input, 2));
    }
}
