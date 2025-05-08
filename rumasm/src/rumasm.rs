
use std::io::{self, Write};
type Umword = u32;

static RA: u32 = 6;
static RB: u32 = 3;
// static RL: u32 = 25;
static OP: u32 = 28;

pub fn asm(inst: Umword) {
    let bytes = inst.to_be_bytes();
    io::stdout().write_all(&bytes).unwrap();
}

pub fn regs(ra: Umword, rb: Umword, rc: Umword) -> Umword {
    ra << RA | rb << RB | rc
}

pub fn cmov(ra: Umword, rb: Umword, rc: Umword)-> Umword {
     regs(ra, rb, rc) 
} 

pub fn load(ra: Umword, rb: Umword, rc: Umword)-> Umword { 
    1_u32 << OP | regs(ra, rb, rc) 
} 

pub fn store(ra: Umword, rb: Umword, rc: Umword)-> Umword { 
    2_u32 << OP | regs(ra, rb, rc) 
}

pub fn add(ra: Umword, rb: Umword, rc: Umword)-> Umword { 
    3_u32 << OP | regs(ra, rb, rc) 
}

pub fn mul(ra: Umword, rb: Umword, rc: Umword)-> Umword { 
    4_u32 << OP | regs(ra, rb, rc) 
}

pub fn div(ra: Umword, rb: Umword, rc: Umword)-> Umword { 
    5_u32 << OP | regs(ra, rb, rc) 
}

pub fn nand(ra: Umword, rb: Umword, rc: Umword)-> Umword { 
    6_u32 << OP | regs(ra, rb, rc) 
}

pub fn halt()-> Umword {
    7_u32 << OP
}

pub fn mapseg(rb: Umword, rc: Umword)-> Umword { 
    8_u32 << OP | regs(0, rb, rc) 
}

pub fn unmapseg(rc: Umword)-> Umword { 
    9_u32 << OP | regs(0, 0, rc) 
}

pub fn output(rc: Umword)-> Umword { 
    10_u32 << OP | regs(0, 0, rc) 
}

pub fn input(rc: Umword)-> Umword { 
    11_u32 << OP | regs(0, 0, rc) 
}

pub fn loadprog(rb: Umword, rc: Umword)-> Umword { 
    12_u32 << OP | regs(0, rb, rc) 
}

pub fn loadv(ra: Umword, value: Umword)-> Umword { 
    13_u32 << OP | ra << RA | (value & 0x1FFFFFF)
}