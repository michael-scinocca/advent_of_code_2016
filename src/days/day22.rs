pub fn part1() {
    let data = std::fs::read_to_string("data/day22.txt").unwrap();

    let nodes = data
        .lines()
        .enumerate()
        .filter(|l| !l.1.is_empty() && !l.1.starts_with('#'))
        .map(|l| {
            let mut parts = l.1.split(' ').filter(|p| p.ends_with('T'));

            parts.next().unwrap();

            (
                l.0,
                parts
                    .next()
                    .unwrap()
                    .trim()
                    .strip_suffix('T')
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
                parts
                    .next()
                    .unwrap()
                    .trim()
                    .strip_suffix('T')
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut pairs = 0;

    for node in &nodes {
        pairs += nodes
            .iter()
            .filter(|n| n.0 != node.0 && node.1 != 0 && n.2 >= node.1)
            .count();
    }

    println!("{}", pairs);
}

pub fn part2() {}
