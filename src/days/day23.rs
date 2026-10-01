use std::fs::read_to_string;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum Instruction {
    CPY = 0,
    CPR = 1,
    INC = 2,
    DEC = 3,
    JNZ = 4,
    JMP = 5,
    TGL = 6,
    JNZR = 7,
    JMPR = 8,
}

impl TryFrom<u8> for Instruction {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CPY),
            1 => Ok(Self::CPR),
            2 => Ok(Self::INC),
            3 => Ok(Self::DEC),
            4 => Ok(Self::JNZ),
            5 => Ok(Self::JMP),
            6 => Ok(Self::TGL),
            7 => Ok(Self::JNZR),
            8 => Ok(Self::JMPR),
            other => Err(other),
        }
    }
}

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        value as u8
    }
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

    fn execute(&mut self, program: &mut [u8]) -> bool {
        if self.pc_index() > program.len() - 3 {
            return false;
        }

        let Ok(instruction) = Instruction::try_from(program[self.pc_index()]) else {
            panic!()
        };

        match instruction {
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
                let check_val = program[self.pc_index() + 1];

                if check_val == 0 {
                    self.pc += 1;
                    return true;
                }

                self.jump(&program[self.pc_index() + 2]);
                true
            }
            Instruction::JNZR => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                if *register == 0 {
                    self.pc += 1;
                    return true;
                }

                let offset = *self.get_register(&program[self.pc_index() + 2]);
                self.jump(&(offset as u8));
                true
            }
            Instruction::JMPR => {
                let check_val = program[self.pc_index() + 1];

                if check_val == 0 {
                    self.pc += 1;
                    return true;
                }

                let offset = *self.get_register(&program[self.pc_index() + 2]);
                self.jump(&(offset as u8));
                true
            }
            Instruction::TGL => {
                let register = *self.get_register(&program[self.pc_index() + 1]);
                let next_index = (self.pc_index() as i32 + register * 3) as usize;

                if next_index > program.len() - 3 {
                    self.pc += 1;
                    return true;
                }

                let Ok(toggle_instruction) = Instruction::try_from(program[next_index]) else {
                    panic!();
                };

                let toggle = match toggle_instruction {
                    Instruction::INC => Instruction::DEC,
                    Instruction::DEC => Instruction::INC,
                    Instruction::CPY => Instruction::JMPR,
                    Instruction::CPR => Instruction::JNZR,
                    Instruction::JNZ | Instruction::JNZR => Instruction::CPR,
                    Instruction::JMP | Instruction::JMPR => Instruction::CPY,
                    Instruction::TGL => Instruction::INC,
                };

                program[next_index] = toggle.into();
                self.pc += 1;

                true
            }
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

                match part.parse::<i8>() {
                    Ok(val) => {
                        program[pc] = Instruction::CPY.into();
                        program[pc + 1] = val as u8;
                    }
                    Err(_) => {
                        program[pc] = Instruction::CPR.into();
                        program[pc + 1] = get_register(part);
                    }
                }

                program[pc + 2] = get_register(line_parts.next().unwrap());
            }
            "inc" => {
                program[pc] = Instruction::INC.into();
                program[pc + 1] = get_register(line_parts.next().unwrap());
            }
            "dec" => {
                program[pc] = Instruction::DEC.into();
                program[pc + 1] = get_register(line_parts.next().unwrap());
            }
            "jnz" => {
                let part = line_parts.next().unwrap();
                let next_part = line_parts.next().unwrap();

                match part.parse::<i8>() {
                    Ok(val) => match next_part.parse::<i8>() {
                        Ok(jmp) => {
                            program[pc] = Instruction::JMP.into();
                            program[pc + 1] = val as u8;
                            program[pc + 2] = jmp as u8;
                        }
                        Err(_) => {
                            program[pc] = Instruction::JMPR.into();
                            program[pc + 1] = val as u8;
                            program[pc + 2] = get_register(next_part);
                        }
                    },
                    Err(_) => match next_part.parse::<i8>() {
                        Ok(jmp) => {
                            program[pc] = Instruction::JNZ.into();
                            program[pc + 1] = get_register(part);
                            program[pc + 2] = jmp as u8;
                        }
                        Err(_) => {
                            program[pc] = Instruction::JNZR.into();
                            program[pc + 1] = get_register(part);
                            program[pc + 2] = get_register(next_part);
                        }
                    },
                }
            }
            "tgl" => {
                program[pc] = Instruction::TGL.into();
                program[pc + 1] = get_register(line_parts.next().unwrap());
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
    let input = read_to_string("data/day23.txt").unwrap();

    let mut program = compile_program(&input);

    let mut cpu = Cpu {
        reg_a: 7,
        ..Cpu::new()
    };

    while cpu.execute(&mut program) {}

    println!("{:?}", cpu);
}

pub fn part2() {
    let input = read_to_string("data/day23.txt").unwrap();

    let mut program = compile_program(&input);

    let mut cpu = Cpu {
        reg_a: 12,
        ..Cpu::new()
    };

    while cpu.execute(&mut program) {}

    println!("{:?}", cpu);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
            cpy 2 a
            tgl a
            tgl a
            tgl a
            cpy 1 a
            dec a
            dec a";

        let mut program = compile_program(input);

        let mut cpu = Cpu::new();

        while cpu.execute(&mut program) {
            println!("{:?}", cpu);
        }

        assert_eq!(3, cpu.reg_a);
    }
}
