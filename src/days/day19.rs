pub fn part1() {
    let mut elves = [1; 3005290].to_vec();

    let elf = 'outer: loop {
        for elf in 0..elves.len() {
            if elves[elf] == 0 {
                continue;
            }

            let mut next_elf = elf + 1;

            loop {
                if next_elf >= elves.len() {
                    next_elf = 0;
                }

                if next_elf == elf {
                    break 'outer elf + 1;
                }

                if elves[next_elf] != 0 {
                    break;
                }

                next_elf += 1;
            }

            elves[elf] += elves[next_elf];
            elves[next_elf] = 0;
        }
    };

    println!("{}", elf);
}

pub fn part2() {
    println!("Day 19 Part 2");
}
