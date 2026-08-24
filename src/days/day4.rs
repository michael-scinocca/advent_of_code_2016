use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

pub fn part1() {
    let input = read_to_string("data/day4.txt").unwrap();

    let mut sector_id_sum = 0;

    for line in input.lines() {
        let (real, sector_id, _) = is_real_room(line);

        if real {
            sector_id_sum += sector_id;
        }
    }

    println!("{sector_id_sum}");
}

pub fn part2() {
    let input = read_to_string("data/day4.txt").unwrap();

    for line in input.lines() {
        let (real, sector_id, real_name) = is_real_room(line);

        if real && real_name.contains("northpole") {
            println!("{sector_id}");
        }
    }
}

fn is_real_room(room: &str) -> (bool, u32, String) {
    let mut is_real = true;
    let mut sector_id = 0;
    let mut real_name = String::new();

    let mut checksum = "";

    let mut parts = room.split('-').peekable();

    let mut char_frequencies = HashMap::new();

    let mut working_name = String::new();

    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            let mut sector_checksum_parts = part.split('[');

            let sector_id_part = sector_checksum_parts.next().unwrap();

            sector_id = sector_id_part.parse::<u32>().unwrap();
            checksum = sector_checksum_parts.next().unwrap().trim_end_matches(']');
        } else {
            for char in part.chars() {
                char_frequencies
                    .entry(char)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);

                working_name.push(char);
            }

            working_name.push(' ');
        }
    }

    for mut char in working_name.chars() {
        if char == ' ' {
            real_name.push(char);
            continue;
        }

        for _ in 0..sector_id {
            char = ((char as u8) + 1) as char;

            if char > 'z' {
                char = 'a';
            }
        }

        real_name.push(char);
    }

    let mut highest_values: Vec<_> = char_frequencies.iter().map(|f| (f.0, f.1)).collect();
    highest_values.sort_by(|a, b| match b.1.cmp(a.1) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a.0.cmp(b.0),
    });

    let mut checksum_chars = checksum.chars();

    for (index, val) in highest_values[0..5].iter().enumerate() {
        let checksum_char = checksum_chars.next().unwrap();

        if checksum.len() < index || checksum_char != *val.0 {
            is_real = false;

            break;
        }
    }

    (is_real, sector_id, real_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (real, sector_id, _) = is_real_room("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_eq!(true, real);
        assert_eq!(123, sector_id);
    }

    #[test]
    fn test_2() {
        let (real, sector_id, _) = is_real_room("a-b-c-d-e-f-g-h-987[abcde]");
        assert_eq!(true, real);
        assert_eq!(987, sector_id);
    }

    #[test]
    fn test_3() {
        let (real, sector_id, _) = is_real_room("not-a-real-room-404[oarel]");
        assert_eq!(true, real);
        assert_eq!(404, sector_id);
    }

    #[test]
    fn test_4() {
        let (real, sector_id, _) = is_real_room("totally-real-room-200[decoy]");
        assert_eq!(false, real);
        assert_eq!(200, sector_id);
    }
}
