use std::fs::File;
use std::io::{self, Read};
use crate::vm::Word;

pub fn load_program(path: &str) -> io::Result<Vec<Word>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut program = Vec::new();
    for chunk in buffer.chunks(4) {
        let word = ((chunk[0] as Word) << 24)
            | ((chunk[1] as Word) << 16)
            | ((chunk[2] as Word) << 8)
            | (chunk[3] as Word);
        program.push(word);
    }
    Ok(program)
}
