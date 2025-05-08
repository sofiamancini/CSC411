
use rumasm::rumasm::asm; 
use rumasm::rumasm::loadv; 
use rumasm::rumasm::output; 
use rumasm::rumasm::halt; 

pub fn main() { 
    asm(loadv(0, 65)); // A into register 0 
    asm(output(0)); // output register 0 
    asm(loadv(0, 10)); // linefeed into register 0 
    asm(output(0)); // output register 0 
    asm(halt()); 
}