pub fn part1() {
    let room = generate_room(
        "^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^",
        40,
    );

    let mut safe_count = 0;

    for row in room {
        println!("{}", row);

        for c in row.chars() {
            if c == '.' {
                safe_count += 1;
            }
        }
    }

    println!("{}", safe_count);
}

pub fn part2() {
    let room = generate_room(
        "^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^",
        400000,
    );

    let mut safe_count = 0;

    for row in room {
        for c in row.chars() {
            if c == '.' {
                safe_count += 1;
            }
        }
    }

    println!("{}", safe_count);
}

fn generate_room(first_row: &str, rows: u32) -> Vec<String> {
    let mut room = Vec::new();
    room.push(first_row.to_string());

    let mut next_row = first_row.to_string();

    for _ in 1..rows {
        next_row = generate_next_row(&next_row);

        room.push(next_row.clone());
    }

    room
}

fn generate_next_row(row: &str) -> String {
    let mut next_row = String::with_capacity(row.len());

    let row_test = format!(".{row}.");

    for window in row_test.as_bytes().windows(3) {
        let cell = match (window[0], window[1], window[2]) {
            (b'^', b'^', b'.') => '^',
            (b'.', b'^', b'^') => '^',
            (b'^', b'.', b'.') => '^',
            (b'.', b'.', b'^') => '^',
            _ => '.',
        };

        next_row.push(cell);
    }

    next_row
}
