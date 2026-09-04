pub fn part1() {
    let input = std::fs::read_to_string("data/day9.txt").unwrap();

    let output = decompress(&input);

    println!("{output}");
    println!("{}", output.len())
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day9.txt").unwrap();

    let output = decompress_v2(&input);

    println!("{}", output)
}

fn decompress(input: &str) -> String {
    let input = input.replace(" ", "");
    let input = input.trim();

    let mut output = String::new();

    let mut chars = input.chars();

    let mut length = String::new();
    let mut repetitions = String::new();
    let mut chunk = String::new();

    while let Some(char) = chars.next() {
        let length_and_repetitions = {
            if char == '(' {
                length.clear();
                repetitions.clear();

                for char in chars.by_ref() {
                    if char == 'x' {
                        break;
                    }

                    length.push(char);
                }

                for char in chars.by_ref() {
                    if char == ')' {
                        break;
                    }

                    repetitions.push(char);
                }

                Some((
                    length.parse::<u32>().unwrap(),
                    repetitions.parse::<usize>().unwrap(),
                ))
            } else {
                None
            }
        };

        if let Some((length, repetitions)) = length_and_repetitions {
            let mut count = 0;

            chunk.clear();

            for char in chars.by_ref() {
                chunk.push(char);

                count += 1;

                if count == length {
                    break;
                }
            }

            output.push_str(&chunk.repeat(repetitions));
        } else {
            output.push(char);
        }
    }

    output
}

fn decompress_v2(input: &str) -> u64 {
    let input = input.replace(" ", "");
    let input = input.trim();

    let mut output_length: u64 = 0;

    let mut chars = input.chars();

    let mut length = String::new();
    let mut repetitions = String::new();
    let mut chunk = String::new();

    while let Some(char) = chars.next() {
        let length_and_repetitions = {
            if char == '(' {
                length.clear();
                repetitions.clear();

                for char in chars.by_ref() {
                    if char == 'x' {
                        break;
                    }

                    length.push(char);
                }

                for char in chars.by_ref() {
                    if char == ')' {
                        break;
                    }

                    repetitions.push(char);
                }

                Some((
                    length.parse::<u32>().unwrap(),
                    repetitions.parse::<usize>().unwrap(),
                ))
            } else {
                None
            }
        };

        if let Some((length, repetitions)) = length_and_repetitions {
            let mut count = 0;

            chunk.clear();

            for char in chars.by_ref() {
                chunk.push(char);

                count += 1;

                if count == length {
                    break;
                }
            }

            output_length =
                output_length.saturating_add(decompress_v2(&chunk) * repetitions as u64);
        } else {
            output_length += 1;
        }
    }

    output_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let output = decompress("ADVENT");

        assert_eq!(output, "ADVENT");
    }

    #[test]
    fn test_2() {
        let output = decompress("A(1x5)BC");

        assert_eq!(output, "ABBBBBC");
    }

    #[test]
    fn test_3() {
        let output = decompress("(3x3)XYZ");

        assert_eq!(output, "XYZXYZXYZ");
    }

    #[test]
    fn test_4() {
        let output = decompress("A(2x2)BCD(2x2)EFG");

        assert_eq!(output, "ABCBCDEFEFG");
    }

    #[test]
    fn test_5() {
        let output = decompress("(6x1)(1x3)A");

        assert_eq!(output, "(1x3)A");
    }

    #[test]
    fn test_6() {
        let output = decompress("X(8x2)(3x3)ABCY");

        assert_eq!(output, "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_v2_1() {
        let output = decompress_v2("(3x3)XYZ");

        assert_eq!(output, 9);
    }

    #[test]
    fn test_v2_2() {
        let output = decompress_v2("X(8x2)(3x3)ABCY");

        assert_eq!(output, 20);
    }

    #[test]
    fn test_v2_3() {
        let output = decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A");

        assert_eq!(output, 241920);
    }

    #[test]
    fn test_v2_4() {
        let output = decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");

        assert_eq!(output, 445);
    }
}
