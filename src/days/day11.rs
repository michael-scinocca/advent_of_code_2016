use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
};

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
enum PartType {
    Generator,
    Microchip,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Part {
    part_type: PartType,
    name: String,
    floor: usize,
}

impl Part {
    pub fn new(part_type: PartType, name: &str, floor: usize) -> Self {
        Self {
            part_type,
            name: name.into(),
            floor,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Layout {
    parts: Vec<Part>,
    current_floor: usize,
}

impl Layout {
    pub fn new(parts: Vec<Part>) -> Self {
        Self {
            parts,
            current_floor: 1,
        }
    }

    fn floors(&self) -> BTreeMap<usize, Vec<&Part>> {
        let mut floors: BTreeMap<usize, Vec<&Part>> = BTreeMap::new();

        for part in &self.parts {
            floors
                .entry(part.floor)
                .and_modify(|f| f.push(part))
                .or_insert(vec![part]);
        }

        floors
    }

    pub fn is_valid(&self) -> bool {
        for floor in self.floors() {
            let generators: HashSet<_> = floor
                .1
                .iter()
                .filter_map(|p| match p.part_type {
                    PartType::Generator => Some(&p.name),
                    _ => None,
                })
                .collect();

            if generators.is_empty() {
                continue;
            }

            if !floor.1.iter().all(|p| generators.contains(&p.name)) {
                return false;
            }
        }

        true
    }

    pub fn is_complete(&self) -> bool {
        self.parts.iter().all(|p| p.floor == 4)
    }

    pub fn run(&self) -> u32 {
        let mut steps = 0;

        let mut next_gen = self.next_gen();

        let mut visited = HashSet::new();

        for next in &next_gen {
            visited.insert(next.clone());
        }

        loop {
            steps += 1;

            let mut next_up = Vec::new();

            for next in &next_gen {
                if next.is_complete() {
                    return steps;
                }

                next_up.extend(next.next_gen());
            }

            next_gen.clear();

            for next in &next_up {
                if visited.insert(next.clone()) {
                    next_gen.push(next.clone());
                }
            }
        }
    }

    fn next_gen(&self) -> Vec<Layout> {
        let mut next_gen = Vec::new();

        let mut next_floors = Vec::new();

        if self.current_floor < 4 {
            next_floors.push(self.current_floor + 1);
        }

        if self.current_floor > 1 {
            next_floors.push(self.current_floor - 1);
        }

        let floor_parts_count = self
            .parts
            .iter()
            .filter(|p| p.floor == self.current_floor)
            .count();

        for next_floor in next_floors {
            Self::move_parts(&mut next_gen, self, next_floor, floor_parts_count, 0);
        }

        next_gen
    }

    fn move_parts(
        next_gen: &mut Vec<Layout>,
        current: &Layout,
        next_floor: usize,
        floor_parts_count: usize,
        level: usize,
    ) {
        for index in 0..floor_parts_count - level {
            let mut next = current.clone();

            let mut next_current_floor_parts: Vec<_> = next
                .parts
                .iter_mut()
                .filter(|p| p.floor == next.current_floor)
                .collect();

            next_current_floor_parts[index].floor = next_floor;

            if next.is_valid() {
                let mut next = next.clone();
                next.current_floor = next_floor;

                next_gen.push(next);
            }

            if level == 0 {
                Self::move_parts(next_gen, &next, next_floor, floor_parts_count, level + 1);
            }
        }
    }
}

pub fn part1() {
    let layout = Layout::new(vec![
        Part::new(PartType::Generator, "T", 1),
        Part::new(PartType::Microchip, "T", 1),
        Part::new(PartType::Generator, "P", 1),
        Part::new(PartType::Generator, "S", 1),
        Part::new(PartType::Microchip, "P", 2),
        Part::new(PartType::Microchip, "S", 2),
        Part::new(PartType::Generator, "PR", 3),
        Part::new(PartType::Microchip, "PR", 3),
        Part::new(PartType::Generator, "R", 3),
        Part::new(PartType::Microchip, "R", 3),
    ]);

    let steps = layout.run();

    println!("{}", steps);
}

pub fn part2() {
    let layout = Layout::new(vec![
        Part::new(PartType::Generator, "E", 1),
        Part::new(PartType::Microchip, "E", 1),
        Part::new(PartType::Generator, "D", 1),
        Part::new(PartType::Microchip, "D", 1),
        Part::new(PartType::Generator, "T", 1),
        Part::new(PartType::Microchip, "T", 1),
        Part::new(PartType::Generator, "P", 1),
        Part::new(PartType::Generator, "S", 1),
        Part::new(PartType::Microchip, "P", 2),
        Part::new(PartType::Microchip, "S", 2),
        Part::new(PartType::Generator, "PR", 3),
        Part::new(PartType::Microchip, "PR", 3),
        Part::new(PartType::Generator, "R", 3),
        Part::new(PartType::Microchip, "R", 3),
    ]);

    let steps = layout.run();

    println!("{}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let layout1 = Layout::new(vec![]);
        assert_eq!(true, layout1.is_valid());

        let layout2 = Layout::new(vec![Part::new(PartType::Microchip, "s", 1)]);
        assert_eq!(true, layout2.is_valid());

        let layout3 = Layout::new(vec![
            Part::new(PartType::Microchip, "s", 1),
            Part::new(PartType::Generator, "t", 1),
        ]);
        assert_ne!(true, layout3.is_valid());

        let layout4 = Layout::new(vec![
            Part::new(PartType::Microchip, "s", 1),
            Part::new(PartType::Generator, "t", 1),
            Part::new(PartType::Generator, "s", 1),
        ]);
        assert_eq!(true, layout4.is_valid());
    }

    #[test]
    fn test2() {
        let layout = Layout::new(vec![
            Part::new(PartType::Microchip, "H", 1),
            Part::new(PartType::Microchip, "L", 1),
            Part::new(PartType::Generator, "H", 2),
            Part::new(PartType::Generator, "L", 3),
        ]);

        assert_eq!(true, layout.is_valid());

        let next_gen = &layout.next_gen();

        assert_eq!(1, next_gen.iter().count());

        assert!(
            next_gen[0]
                .parts
                .iter()
                .find(|p| p.floor == 2 && p.name == "H" && p.part_type == PartType::Microchip)
                .is_some()
        );
    }

    #[test]
    fn test3() {
        let layout = Layout::new(vec![
            Part::new(PartType::Microchip, "H", 4),
            Part::new(PartType::Microchip, "L", 4),
            Part::new(PartType::Generator, "H", 4),
            Part::new(PartType::Generator, "L", 4),
        ]);

        assert_eq!(true, layout.is_complete());
    }

    #[test]
    fn test4() {
        let layout = Layout::new(vec![
            Part::new(PartType::Microchip, "H", 1),
            Part::new(PartType::Microchip, "L", 1),
            Part::new(PartType::Generator, "H", 2),
            Part::new(PartType::Generator, "L", 3),
        ]);

        let steps = layout.run();

        assert_eq!(11, steps);
    }
}
