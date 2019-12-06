extern crate rustyline;

mod state;
mod words;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::state::ExecutionState;
use crate::state::Interpreter;
use crate::words::Word;

fn main() {
    let mut rl = Editor::<()>::new();

    let mut state = ExecutionState::new();
    let mut dictionary = Word::init_dictionary();

    loop {
        let readline = rl.readline("FeO₂> ");
        match readline {
            Ok(line) => {
                state.interpret(&mut dictionary, &line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ruuuuuude");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Wut?");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            },
        }
    }
}
