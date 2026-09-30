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

pub fn part2() {
    let data = std::fs::read_to_string("data/day22.txt").unwrap();

    let nodes = data
        .lines()
        .enumerate()
        .map(|l| (l.0, l.1.trim()))
        .filter(|l| !l.1.is_empty() && !l.1.starts_with('#'))
        .map(|l| {
            let mut parts = l.1.split(' ').filter(|p| !p.is_empty());

            let mut address = parts
                .next()
                .unwrap()
                .strip_prefix("/dev/grid/node-")
                .unwrap()
                .split('-');

            Cell {
                position: Position {
                    x: address
                        .next()
                        .unwrap()
                        .strip_prefix('x')
                        .unwrap()
                        .parse()
                        .unwrap(),
                    y: address
                        .next()
                        .unwrap()
                        .strip_prefix('y')
                        .unwrap()
                        .parse()
                        .unwrap(),
                },
                used: parts
                    .nth(1)
                    .unwrap()
                    .trim()
                    .strip_suffix('T')
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
                available: parts
                    .next()
                    .unwrap()
                    .trim()
                    .strip_suffix('T')
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let position = {
        let most_available = nodes.iter().max_by(|x, y| x.available.cmp(&y.available));

        most_available
            .map(|most_available| (most_available.position.clone(), most_available.available))
    }
    .unwrap();

    let limit_x = nodes.iter().map(|n| n.position.x).max().unwrap() + 1;
    let limit_y = nodes.iter().map(|n| n.position.y).max().unwrap() + 1;

    let mut grid = Vec::new();

    for _ in 0..limit_x {
        grid.push(vec!["."; limit_y]);
    }

    for cell in &nodes {
        grid[cell.position.x][cell.position.y] = {
            if cell.used > position.1 {
                "#"
            } else if cell.available == position.1 {
                "_"
            } else if cell.position.x == limit_x - 1 && cell.position.y == 0 {
                "G"
            } else {
                "."
            }
        };
    }

    for y in 0..grid[0].len() {
        for row in &grid {
            print!("{} ", row[y]);
        }

        println!();
    }

    let mut moves = 0;
    moves += position.0.y; // move up to first row
    moves += limit_x - position.0.x - 1; // move over to goal
    moves += (limit_x - 2) * 5; // loop of 5 per slot to move goal to start
    moves += 6; // lateral move over and back to get around obstacle

    println!("{}", moves);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
struct Cell {
    pub position: Position,
    pub used: u32,
    pub available: u32,
}
