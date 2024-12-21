// 指令枚举
enum Instruction {
    Load(u32),
    Store,
    Add,
    Sub,
    Mul,
    Div,
    JumpIfZero(usize),
    JumpIfNotZero(usize),
    Jump(usize),
    Halt,
}

// 虚拟机结构体
struct VirtualMachine {
    memory: Vec<u32>,
    registers: [u32; 4],
    program_counter: usize,
}

impl VirtualMachine {
    fn new() -> Self {
        VirtualMachine {
            memory: Vec::new(),
            registers: [0; 4],
            program_counter: 0,
        }
    }

    fn load_instruction(&mut self) -> Option<Instruction> {
        if self.program_counter >= self.memory.len() {
            return None;
        }
        let opcode = self.memory[self.program_counter];
        self.program_counter += 1;
        match opcode {
            0 => Some(Instruction::Load(self.memory[self.program_counter]))
              .and_then(|_| { self.program_counter += 1; Some(Instruction::Load(self.memory[self.program_counter])) }),
            1 => Some(Instruction::Store),
            2 => Some(Instruction::Add),
            3 => Some(Instruction::Sub),
            4 => Some(Instruction::Mul),
            5 => Some(Instruction::Div),
            6 => Some(Instruction::JumpIfZero(self.memory[self.program_counter]))
              .and_then(|_| { self.program_counter += 1; Some(Instruction::JumpIfZero(self.memory[self.program_counter])) }),
            7 => Some(Instruction::JumpIfNotZero(self.memory[self.program_counter]))
              .and_then(|_| { self.program_counter += 1; Some(Instruction::JumpIfNotZero(self.memory[self.program_counter])) }),
            8 => Some(Instruction::Jump(self.memory[self.program_counter]))
              .and_then(|_| { self.program_counter += 1; Some(Instruction::Jump(self.memory[self.program_counter])) }),
            9 => Some(Instruction::Halt),
            _ => None,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Load(value) => {
                self.registers[0] = value;
            }
            Instruction::Store => {
                let address = self.registers[1] as usize;
                if address < self.memory.len() {
                    self.memory[address] = self.registers[0];
                } else {
                    println!("Invalid memory address for store!");
                }
            }
            Instruction::Add => {
                self.registers[0] = self.registers[0] + self.registers[1];
            }
            Instruction::Sub => {
                self.registers[0] = self.registers[0] - self.registers[1];
            }
            Instruction::Mul => {
                self.registers[0] = self.registers[0] * self.registers[1];
            }
            Instruction::Div => {
                if self.registers[1]!= 0 {
                    self.registers[0] = self.registers[0] / self.registers[1];
                } else {
                    println!("Division by zero!");
                }
            }
            Instruction::JumpIfZero(target) => {
                if self.registers[0] == 0 {
                    self.program_counter = target;
                }
            }
            Instruction::JumpIfNotZero(target) => {
                if self.registers[0]!= 0 {
                    self.program_counter = target;
                }
            }
            Instruction::Jump(target) => {
                self.program_counter = target;
            }
            Instruction::Halt => {
                println!("VM halted.");
            }
        }
    }

    fn run(&mut self) {
        while let Some(instruction) = self.load_instruction() {
            if let Instruction::Halt = instruction {
                break;
            }
            self.execute_instruction(instruction);
        }
    }
}

fn main() {
    let mut vm = VirtualMachine::new();
    // 一个简单的计算阶乘的程序示例
    vm.memory = vec![0, 5, 0, 1, 2, 8, 0, 0, 4, 0, 0, 1, 3, 8, 0, 1, 0, 2, 0, 9];
    vm.run();
}
