use std::fmt;

use crate::state::ExecutionState;

enum Code {
    Primitive(Box<dyn Fn(&mut ExecutionState)>),
}

pub struct Word {
    pub name: String,
    code: Code,
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word({})", self.name)
    }
}

impl Word {
    pub fn execute(&self, state: &mut ExecutionState) {
        match &self.code {
            Code::Primitive(func) => func(state),
        }
    }

    pub fn init_dictionary() -> Vec<Word> {
        let mut dictionary = Vec::new();
        dictionary.push(Word{
            name: "+".to_string(),
            code: Code::Primitive(Box::new(|state| { 
                let vals = state.pop_two();
                state.data_stack.push(vals.1 + vals.0);
            }))
        });
        dictionary.push(Word{
            name: "-".to_string(),
            code: Code::Primitive(Box::new(|state| { 
                let vals = state.pop_two();
                state.data_stack.push(vals.1 - vals.0);
            }))
        });
        dictionary.push(Word{
            name: "*".to_string(),
            code: Code::Primitive(Box::new(|state| { 
                let vals = state.pop_two();
                state.data_stack.push(vals.1 * vals.0);
            }))
        });
        dictionary.push(Word{
            name: "/".to_string(),
            code: Code::Primitive(Box::new(|state| { 
                let vals = state.pop_two();
                state.data_stack.push(vals.1 / vals.0);
            }))
        });
        dictionary.push(Word{
            name: "%".to_string(),
            code: Code::Primitive(Box::new(|state| { 
                let vals = state.pop_two();
                state.data_stack.push(vals.1 % vals.0);
            }))
        });
        dictionary.push(Word{
            name: "drop".to_string(),
            code: Code::Primitive(Box::new(|state| { state.pop_one(); }))
        });
        dictionary.push(Word{
            name: "nip".to_string(),
            code: Code::Primitive(Box::new(|state| {
                let vals = state.pop_two();
                state.data_stack.push(vals.0);
            }))
        });
        dictionary.push(Word{
            name: "dup".to_string(),
            code: Code::Primitive(Box::new(|state| {
                let val = state.data_stack.last().unwrap().clone();
                state.data_stack.push(val);
            }))
        });
        dictionary.push(Word{
            name: ".".to_string(),
            code: Code::Primitive(Box::new(|state| {
                println!("{}", state.pop_one());
            }))
        });
        dictionary.push(Word{
            name: ".s".to_string(),
            code: Code::Primitive(Box::new(|state| {
                println!("{:?}", state.data_stack);
            }))
        });
        dictionary
    }
}
