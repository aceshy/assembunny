type RegisterIndex = usize;

#[derive(Debug)]
enum ArgumentType {
    Number(i32),
    Register(RegisterIndex),
}

#[derive(Debug)]
pub struct Copy {
    source: ArgumentType,
    destination: RegisterIndex,
}

#[derive(Debug)]
pub struct Increment {
    target: RegisterIndex,
}

#[derive(Debug)]
pub struct Decrement {
    target: RegisterIndex,
}

#[derive(Debug)]
pub struct JumpNotZero {
    target: ArgumentType,
    jump_distance: ArgumentType,
}

impl Copy {
    fn run(&self, registers: &mut [i32; 4]) {
        registers[self.destination] = match self.source {
            ArgumentType::Number(value) => value,
            ArgumentType::Register(register_index) => registers[register_index],
        };
    }

    fn parse(source: &str, destination: &str) -> Self {
        Copy {
            source: Instruction::parse_argument_type(source),
            destination: match Instruction::parse_argument_type(destination) {
                ArgumentType::Number(_) => {
                    panic!("Copy requires a destination register but was supplied a number instead")
                }
                ArgumentType::Register(register_index) => register_index,
            },
        }
    }
}

impl Increment {
    fn run(&self, registers: &mut [i32; 4]) {
        registers[self.target] += 1;
    }

    fn parse(target: &str) -> Self {
        Increment {
            target: match Instruction::parse_argument_type(target) {
                ArgumentType::Number(_) => {
                    panic!("Increment requires a register but was supplied a number instead")
                }
                ArgumentType::Register(register_index) => register_index,
            },
        }
    }
}

impl Decrement {
    fn run(&self, registers: &mut [i32; 4]) {
        registers[self.target] -= 1;
    }

    fn parse(target: &str) -> Self {
        Decrement {
            target: match Instruction::parse_argument_type(target) {
                ArgumentType::Number(_) => {
                    panic!("Decrement requires a register but was supplied a number instead")
                }
                ArgumentType::Register(register_index) => register_index,
            },
        }
    }
}

impl JumpNotZero {
    fn run(&self, registers: &[i32; 4], address: &mut usize) {
        if match self.target {
            ArgumentType::Number(value) => value != 0,
            ArgumentType::Register(register_index) => registers[register_index] != 0,
        } {
            let jump_distance = match self.jump_distance {
                ArgumentType::Number(value) => value,
                ArgumentType::Register(register_index) => registers[register_index],
            };

            if jump_distance >= 0 {
                *address += jump_distance as usize;
            } else {
                *address -= jump_distance.unsigned_abs() as usize;
            }
            *address -= 1;
        };
    }

    fn parse(target: &str, jump_distance: &str) -> Self {
        JumpNotZero {
            target: Instruction::parse_argument_type(target),
            jump_distance: Instruction::parse_argument_type(jump_distance),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Copy(Copy),
    Increment(Increment),
    Decrement(Decrement),
    JumpNotZero(JumpNotZero),
}

impl Instruction {
    pub fn run(&self, registers: &mut [i32; 4], address: &mut usize) {
        match self {
            Instruction::Copy(instruction) => &instruction.run(registers),
            Instruction::Increment(instruction) => &instruction.run(registers),
            Instruction::Decrement(instruction) => &instruction.run(registers),
            Instruction::JumpNotZero(instruction) => &instruction.run(registers, address),
        };
    }

    fn parse_argument_type(argument: &str) -> ArgumentType {
        match argument {
            "a" | "b" | "c" | "d" => {
                ArgumentType::Register(argument.chars().next().unwrap() as usize - 97)
            }
            _ => ArgumentType::Number(
                argument
                    .parse::<i32>()
                    .expect(&format!("{} is not a valid input value", argument)),
            ),
        }
    }

    pub fn parse(input: &str) -> Instruction {
        let input = input.trim().split(' ').collect::<Vec<&str>>();
        match input[0] {
            "cpy" => Instruction::Copy(Copy::parse(
                input.get(1).expect("Copy requires a source to copy"),
                input.get(2).expect("Copy requires a destiantion register"),
            )),
            "inc" => Instruction::Increment(Increment::parse(
                input
                    .get(1)
                    .expect("Increment requires a register to increment"),
            )),
            "dec" => Instruction::Decrement(Decrement::parse(
                input
                    .get(1)
                    .expect("Decrement requires a register to decrement"),
            )),
            "jnz" => Instruction::JumpNotZero(JumpNotZero::parse(
                input
                    .get(1)
                    .expect("Jump Not Zero requires a target to compare"),
                input
                    .get(2)
                    .expect("Jump Not Zero requires a jump distance"),
            )),
            _ => todo!("Unknown instruction: {}", input[0]),
        }
    }

    pub fn parse_str(input: &str) -> Vec<Instruction> {
        input
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(Self::parse)
            .collect()
    }
}
