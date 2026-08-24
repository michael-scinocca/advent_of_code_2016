use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("data/day7.txt").unwrap();

    let mut count = 0;

    for line in input.lines() {
        if detect_tls(line) {
            count += 1;
        }
    }

    println!("{count}");
}

pub fn part2() {
    let input = read_to_string("data/day7.txt").unwrap();

    let mut count = 0;

    for line in input.lines() {
        if detect_ssl(line) {
            count += 1;
        }
    }

    println!("{count}");
}

fn detect_tls(input: &str) -> bool {
    let mut is_abba = false;

    let mut in_hypernet = false;

    for window in input.as_bytes().windows(4) {
        if window[0] == b'[' {
            in_hypernet = true;
        }

        if window[3] == b']' {
            in_hypernet = false;
        }

        if window.contains(&b'[') || window.contains(&b']') {
            continue;
        }

        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            if in_hypernet {
                is_abba = false;
                break;
            } else {
                is_abba = true;
            }
        }
    }

    is_abba
}

fn detect_ssl(input: &str) -> bool {
    let mut in_hypernet = false;

    let mut aba_list = Vec::new();
    let mut bab_list = Vec::new();

    for window in input.as_bytes().windows(3) {
        if window[0] == b'[' {
            in_hypernet = true;
        }

        if window[2] == b']' {
            in_hypernet = false;
        }

        if window.contains(&b'[') || window.contains(&b']') {
            continue;
        }

        if window[0] == window[2] && window[0] != window[1] {
            if in_hypernet {
                bab_list.push(format!(
                    "{}{}{}",
                    window[0] as char, window[1] as char, window[2] as char
                ));
            } else {
                aba_list.push(format!(
                    "{}{}{}",
                    window[0] as char, window[1] as char, window[2] as char
                ));
            }
        }
    }

    if aba_list.is_empty() || bab_list.is_empty() {
        return false;
    }

    for aba in aba_list {
        let bab_02 = aba.chars().nth(1).unwrap();
        let bab_1 = aba.chars().next().unwrap();

        if bab_list.contains(&format!("{bab_02}{bab_1}{bab_02}")) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "abba[mnop]qrst";

        assert_eq!(true, detect_tls(input));
    }

    #[test]
    fn test_2() {
        let input = "abcd[bddb]xyyx";

        assert_eq!(false, detect_tls(input));
    }

    #[test]
    fn test_3() {
        let input = "aaaa[qwer]tyui";

        assert_eq!(false, detect_tls(input));
    }

    #[test]
    fn test_4() {
        let input = "ioxxoj[asdfgh]zxcvbn";

        assert_eq!(true, detect_tls(input));
    }

    #[test]
    fn test_5() {
        let input = "aba[bab]xyz";

        assert_eq!(true, detect_ssl(input));
    }

    #[test]
    fn test_6() {
        let input = "xyx[xyx]xyx";

        assert_eq!(false, detect_ssl(input));
    }

    #[test]
    fn test_7() {
        let input = "aaa[kek]eke";

        assert_eq!(true, detect_ssl(input));
    }

    #[test]
    fn test_8() {
        let input = "zazbz[bzb]cdb";

        assert_eq!(true, detect_ssl(input));
    }
}
