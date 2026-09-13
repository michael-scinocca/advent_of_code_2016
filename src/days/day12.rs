use std::fs::read_to_string;

struct Instruction;

impl Instruction {
    const CPY: u8 = 0;
    const CPR: u8 = 1;
    const INC: u8 = 2;
    const DEC: u8 = 3;
    const JNZ: u8 = 4;
    const JMP: u8 = 5;
}

#[derive(Debug)]
struct Cpu {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    reg_d: i32,
    pc: usize,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            pc: 0,
        }
    }

    fn pc_index(&self) -> usize {
        self.pc * 3
    }

    fn execute(&mut self, program: &[u8]) -> bool {
        if self.pc_index() > program.len() - 3 {
            return false;
        }

        match program[self.pc_index()] {
            Instruction::CPY => {
                let value = &program[self.pc_index() + 1];
                let register = self.get_register(&program[self.pc_index() + 2]);

                *register = *value as i32;
                self.pc += 1;
                true
            }
            Instruction::CPR => {
                let value = *self.get_register(&program[self.pc_index() + 1]);
                let register = self.get_register(&program[self.pc_index() + 2]);

                *register = value;
                self.pc += 1;
                true
            }
            Instruction::INC => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                *register += 1;
                self.pc += 1;
                true
            }
            Instruction::DEC => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                *register -= 1;
                self.pc += 1;
                true
            }
            Instruction::JNZ => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                if *register == 0 {
                    self.pc += 1;
                    return true;
                }

                self.jump(&program[self.pc_index() + 2]);
                true
            }
            Instruction::JMP => {
                self.jump(&program[self.pc_index() + 1]);
                true
            }
            _ => false,
        }
    }

    fn get_register(&mut self, register_id: &u8) -> &mut i32 {
        match register_id {
            0 => &mut self.reg_a,
            1 => &mut self.reg_b,
            2 => &mut self.reg_c,
            3 => &mut self.reg_d,
            _ => panic!("Unknown register {register_id}"),
        }
    }

    fn jump(&mut self, offset: &u8) {
        let jump = *offset as i8 as isize;

        self.pc = self.pc.checked_add_signed(jump).unwrap();
    }
}

fn compile_program(program_data: &str) -> Vec<u8> {
    let mut program = vec![0; program_data.lines().filter(|l| !l.is_empty()).count() * 3];

    let mut pc = 0;

    for line in program_data.lines() {
        if line.is_empty() {
            continue;
        }

        let mut line_parts = line.trim().split(" ");

        match line_parts.next().unwrap() {
            "cpy" => {
                let part = line_parts.next().unwrap();

                match part.parse::<u8>() {
                    Ok(val) => {
                        program[pc] = Instruction::CPY;
                        program[pc + 1] = val;
                    }
                    Err(_) => {
                        program[pc] = Instruction::CPR;
                        program[pc + 1] = get_register(part);
                    }
                }

                program[pc + 2] = get_register(line_parts.next().unwrap());
            }
            "inc" => {
                program[pc] = Instruction::INC;
                program[pc + 1] = get_register(line_parts.next().unwrap());
            }
            "dec" => {
                program[pc] = Instruction::DEC;
                program[pc + 1] = get_register(line_parts.next().unwrap());
            }
            "jnz" => {
                let part = line_parts.next().unwrap();

                match part.parse::<u8>() {
                    Ok(val) => {
                        program[pc] = Instruction::JMP;
                        program[pc + 1] = {
                            if val == 0 {
                                1
                            } else {
                                line_parts.next().unwrap().parse::<i8>().unwrap() as u8
                            }
                        };
                    }
                    Err(_) => {
                        program[pc] = Instruction::JNZ;
                        program[pc + 1] = get_register(part);
                        program[pc + 2] = line_parts.next().unwrap().parse::<i8>().unwrap() as u8;
                    }
                }
            }
            _ => {}
        }

        pc += 3;
    }

    program
}

fn get_register(register_name: &str) -> u8 {
    match register_name {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Unknown register {register_name}"),
    }
}

pub fn part1() {
    let input = read_to_string("data/day12.txt").unwrap();

    let program = compile_program(&input);

    let mut cpu = Cpu::new();

    while cpu.execute(&program) {
        println!("{:?}", cpu);
    }
}

pub fn part2() {
    let input = read_to_string("data/day12.txt").unwrap();

    let program = compile_program(&input);

    let mut cpu = Cpu {
        reg_c: 1,
        ..Cpu::new()
    };

    while cpu.execute(&program) {
        println!("{:?}", cpu);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
            cpy 41 a
            inc a
            inc a
            dec a
            jnz a 2
            dec a";

        let program = compile_program(input);

        let mut cpu = Cpu::new();

        while cpu.execute(&program) {}

        assert_eq!(42, cpu.reg_a);
    }

    #[test]
    fn test2() {
        let input = r"cpy 41 a";

        let program = compile_program(input);

        let mut cpu = Cpu::new();

        while cpu.execute(&program) {}

        assert_eq!(41, cpu.reg_a);
    }

    #[test]
    fn test3() {
        let input = r"
            cpy 41 b
            cpy b a";

        let program = compile_program(input);

        let mut cpu = Cpu::new();

        while cpu.execute(&program) {}

        assert_eq!(41, cpu.reg_a);
    }
}
