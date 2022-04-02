use crate::instructions::*;
use std::vec;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    // heap: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            // heap: vec![],
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        };

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize; // We cast to usize so we can use it as an index into the array
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32; // Our registers are i32s, so we need to cast it. We'll cover that later.
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                // return true;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= value;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.registers[self.next_8_bits() as usize] = 1;
                } else {
                    self.registers[self.next_8_bits() as usize] = 0;
                }
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 != register2 {
                    self.registers[self.next_8_bits() as usize] = 1;
                } else {
                    self.registers[self.next_8_bits() as usize] = 0;
                }
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 > register2 {
                    self.registers[self.next_8_bits() as usize] = 1;
                } else {
                    self.registers[self.next_8_bits() as usize] = 0;
                }
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 >= register2 {
                    self.registers[self.next_8_bits() as usize] = 1;
                } else {
                    self.registers[self.next_8_bits() as usize] = 0;
                }
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 < register2 {
                    self.registers[self.next_8_bits() as usize] = 1;
                } else {
                    self.registers[self.next_8_bits() as usize] = 0;
                }
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 <= register2 {
                    self.registers[self.next_8_bits() as usize] = 1;
                } else {
                    self.registers[self.next_8_bits() as usize] = 0;
                }
            }
            Opcode::JEQ => {
                let target_register = self.registers[self.next_8_bits() as usize];
                let eq_register = self.registers[self.next_8_bits() as usize];
                match eq_register {
                    0 => {}
                    1 => self.pc = target_register as usize,
                    _ => println!("Non-Valid register"),
                }
            }
            Opcode::IGL => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            }
        }

        false
    }

    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        // test_vm.float_registers[0] = 5.0;
        // test_vm.float_registers[1] = 10.0;
        test_vm
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_length() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0, 5, 0, 1, 0, 10, 1, 0, 1, 2]; // Ja Sem, dit werkt wees niet bang :)
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 13)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![2, 1, 0, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![4, 1, 0, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 2, 9, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.program = vec![10, 0, 1, 2, 10, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_gte_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 2, 11, 0, 1, 2, 11, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_lte_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 2, 12, 0, 1, 2, 12, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 2, 13, 0, 1, 2, 13, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 2, 14, 0, 1, 2, 14, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.registers[1] = 1;
        test_vm.program = vec![15, 0, 1, 0, 15, 0, 0, 0, 15, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}
