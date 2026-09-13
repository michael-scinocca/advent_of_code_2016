use std::collections::HashSet;

pub fn part1() {
    let grid = construct_grid(1358);

    let steps = run(&grid, (31, 39), 0);

    let Some(steps) = steps else {
        println!("No path");
        return;
    };

    println!("{}", steps);
}

pub fn part2() {
    let mut locations = 0;

    let grid = construct_grid(1358);

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let steps = run(&grid, (x, y), 50);

            let Some(steps) = steps else {
                continue;
            };

            if steps <= 50 {
                locations += 1;
            }
        }
    }

    println!("{}", locations);
}

const GRID_SIZE: usize = 50;

fn run(
    grid: &[[u8; GRID_SIZE]; GRID_SIZE],
    destination: (usize, usize),
    steps_max: u32,
) -> Option<u32> {
    let mut steps = 0;

    let position = (1, 1);

    let mut next_positions = get_next_positions(&position, grid);

    let mut visited = HashSet::new();

    for next_position in &next_positions {
        visited.insert(*next_position);
    }

    loop {
        steps += 1;

        if steps_max > 0 && steps > steps_max {
            return None;
        }

        let mut next_up = Vec::new();

        for next_position in &next_positions {
            if next_position.0 == destination.0 && next_position.1 == destination.1 {
                return Some(steps);
            }

            next_up.extend(get_next_positions(next_position, grid));
        }

        next_positions.clear();

        for next in &next_up {
            if visited.insert(*next) {
                next_positions.push(*next);
            }
        }
    }
}

fn get_next_positions(
    position: &(usize, usize),
    grid: &[[u8; GRID_SIZE]; GRID_SIZE],
) -> Vec<(usize, usize)> {
    let mut next_positions = Vec::new();

    let (x, y) = *position;

    if x > 0 && grid[x - 1][y] != 1 {
        next_positions.push((x - 1, y));
    }

    if x < GRID_SIZE - 1 && grid[x + 1][y] != 1 {
        next_positions.push((x + 1, y));
    }

    if y > 0 && grid[x][y - 1] != 1 {
        next_positions.push((x, y - 1));
    }

    if y < GRID_SIZE - 1 && grid[x][y + 1] != 1 {
        next_positions.push((x, y + 1));
    }

    next_positions
}

fn construct_grid(num: u32) -> [[u8; GRID_SIZE]; GRID_SIZE] {
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for (x, row) in grid.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            *cell = get_grid_type(x, y, num);
        }
    }

    grid
}

fn get_grid_type(x: usize, y: usize, num: u32) -> u8 {
    let sum = ((x * x + 3 * x + 2 * x * y + y + y * y) + num as usize) as u32;

    let mut xor = 0;

    for index in 0..32 {
        xor ^= sum >> index & 1;
    }

    xor as u8
}
