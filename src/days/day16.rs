pub fn part1() {
    let output = generate_data("00111101111101000", 272);
    let checksum = get_checksum(&output);

    println!("{}", checksum);
}

pub fn part2() {
    let output = generate_data("00111101111101000", 35651584);

    let checksum = get_checksum(&output);

    println!("{}", checksum);
}

fn transform_input(input: &str) -> String {
    let input_len = input.len();

    let mut transform = String::with_capacity(input_len * 2 + 1);

    transform.push_str(input);
    transform.push('0');

    for char in input.chars().rev() {
        transform.push(match char {
            '0' => '1',
            '1' => '0',
            _ => char,
        });
    }

    transform
}

fn generate_data(input: &str, length: usize) -> String {
    let mut output = input.to_string();

    while output.len() < length {
        output = transform_input(&output);
    }

    output[0..length].to_string()
}

fn get_checksum(input: &str) -> String {
    let mut checksum = String::new();

    let mut input = input.to_owned();

    loop {
        let mut chars = input.chars();

        while let Some(char1) = &chars.next() {
            let char2 = &chars.next().unwrap();

            if char1 == char2 {
                checksum.push('1');
            } else {
                checksum.push('0');
            }
        }

        if !checksum.len().is_multiple_of(2) {
            break;
        }

        input = checksum.clone();
        checksum.clear();
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let output1 = transform_input("1");
        let output2 = transform_input("0");
        let output3 = transform_input("11111");
        let output4 = transform_input("111100001010");
        let output5 = transform_input("10000");

        assert_eq!(output1, "100");
        assert_eq!(output2, "001");
        assert_eq!(output3, "11111000000");
        assert_eq!(output4, "1111000010100101011110000");
        assert_eq!(output5, "10000011110");
    }

    #[test]
    fn test2() {
        let output = generate_data("10000", 20);

        assert_eq!(output, "10000011110010000111");
    }

    #[test]
    fn test3() {
        let checksum = get_checksum("110010110100");

        assert_eq!(checksum, "100");
    }
}
