use crate::words::Word;

pub trait Interpreter {
    fn interpret(&mut self, dictionary: &mut Vec<Word>, code: &str);
}

pub struct ExecutionState {
    pub data_stack: Vec<i64>,
}

impl ExecutionState {
    pub fn new() -> ExecutionState {
        ExecutionState {
	        data_stack: Vec::new(),
        }
    }

    pub fn pop_one(&mut self) -> i64 {
        self.data_stack.pop().unwrap()
    }

    pub fn pop_two(&mut self) -> (i64, i64) {
        (self.data_stack.pop().unwrap(), self.data_stack.pop().unwrap())
    }

    pub fn pop_three(&mut self) -> (i64, i64, i64) {
        (
            self.data_stack.pop().unwrap(),
            self.data_stack.pop().unwrap(),
            self.data_stack.pop().unwrap(),
        )
    }
}

impl Interpreter for ExecutionState {
    fn interpret(&mut self, dictionary: &mut Vec<Word>, code: &str) {
        for bit in code.split_whitespace() {
            match bit.parse::<i64>() {
                Ok(n) => self.data_stack.push(n),
                Err(_) => {
                    for word in dictionary.iter() {
                        if word.name == bit {
                            word.execute(self);
                        }
                    }
                }
            }
        }
    }
}
