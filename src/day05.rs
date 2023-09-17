#[derive(Debug, PartialEq, Clone)]
struct Memory {
    current_op: usize,
    memory: Vec<i32>,
    running: bool,
    panicked: bool,
    current_input: usize,
    input: Vec<i32>,
    output: Vec<i32>,
}

enum Parameter {
    Position(i32),
    Immediate(i32),
}

enum Op {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

fn make_parameter(mode: i32, value: i32) -> Parameter {
    match mode {
        0 => Parameter::Position(value),
        1 => Parameter::Immediate(value),
        _ => panic!("Unsupported mode {}", mode),
    }
}

trait Runnable {
    fn interpret_op(&self) -> Op;

    fn read_parameter(&self, parameter: &Parameter) -> i32;

    fn write_parameter(&mut self, parameter: &Parameter, value: i32);

    fn execute_step(&mut self);

    fn execute_until_stopped(&mut self);
}

impl Runnable for Memory {
    fn interpret_op(&self) -> Op {
        let instruction = self.memory[self.current_op];
        let opcode = instruction % 100;
        let mode1 = (instruction / 100) % 10;
        let mode2 = (instruction / 1000) % 10;
        let mode3 = (instruction / 10000) % 10;
        
        match opcode {
            1 => Op::Add(
                make_parameter(mode1, self.memory[self.current_op + 1]),
                make_parameter(mode2, self.memory[self.current_op + 2]),
                make_parameter(mode3, self.memory[self.current_op + 3])),
            2 => Op::Multiply(
                make_parameter(mode1, self.memory[self.current_op + 1]),
                make_parameter(mode2, self.memory[self.current_op + 2]),
                make_parameter(mode3, self.memory[self.current_op + 3])),
            3 => Op::Input(
                make_parameter(mode1, self.memory[self.current_op + 1])),
            4 => Op::Output(
                make_parameter(mode1, self.memory[self.current_op + 1])),
            5 => Op::JumpIfTrue(
                make_parameter(mode1, self.memory[self.current_op + 1]),
                make_parameter(mode2, self.memory[self.current_op + 2])),
            6 => Op::JumpIfFalse(
                make_parameter(mode1, self.memory[self.current_op + 1]),
                make_parameter(mode2, self.memory[self.current_op + 2])),
            7 => Op::LessThan(
                make_parameter(mode1, self.memory[self.current_op + 1]),
                make_parameter(mode2, self.memory[self.current_op + 2]),
                make_parameter(mode3, self.memory[self.current_op + 3])),
            8 => Op::Equals(
                make_parameter(mode1, self.memory[self.current_op + 1]),
                make_parameter(mode2, self.memory[self.current_op + 2]),
                make_parameter(mode3, self.memory[self.current_op + 3])),
            99 => Op::Halt,
            _ => panic!("Invalid opcode {}", opcode)
        }
    }

    fn read_parameter(&self, parameter: &Parameter) -> i32 {
        match parameter {
            Parameter::Immediate(x) => *x,
            Parameter::Position(x) => self.memory[*x as usize],
        }
    }

    fn write_parameter(&mut self, parameter: &Parameter, value: i32) {
        match parameter {
            Parameter::Immediate(_) => panic!("Cannot write to an immediate parameter!"),
            Parameter::Position(x) => self.memory[*x as usize] = value,
        };
    }


    fn execute_step(&mut self) {
        if !self.running{
            return;
        }
        match self.interpret_op() {
            Op::Add(l, r, o) => {
                self.write_parameter(&o, self.read_parameter(&l) + self.read_parameter(&r));
                self.current_op += 4;
            },
            Op::Multiply(l, r, o) => {
                self.write_parameter(&o, self.read_parameter(&l) * self.read_parameter(&r));
                self.current_op += 4;
            },
            Op::Input(o) => {
                self.write_parameter(&o, self.input[self.current_input]);
                self.current_op += 2;
                self.current_input += 1;
            },
            Op::Output(o) => {
                self.output.push(self.read_parameter(&o));
                self.current_op += 2;
            },
            Op::JumpIfTrue(t, new_op) => {
                if self.read_parameter(&t) != 0 {
                    self.current_op = self.read_parameter(&new_op) as usize;
                } else {
                    self.current_op += 3;
                }
            },
            Op::JumpIfFalse(t, new_op) => {
                if self.read_parameter(&t) == 0 {
                    self.current_op = self.read_parameter(&new_op) as usize;
                } else {
                    self.current_op += 3;
                }
            },
            Op::LessThan(l, r, o) => {
                if self.read_parameter(&l) < self.read_parameter(&r) {
                    self.write_parameter(&o, 1);
                } else {
                    self.write_parameter(&o, 0);
                }
                self.current_op += 4;
            },
            Op::Equals(l, r, o) => {
                if self.read_parameter(&l) == self.read_parameter(&r) {
                    self.write_parameter(&o, 1);
                } else {
                    self.write_parameter(&o, 0);
                }
                self.current_op += 4;
            },
            Op::Halt => {
                self.running = false;
            },
        };
    }

    fn execute_until_stopped(&mut self) {
        while self.running {
            self.execute_step();
        }
    }
}

fn read_program(content: String, input: i32) -> Memory {
    let c = content.matches(",").count() + 1;
    let mut m = Memory {
        current_op: 0,
        memory: vec![0; c],
        running: true,
        panicked: false,
        current_input: 0,
        input: Vec::from([input]),
        output: Vec::new(),
    };
    let mut i = 0;
    for line in content.split(",") {
        let value = line.trim().parse::<i32>().unwrap();
        m.memory[i] = value;
        i += 1;
    }
    return m;
}


pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day05.txt"));
    let mut m1 = read_program(content.clone(), 1);
    m1.execute_until_stopped();
    println!("Part 1: Diagnostic output: {}", m1.output[m1.output.len() - 1]);
    let mut m2 = read_program(content.clone(), 5);
    m2.execute_until_stopped();
    println!("Part 2: Diagnostic output: {}", m2.output[m2.output.len() - 1]);
}


#[cfg(test)]
mod tests {
    use crate::day05::Runnable;

    #[test]
    fn test_simple_program() {
        let mut m = crate::day05::read_program(String::from("3,0,4,0,99"), 77);
        m.execute_until_stopped();
        assert_eq!(m.output.len(), 1);
        assert_eq!(m.output[0], 77);
    }

    #[test]
    fn test_eq_positional() {
        let mut m1 = crate::day05::read_program(String::from("3,9,8,9,10,9,4,9,99,-1,8"), 8);
        m1.execute_until_stopped();
        assert_eq!(m1.output.len(), 1);
        assert_eq!(m1.output[0], 1);
        let mut m2 = crate::day05::read_program(String::from("3,9,8,9,10,9,4,9,99,-1,8"), 9);
        m2.execute_until_stopped();
        assert_eq!(m2.output.len(), 1);
        assert_eq!(m2.output[0], 0);
    }

    #[test]
    fn test_lt_positional() {
        let mut m1 = crate::day05::read_program(String::from("3,9,7,9,10,9,4,9,99,-1,8"), 7);
        m1.execute_until_stopped();
        assert_eq!(m1.output.len(), 1);
        assert_eq!(m1.output[0], 1);
        let mut m2 = crate::day05::read_program(String::from("3,9,7,9,10,9,4,9,99,-1,8"), 9);
        m2.execute_until_stopped();
        assert_eq!(m2.output.len(), 1);
        assert_eq!(m2.output[0], 0);
    }

    #[test]
    fn test_eq_immediate() {
        let mut m1 = crate::day05::read_program(String::from("3,3,1108,-1,8,3,4,3,99"), 8);
        m1.execute_until_stopped();
        assert_eq!(m1.output.len(), 1);
        assert_eq!(m1.output[0], 1);
        let mut m2 = crate::day05::read_program(String::from("3,3,1108,-1,8,3,4,3,99"), 9);
        m2.execute_until_stopped();
        assert_eq!(m2.output.len(), 1);
        assert_eq!(m2.output[0], 0);
    }

    #[test]
    fn test_lt_immediate() {
        let mut m1 = crate::day05::read_program(String::from("3,3,1107,-1,8,3,4,3,99"), 7);
        m1.execute_until_stopped();
        assert_eq!(m1.output.len(), 1);
        assert_eq!(m1.output[0], 1);
        let mut m2 = crate::day05::read_program(String::from("3,3,1107,-1,8,3,4,3,99"), 9);
        m2.execute_until_stopped();
        assert_eq!(m2.output.len(), 1);
        assert_eq!(m2.output[0], 0);
    }

    #[test]
    fn test_jump_positional() {
        let mut m1 = crate::day05::read_program(String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), 7);
        m1.execute_until_stopped();
        assert_eq!(m1.output.len(), 1);
        assert_eq!(m1.output[0], 1);
        let mut m2 = crate::day05::read_program(String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), 0);
        m2.execute_until_stopped();
        assert_eq!(m2.output.len(), 1);
        assert_eq!(m2.output[0], 0);
    }

    #[test]
    fn test_jump_immediate() {
        let mut m1 = crate::day05::read_program(String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), 7);
        m1.execute_until_stopped();
        assert_eq!(m1.output.len(), 1);
        assert_eq!(m1.output[0], 1);
        let mut m2 = crate::day05::read_program(String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), 0);
        m2.execute_until_stopped();
        assert_eq!(m2.output.len(), 1);
        assert_eq!(m2.output[0], 0);
    }

    #[test]
    fn test_large() {
        let mut m1 = crate::day05::read_program(String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), 7);
        m1.execute_until_stopped();
        assert_eq!(m1.output.len(), 1);
        assert_eq!(m1.output[0], 999);
        let mut m2 = crate::day05::read_program(String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), 8);
        m2.execute_until_stopped();
        assert_eq!(m2.output.len(), 1);
        assert_eq!(m2.output[0], 1000);
        let mut m3 = crate::day05::read_program(String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), 9);
        m3.execute_until_stopped();
        assert_eq!(m3.output.len(), 1);
        assert_eq!(m3.output[0], 1001);
    }
}
