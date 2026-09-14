#[derive(Clone)]
struct Disk {
    position: u32,
    num_positions: u32,
}

impl Disk {
    pub fn tick(&mut self) {
        self.position += 1;

        if self.position >= self.num_positions {
            self.position = 0;
        }
    }
}

pub fn part1() {
    let mut disks = vec![
        Disk {
            position: 10,
            num_positions: 13,
        },
        Disk {
            position: 15,
            num_positions: 17,
        },
        Disk {
            position: 17,
            num_positions: 19,
        },
        Disk {
            position: 1,
            num_positions: 7,
        },
        Disk {
            position: 0,
            num_positions: 5,
        },
        Disk {
            position: 1,
            num_positions: 3,
        },
    ];
    
    let time = check_disks(&mut disks);
    
    println!("{}", time);
}

pub fn part2() {
    let mut disks = vec![
        Disk {
            position: 10,
            num_positions: 13,
        },
        Disk {
            position: 15,
            num_positions: 17,
        },
        Disk {
            position: 17,
            num_positions: 19,
        },
        Disk {
            position: 1,
            num_positions: 7,
        },
        Disk {
            position: 0,
            num_positions: 5,
        },
        Disk {
            position: 1,
            num_positions: 3,
        },
        Disk {
            position: 0,
            num_positions: 11,
        },
    ];
    
    let time = check_disks(&mut disks);
    
    println!("{}", time);
}

fn check_disks(disks: &mut Vec<Disk>) -> u64 {
    let mut time = 0;

    loop {
        let mut ticks = 1;

        let mut check_disks = disks.clone();

        for disk in &mut check_disks {
            for _ in 0..ticks {
                disk.tick();
            }

            ticks += 1;
        }

        if check_disks.iter().all(|d| d.position == 0) {
            break;
        }

        for disk in disks.iter_mut() {
            disk.tick();
        }

        time += 1;
    }

    time
}
