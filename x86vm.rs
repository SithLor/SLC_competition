//shity code vm 1.0

struct VM {
    registers: [f64; 32],
    stack: Vec<f64>,
    debug: bool,
    program_couter: i32
}
enum OpCode {
    ADD(f64, f64,i32),//add two values from arg 1 and arg 2 and store in arg 3
    SUB(f64, f64,i32),
    MUL(f64, f64,i32),
    DIV(f64, f64,i32),
    XOR(f64, f64,i32),

    HALT,
    MOV(f64, i32),

    JMP(i32),
    JMPZ(i32)

}


impl VM {
    fn new(_debug: bool) -> VM {
        VM {
            registers: [0.0; 32],
            stack: Vec::new(),
            debug: _debug,
            program_couter: 0
        }
    }


    fn exec(&mut self, code: Vec<OpCode>){
        let mut program_couter: i32 = self.program_couter;
        //have vm support jmp istruction able to jump to any instruction in the code array
        while program_couter < code.len() as i32 {
            match code[program_couter as usize] {
                OpCode::ADD(arg1, arg2, arg3) => {
                    self.registers[arg3 as usize] = arg1 + arg2;
                },
                OpCode::SUB(arg1, arg2, arg3) => {
                    self.registers[arg3 as usize] = arg1 - arg2;
                },
                OpCode::MUL(arg1, arg2, arg3) => {
                    self.registers[arg3 as usize] = arg1 * arg2;
                },
                OpCode::DIV(arg1, arg2, arg3) => {
                    self.registers[arg3 as usize] = arg1 / arg2;
                },
                OpCode::XOR(arg1, arg2, arg3) => {
                    self.registers[arg3 as usize] = arg1 * arg2;
                },
                OpCode::MOV(value, register) => {
                    self.registers[register as usize] = value;
                },
                OpCode::JMP(index) => {
                    program_couter = index;
                    continue;
                },
                OpCode::JMPZ(index) => {
                    if self.registers[0] == 0.0 {
                        program_couter = index;
                        continue;
                    }
                },
                OpCode::HALT => {
                    break;
                }
            }
            program_couter += 1;
        }


    }
}
//registers 0-31 are f64 values

//
fn code_paser(code_string:String) -> Vec<OpCode> {
    let mut code: Vec<OpCode> = Vec::new();
    for line in code_string.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[0] {
            "MOV" => {
                let value = parts[1].parse::<f64>().unwrap();
                let register = parts[2].parse::<i32>().unwrap();
                code.push(OpCode::MOV(value, register));
            },
            "ADD" => {
                let arg1 = parts[1].parse::<f64>().unwrap();
                let arg2 = parts[2].parse::<f64>().unwrap();
                let arg3 = parts[3].parse::<i32>().unwrap();
                code.push(OpCode::ADD(arg1, arg2, arg3));
            },
            "SUB" => {
                let arg1 = parts[1].parse::<f64>().unwrap();
                let arg2 = parts[2].parse::<f64>().unwrap();
                let arg3 = parts[3].parse::<i32>().unwrap();
                code.push(OpCode::SUB(arg1, arg2, arg3));
            },
            "JMP" => {
                let index = parts[1].parse::<i32>().unwrap();
                code.push(OpCode::JMP(index));
            },
            "HALT" => {
                code.push(OpCode::HALT);
            },
            "JMPZ" => {
                let index = parts[1].parse::<i32>().unwrap();
                code.push(OpCode::JMPZ(index));
            },
            _ => {
                panic!("Invalid instruction");
            }
        }
    }
    code
}

//fn main(){
//    let mut vm = VM::new(true);
//    //let code = code_paser(TEST_CODE.to_string());
//
//    //
//    println!("Hello, world!");
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_code_parser() {
    let code_string: String = "MOV 10.0 0\nADD 1.0 2.0 1\nPRINT \"Hello, World!\"\nHALT".to_string();
    let code = code_paser(code_string);
    assert_eq!(code.len(), 4);
    match code[0] {
        OpCode::MOV(value, register) => {
            assert_eq!(value, 10.0);
            assert_eq!(register, 0);
        },
        _ => panic!("Expected MOV instruction"),
    }
    match code[1] {
        OpCode::ADD(arg1, arg2, arg3) => {
            assert_eq!(arg1, 1.0);
            assert_eq!(arg2, 2.0);
            assert_eq!(arg3, 1);
        },
        _ => panic!("Expected ADD instruction"),
    }
    match code[2] {
        OpCode::PRINT(s) => {
            assert_eq!(s, "Hello, World!");
        },
        _ => panic!("Expected PRINT instruction"),
    }
    match code[3] {
        OpCode::HALT => {},
        _ => panic!("Expected HALT instruction"),
    }
}

    #[test]
    fn test_add() {
        let mut vm = VM::new(false);
        vm.exec(vec![
            OpCode::ADD(1.0, 2.0, 0), // 1.0 + 2.0 = 3.0
            OpCode::HALT
        ]);
        println!("{:?}", vm.registers[0]);
        assert_eq!(vm.registers[0], 3.0);

    }

    #[test]
    fn test_mov() {
        let mut vm = VM::new(false);
        vm.exec(vec![
            OpCode::MOV(10.0, 1), // Move 10.0 into register 1
            OpCode::HALT
        ]);
        println!("{:?}", vm.registers[1]);
        assert_eq!(vm.registers[1], 10.0);
    }

    #[test]
    fn test_jmp() {
        let mut vm = VM::new(false);
        vm.exec(vec![
            OpCode::MOV(10.0, 0), // Move 10.0 into register 0
            OpCode::JMP(3), // Jump to instruction at index 3
            OpCode::MOV(20.0, 0), // This instruction will be skipped
            OpCode::HALT
        ]);
        println!("{:?}", vm.registers[1]);
        assert_eq!(vm.registers[0], 10.0);
    }

    #[test]
    fn test_jmpz() {
        let mut vm = VM::new(false);
        vm.exec(vec![
            OpCode::MOV(0.0, 0), // Move 0.0 into register 0
            OpCode::JMPZ(3), // Jump to instruction at index 3 because register 0 is zero
            OpCode::MOV(20.0, 0), // This instruction will be skipped
            OpCode::HALT
        ]);
        assert_eq!(vm.registers[0], 0.0);
    }
}