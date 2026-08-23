pub fn part1() {
    println!("{}", get_password("wtnhxymk"));
}

pub fn part2() {
    println!("{}", get_password_2("wtnhxymk"));
}

fn get_password(input: &str) -> String {
    let mut password = String::new();

    let mut number = 0;

    loop {
        let hash = md5::compute(format!("{input}{}", number));
        let hash_hex = format!("{:x}", hash);

        if hash_hex.starts_with("00000") {
            password.push(hash_hex.chars().nth(5).unwrap());

            if password.len() == 8 {
                break;
            }
        }

        number += 1;
    }

    password
}

fn get_password_2(input: &str) -> String {
    let mut password = String::from("________");

    let mut number = 0;

    loop {
        let hash = md5::compute(format!("{input}{}", number));
        let hash_hex = format!("{:x}", hash);

        if hash_hex.starts_with("00000") {
            let mut hash_hex_chars = hash_hex.chars();

            let position = hash_hex_chars.nth(5).unwrap();

            if let Some(index) = position.to_digit(10) {
                let index = index as usize;

                if index < 8 && password.chars().nth(index).unwrap() == '_' {
                    let password_character = hash_hex_chars.nth(0).unwrap().to_string();

                    password.replace_range(index..=index, &password_character);

                    println!("{password}");
                }
            }

            if !password.contains('_') {
                break;
            }
        }

        number += 1;
    }

    password
}
