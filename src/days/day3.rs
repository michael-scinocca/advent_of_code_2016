use std::fs::read_to_string;

pub fn part1() {
    let input = read_to_string("data/day3.txt").unwrap();

    let mut valid_count = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        let sides: Vec<_> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();

        if is_valid_triangle(sides[0], sides[1], sides[2]) {
            valid_count += 1;
        }
    }

    println!("{valid_count}");
}

pub fn part2() {
    let input = read_to_string("data/day3.txt").unwrap();

    let mut valid_count = 0;

    let mut triangles = Vec::new();

    let mut triangle1 = Vec::new();
    let mut triangle2 = Vec::new();
    let mut triangle3 = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let sides: Vec<_> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();

        triangle1.push(sides[0]);
        triangle2.push(sides[1]);
        triangle3.push(sides[2]);

        if triangle1.len() == 3 {
            triangles.push(triangle1.clone());
            triangle1.clear();

            triangles.push(triangle2.clone());
            triangle2.clear();

            triangles.push(triangle3.clone());
            triangle3.clear();
        }
    }

    for triangle in triangles {
        if is_valid_triangle(triangle[0], triangle[1], triangle[2]) {
            valid_count += 1;
        }
    }

    println!("{valid_count}");
}

fn is_valid_triangle(side_1: u32, side_2: u32, side_3: u32) -> bool {
    side_1 + side_2 > side_3 && side_1 + side_3 > side_2 && side_2 + side_3 > side_1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, is_valid_triangle(5, 10, 25));
    }
}
