pub mod instruction;

use instruction::Instruction;

#[derive(Debug)]
pub struct Program {
    registers: [i32; 4],
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        let registers = [0; 4];
        Self {
            registers,
            instructions,
        }
    }

    pub fn from_str(input: &str) -> Self {
        Self::new(Instruction::parse_str(input))
    }

    pub fn run(mut self) {
        let mut address: usize = 0;
        while let Some(instruction) = self.instructions.get(address) {
            if instruction.run(&mut self.registers, &mut address) {
                address += 1;
            }
        }

        println!("{:?}", self.registers);
    }
}
