pub fn part1() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();

    let safe_ips = get_safe_ips(&input, u32::MAX);

    println!("{}", *safe_ips.iter().min().unwrap());
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();

    let safe_ips = get_safe_ips(&input, u32::MAX);

    println!("{}", safe_ips.len());
}

fn get_safe_ips(input: &str, max: u32) -> Vec<u32> {
    let mut safe_ips: Vec<_> = (0..=max).map(|v| (v, 1_u8)).collect();

    for line in input.lines().filter(|l| !l.is_empty()).map(|l| l.trim()) {
        let mut line_parts = line.split('-');

        let start = &line_parts.next().unwrap().trim().parse::<usize>().unwrap();
        let end = &line_parts.next().unwrap().trim().parse::<usize>().unwrap();

        for ip in safe_ips.iter_mut().take(*end + 1).skip(*start) {
            ip.1 = 0;
        }
    }

    safe_ips.retain(|v| v.1 == 1);

    safe_ips.into_iter().map(|v| v.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let safe_ips = get_safe_ips(
            r"
            5-8
            0-2
            4-7",
            9,
        );

        assert_eq!(3, *safe_ips.iter().min().unwrap());
    }
}
