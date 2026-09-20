pub fn part1() {
    let mut elves: Vec<_> = (1..=3005290).map(|i| (i as u32, 1_u8)).collect();

    let elf = loop {
        for elf in 0..elves.len() {
            if elves[elf].1 == 0 {
                continue;
            }

            let next_elf = { if elf >= elves.len() - 1 { 0 } else { elf + 1 } };

            elves[next_elf].1 = 0;
        }

        elves.retain(|e| e.1 != 0);

        if elves.len() == 1 {
            break elves[0].0;
        }
    };

    println!("{}", elf);
}

pub fn part2() {
    let mut elves: Vec<_> = (1..=3005290).collect();

    let mut elf = 0;

    let elf = loop {
        let mut next_elf = elf + elves.len() / 2;

        if next_elf >= elves.len() {
            next_elf -= elves.len();
        }

        elves.remove(next_elf);

        if elves.len() == 1 {
            break elves[0];
        }

        if elf == elves.len() - 1 {
            continue;
        }

        if next_elf > elf {
            elf += 1;
        }

        if elf >= elves.len() {
            elf = 0;
        }
    };

    println!("{}", elf);
}
