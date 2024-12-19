use regex::Regex;

const EXAMPLE: &str = r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

const INPUT: &str = r#"
Register A: 17323786
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0
"#;

enum Operand {
    Literal(i8),
    Combo(i8),
}

fn get_operand_value(operand: &Operand, registers: &Vec<i32>) -> i32 {
    match *operand {
        Operand::Literal(value) => value as i32,
        Operand::Combo(value) => match value {
            0..=3 => value as i32,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => panic!("Should not happen"),
        }
    }
}

#[repr(i8)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}


impl Opcode {
    fn from_i8(num: i8) -> Option<Self> {
        match num {
            0 => Some(Opcode::Adv),
            1 => Some(Opcode::Bxl),
            2 => Some(Opcode::Bst),
            3 => Some(Opcode::Jnz),
            4 => Some(Opcode::Bxc),
            5 => Some(Opcode::Out),
            6 => Some(Opcode::Bdv),
            7 => Some(Opcode::Cdv),
            _ => None, // Return None if the number doesn't correspond to any variant
        }
    }
}

struct Program {
    code_and_data: Vec<i8>,
    registers: Vec<i32>,
    output: Vec<i32>,
    sp: usize,  //stack pointer
}

impl Program {
    fn read_from_string(input: &str) -> Program{
        let re_a = Regex::new(r"Register A: (\d+)").unwrap();
        let re_b = Regex::new(r"Register B: (\d+)").unwrap();
        let re_c = Regex::new(r"Register C: (\d+)").unwrap();
        println!("{}", input);
        println!("{:?}", re_a.captures(input));
        let a = re_a.captures(input).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = re_b.captures(input).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let c = re_c.captures(input).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let re_program = Regex::new(r"Program: (.*)").unwrap();
        let p = re_program.captures(input).unwrap().get(1).unwrap();
        let p = p.as_str().split(',').map(|c| {
            c.parse::<i8>().unwrap()
        }).collect::<Vec<i8>>();
        Program::new(p, vec![a, b, c])
    }

    fn print(&self) {
        println!("Register A: {},\tRegister B: {}\tRegister C: {}", self.registers[0], self.registers[1], self.registers[2]);
        println!("sp: {}", self.sp);
        println!("code and data: {:?}", self.code_and_data);
        println!("output: {:?}", self.output);
        println!("-------------------------------------------------\n");
    }

    fn new(code_and_data: Vec<i8>, registers: Vec<i32>) -> Program {
        return Program{code_and_data: code_and_data, output: Vec::new(), registers: registers, sp: 0};
    }

    fn step(&mut self) {
        let opcode: Opcode = Opcode::from_i8(self.code_and_data[self.sp]).unwrap();
        let operand = self.code_and_data[self.sp + 1];
        self.sp += 2;
        match opcode {
            Opcode::Adv => {
                let denom = get_operand_value(&Operand::Combo(operand), &self.registers);
                let num = self.registers[0];
                self.registers[0] = num >> denom;
            },
            Opcode::Bxl => {
                let element = get_operand_value(&Operand::Literal(operand), &self.registers);
                self.registers[1] = self.registers[1] ^ element;
            },
            Opcode::Bst => {
                self.registers[1] = get_operand_value(&Operand::Combo(operand), &self.registers) & 7;
            },
            Opcode::Jnz => {
                match self.registers[0] {
                    0 => {},
                    _ => {self.sp = get_operand_value(&Operand::Literal(operand), &self.registers) as usize},
                }
            },
            Opcode::Bxc => {
                self.registers[1] = self.registers[1] ^ self.registers[2];
            },
            Opcode::Out => {
                let a = get_operand_value(&Operand::Combo(operand), &self.registers);
                // println!("operand: {}, a: {}, mod: {}", operand, a, a & 7);
                self.output.push(get_operand_value(&Operand::Combo(operand), &self.registers) & 7);
            },
            Opcode::Bdv => {
                let denom = get_operand_value(&Operand::Combo(operand), &self.registers);
                let num = self.registers[0];
                self.registers[1] = num >> denom;
            },
            Opcode::Cdv => {
                let denom = get_operand_value(&Operand::Combo(operand), &self.registers);
                let num = self.registers[0];
                self.registers[2] = num >> denom;
            },
        }
    }

    fn run(&mut self, debug: bool) -> String {
        self.print();
        let mut i: i32 = 0;
        while self.sp < self.code_and_data.len() {
            self.step();
            if debug {
                self.print();
                i += 1;
                if i >= 20 { panic!(); } // just for debugging. Ignore.
            }
        };
        self.print();
        self.output.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",")
    }

    fn min_self_repeating_a(&mut self) -> i32 {
        let program_str = self.code_and_data.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",");

        (0..).into_iter().take_while(|a| {
            self.registers[0] = *a as i32;
            let o = self.run(false);
            o != program_str
        }).last().unwrap()
        
    }
}

fn pg() {
    let mut p = Program::read_from_string(INPUT);
    println!("{}", p.run(true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut p = Program::read_from_string(INPUT);
        println!("{}", p.min_self_repeating_a());
        // pg();
        // assert_eq!(4, 4);
    }
}
