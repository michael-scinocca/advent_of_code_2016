use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1() {
    let input = std::fs::read_to_string("data/day24.txt").unwrap();

    let (map, goals) = construct_map(&input);

    let moves = find_lowest_moves(&map, &goals);

    println!("{}", moves);
}

struct Goal {
    id: u8,
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Trip {
    start_id: u8,
    end_id: u8,
    moves: u32,
}

fn run_navigation(map: &[Vec<u8>], x: usize, y: usize, goal: (usize, usize)) -> u32 {
    let map_width = map[0].len();
    let map_height = map.len();

    let mut queue = VecDeque::new();

    let mut visited = HashSet::new();

    queue.push_back((x, y, 0));

    loop {
        let Some((x, y, mut moves)) = queue.pop_front() else {
            break 0;
        };

        if x == goal.0 && y == goal.1 {
            break moves;
        }

        let next_moves = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        moves += 1;

        for next_move in next_moves {
            if next_move.0 >= map_width
                || next_move.1 >= map_height
                || map[next_move.1][next_move.0] == 0
                || visited.contains(&next_move)
            {
                continue;
            }

            visited.insert(next_move);

            queue.push_back((next_move.0, next_move.1, moves));
        }
    }
}

fn find_lowest_moves(map: &[Vec<u8>], goals: &[Goal]) -> u32 {
    let mut move_graph: HashMap<(u8, u8), Trip> = HashMap::new();

    for i in 0..goals.len() {
        for j in 0..goals.len() {
            if i == j {
                continue;
            }

            if move_graph.contains_key(&(goals[j].id, goals[i].id)) {
                let trip = move_graph.get(&(goals[j].id, goals[i].id)).unwrap().clone();
                
                move_graph.insert(
                    (goals[i].id, goals[j].id),
                    Trip {
                        start_id: trip.end_id,
                        end_id: trip.start_id,
                        moves: trip.moves,
                    },
                );

                continue;
            }

            let moves = run_navigation(map, goals[i].x, goals[i].y, (goals[j].x, goals[j].y));

            move_graph.insert(
                (goals[i].id, goals[j].id),
                Trip {
                    start_id: goals[i].id,
                    end_id: goals[j].id,
                    moves,
                },
            );
        }
    }

    let mut trips = Vec::new();

    get_all_trips(&move_graph, 0, goals.len(), &mut vec![(0, 0)], &mut trips);

    *trips.iter().min().unwrap()
}

fn get_all_trips(
    move_graph: &HashMap<(u8, u8), Trip>,
    current_id: u8,
    total_ids: usize,
    trip: &mut Vec<(u8, u32)>,
    trips: &mut Vec<u32>,
) {
    let links: Vec<&Trip> = move_graph.values().filter(|e| e.start_id == current_id).collect();
    
    for link in &links {
        if trip.iter().any(|e| e.0 == link.end_id) {
            continue;
        }

        trip.push((link.end_id, link.moves));

        if trip.len() == total_ids {
            let trip_total: u32 = trip.iter().map(|l| l.1).sum();
            
            trips.push(trip_total);

            trip.pop();

            continue;
        }

        get_all_trips(move_graph, link.end_id, total_ids, trip, trips);
    }

    trip.pop();
}

#[allow(unused)]
fn print_map(map: &[Vec<u8>]) {
    for row in map {
        for col in row {
            print!("{}", col);
        }

        println!();
    }
}

fn construct_map(input: &str) -> (Vec<Vec<u8>>, Vec<Goal>) {
    let mut map = Vec::new();

    let mut goals = Vec::new();

    for line in input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|l| (l.0, l.1.trim()))
    {
        let map_row = line
            .1
            .char_indices()
            .map(|c| match c.1 {
                '#' => 0,
                '.' => 1,
                goal => {
                    let goal_val = goal.to_string().parse::<u8>().unwrap();
                    goals.push(Goal {
                        id: goal_val,
                        x: c.0,
                        y: line.0,
                    });
                    goal_val + 2
                }
            })
            .collect();

        map.push(map_row);
    }

    (map, goals)
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
            ###########
            #0.1.....2#
            #.#######.#
            #4.......3#
            ###########
        ";

        let (map, goals) = construct_map(input);

        let moves = find_lowest_moves(&map, &goals);

        assert_eq!(14, moves);
    }
}
