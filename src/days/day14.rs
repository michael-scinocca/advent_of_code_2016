use std::collections::HashMap;

pub fn part1() {
    let index = generate("qzyelonm", 64, hash_generation);

    println!("{}", index);
}

pub fn part2() {
    let index = generate("qzyelonm", 64, hash_generation_2);

    println!("{}", index);
}

fn hash_generation(seed: &str, index: u64) -> String {
    let hash_data = md5::compute(format!("{seed}{}", index));
    format!("{:x}", hash_data)
}

fn hash_generation_2(seed: &str, index: u64) -> String {
    let hash_data = md5::compute(format!("{seed}{}", index));
    let mut hash = format!("{:x}", hash_data);

    for _ in 0..2016 {
        let hash_data = md5::compute(hash);
        hash = format!("{:x}", hash_data);
    }

    hash
}

fn generate(seed: &str, key_num: u64, hash_generation: impl Fn(&str, u64) -> String) -> u64 {
    let mut index = 0;

    let mut generated = 0;

    let mut hash_cache = HashMap::new();

    loop {
        if !is_hash_valid(&mut hash_cache, seed, index, &hash_generation) {
            index += 1;
            continue;
        }

        generated += 1;

        if generated == key_num {
            break;
        }

        index += 1;
    }

    index
}

fn is_hash_valid(
    hash_cache: &mut HashMap<u64, String>,
    seed: &str,
    index: u64,
    hash_generation: impl Fn(&str, u64) -> String,
) -> bool {
    let hash = hash_cache
        .entry(index)
        .or_insert_with(|| hash_generation(seed, index));

    let mut repeat_char: Option<u8> = None;

    for window in hash.as_bytes().windows(3) {
        if window[0] == window[1] && window[1] == window[2] {
            repeat_char = Some(window[0]);
            break;
        }
    }

    let Some(char) = repeat_char else {
        return false;
    };

    for check_index in index + 1..index + 1 + 1000 {
        let hash = hash_cache
            .entry(check_index)
            .or_insert_with(|| hash_generation(seed, check_index));

        for window in hash.as_bytes().windows(5) {
            if window.iter().all(|w| *w == char) {
                return true;
            }
        }
    }

    false
}
