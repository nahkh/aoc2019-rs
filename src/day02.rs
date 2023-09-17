#[derive(Debug, PartialEq, Clone)]
struct Memory {
    current_op: usize,
    memory: Vec<usize>,
    running: bool,
    panicked: bool,
}

trait Runnable {
    fn execute_step(&mut self);

    fn execute_until_stopped(&mut self);
}

impl Runnable for Memory {
    fn execute_step(&mut self) {
        if !self.running{
            return;
        }

        match self.memory[self.current_op] {
            1 => {
                let index1 = self.memory[self.current_op + 1];
                let index2 = self.memory[self.current_op + 2];
                let target_index = self.memory[self.current_op + 3];
                let value1 = self.memory[index1];
                let value2 = self.memory[index2];
                self.memory[target_index] = value1 + value2;
                self.current_op += 4;
            },
            2 => {
                let index1 = self.memory[self.current_op + 1];
                let index2 = self.memory[self.current_op + 2];
                let target_index = self.memory[self.current_op + 3];
                let value1 = self.memory[index1];
                let value2 = self.memory[index2];
                self.memory[target_index] = value1 * value2;
                self.current_op += 4;
            },
            99 => {
                self.running = false;
            },
            _ => {
                self.running = false;
                self.panicked = true;
            }
        }
    }

    fn execute_until_stopped(&mut self) {
        while self.running {
            self.execute_step();
        }
    }
}

fn read_program(content: String) -> Memory {
    let c = content.matches(",").count() + 1;
    let mut m = Memory {
        current_op: 0,
        memory: vec![0; c],
        running: true,
        panicked: false,
    };
    let mut i = 0;
    for line in content.split(",") {
        let value = line.trim().parse::<usize>().unwrap();
        m.memory[i] = value;
        i += 1;
    }
    return m;
}


pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day02.txt"));
    let c = content.matches(",").count() + 1;
    for i in 1..c {
        for j in 1..c {
            let mut m = read_program(content.clone());
            m.memory[1] = i;
            m.memory[2] = j;
            m.execute_until_stopped();
            if m.memory[0] == 19690720 {
                println!("Noun {}, verb {}, output: {}", i, j, 100 * i + j);
                return;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::day02::Runnable;

    #[test]
    fn test_simple_program() {
        let mut m = crate::day02::read_program(String::from("1,9,10,3,2,3,11,0,99,30,40,50"));
        m.execute_until_stopped();
        let mut expected = crate::day02::read_program(String::from("3500,9,10,70,2,3,11,0,99,30,40,50"));
        expected.current_op = 8;
        expected.running = false;
        assert_eq!(m, expected);
    }
}
