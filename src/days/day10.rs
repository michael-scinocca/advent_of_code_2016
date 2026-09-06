use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();

    let (bot, _outputs) = run_bots(&input, 17, 61);

    println!("{bot}");
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();

    let (_bot, outputs) = run_bots(&input, 0, 0);

    println!(
        "{}",
        outputs.get(&0).unwrap()[0] * outputs.get(&1).unwrap()[0] * outputs.get(&2).unwrap()[0]
    );
}

pub fn run_bots(input: &str, low_check: u32, high_check: u32) -> (u32, HashMap<u32, Vec<u32>>) {
    let input = input.trim();

    let mut output_bot = 0;

    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();

    for line in input.lines().map(|l| l.trim()) {
        if line.trim().starts_with("value") {
            let mut line_parts = line.split(' ');

            let value = line_parts.nth(1).unwrap().parse::<u32>().unwrap();
            let bot = line_parts.nth(3).unwrap().parse::<u32>().unwrap();

            bots.entry(bot)
                .and_modify(|f: &mut Vec<_>| f.push(value))
                .or_insert(vec![value]);
        }
    }

    loop {
        for line in input.lines().map(|l| l.trim()) {
            if line.trim().starts_with("bot") {
                let mut line_parts = line.split(' ');

                let bot = line_parts.nth(1).unwrap().parse::<u32>().unwrap();

                if !bots.contains_key(&bot) || bots.get(&bot).unwrap().len() != 2 {
                    continue;
                }

                let low_dest = line_parts.nth(3).unwrap();
                let low_id = line_parts.next().unwrap().parse::<u32>().unwrap();
                let high_dest = line_parts.nth(3).unwrap();
                let high_id = line_parts.next().unwrap().parse::<u32>().unwrap();

                let low_value = *bots.get(&bot).unwrap().iter().min().unwrap();
                let high_value = *bots.get(&bot).unwrap().iter().max().unwrap();

                bots.entry(bot)
                    .and_modify(|f| f.retain(|r| *r != low_value));
                bots.entry(bot)
                    .and_modify(|f| f.retain(|r| *r != high_value));

                if low_value == low_check && high_value == high_check {
                    output_bot = bot;
                }

                if low_dest == "bot" {
                    bots.entry(low_id)
                        .and_modify(|f: &mut Vec<_>| f.push(low_value))
                        .or_insert(vec![low_value]);
                } else {
                    outputs
                        .entry(low_id)
                        .and_modify(|f: &mut Vec<_>| f.push(low_value))
                        .or_insert(vec![low_value]);
                }

                if high_dest == "bot" {
                    bots.entry(high_id)
                        .and_modify(|f: &mut Vec<_>| f.push(high_value))
                        .or_insert(vec![high_value]);
                } else {
                    outputs
                        .entry(high_id)
                        .and_modify(|f: &mut Vec<_>| f.push(high_value))
                        .or_insert(vec![high_value]);
                }
            }
        }

        let mut done = true;

        for bot_chips in bots.values() {
            if !bot_chips.is_empty() {
                done = false;

                break;
            }
        }

        if done {
            break;
        }
    }

    (output_bot, outputs)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let (bot, outputs) = run_bots(
            r"
        value 5 goes to bot 2
        bot 2 gives low to bot 1 and high to bot 0
        value 3 goes to bot 1
        bot 1 gives low to output 1 and high to bot 0
        bot 0 gives low to output 2 and high to output 0
        value 2 goes to bot 2",
            2,
            5,
        );

        assert_eq!(2, bot);
        assert!(outputs.get(&0).unwrap().contains(&5));
        assert!(outputs.get(&1).unwrap().contains(&2));
        assert!(outputs.get(&2).unwrap().contains(&3));
    }
}
