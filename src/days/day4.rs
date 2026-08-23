use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

pub fn part1() {
    let input = read_to_string("data/day4.txt").unwrap();

    let mut sector_id_sum = 0;

    for line in input.lines() {
        let (real, sector_id) = is_real_room(line);

        if real {
            sector_id_sum += sector_id;
        }
    }

    println!("{sector_id_sum}");
}

pub fn part2() {}

fn is_real_room(room: &str) -> (bool, u32) {
    let mut is_real = true;
    let mut sector_id = 0;
    let mut checksum = "";

    let mut parts = room.split('-').peekable();

    let mut char_frequencies = HashMap::new();

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
            }
        }
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

        //println!("{} -> {} | {}", val.0, val.1, checksum_char);

        if checksum.len() < index || checksum_char != *val.0 {
            is_real = false;

            break;
        }
    }

    (is_real, sector_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!((true, 123), is_real_room("aaaaa-bbb-z-y-x-123[abxyz]"));
    }

    #[test]
    fn test_2() {
        assert_eq!((true, 987), is_real_room("a-b-c-d-e-f-g-h-987[abcde]"));
    }

    #[test]
    fn test_3() {
        assert_eq!((true, 404), is_real_room("not-a-real-room-404[oarel]"));
    }

    #[test]
    fn test_4() {
        assert_eq!((false, 200), is_real_room("totally-real-room-200[decoy]"));
    }
}
