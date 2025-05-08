use std::io::{self, Read, Write};

pub type Word = u32;
type SegmentID = u32;

pub struct VirtualMachine {
    registers: [Word; 8],
    segments: Vec<Option<Vec<Word>>>,
    free_ids: Vec<usize>,
    pc: usize,
    next_id: usize,
    segment_pool: Vec<Vec<Word>>,
}

impl VirtualMachine {
    pub fn new(program: Vec<Word>) -> Self {
        let mut segments = Vec::new();
        // Initial program in segment 0
        segments.push(Some(program));
        VirtualMachine {
            registers: [0; 8],
            segments,
            free_ids: Vec::new(),
            pc: 0,
            next_id: 1,
            segment_pool: Vec::new(),
        }
    }

    fn map_segment(&mut self, size: usize) -> SegmentID {
        let id = if let Some(reused) = self.free_ids.pop() {
            reused
        } else {
            let id = self.next_id;
            self.next_id += 1;
            // Expand vector to accommodate new segment
            self.segments.push(None);
            id
        };
        // Allocate and assign the segment
        self.segments[id] = Some(vec![0; size]);
        id as SegmentID
    }

    fn unmap_segment(&mut self, id: SegmentID) {
        let idx = id as usize;
        self.segments[idx] = None;
        self.free_ids.push(idx);
    }

    #[allow(dead_code)]
    fn load_value(&mut self, a: usize, value: Word) {
        self.registers[a] = value;
    }

    #[inline(never)]
    pub fn run(&mut self) {
        loop {
            // Fetch
            let segment0 = self.segments[0].as_ref().expect("Segment 0 missing");
            if self.pc >= segment0.len() {
                break;
            }
            let instruction = segment0[self.pc];
            self.pc += 1;
            let opcode = instruction >> 28;

            match opcode {
                0 => { // Conditional Move
                    let a = ((instruction >> 6) & 0x7) as usize;
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    if self.registers[c] != 0 {
                        self.registers[a] = self.registers[b];
                    }
                }
                1 => { // Segmented Load
                    let a = ((instruction >> 6) & 0x7) as usize;
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    let segment = self.segments[self.registers[b] as usize]
                        .as_ref().expect("Invalid segment");
                    self.registers[a] = segment[self.registers[c] as usize];
                }
                2 => { // Segmented Store
                    let a = ((instruction >> 6) & 0x7) as usize;
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    let segment = self.segments[self.registers[a] as usize]
                        .as_mut().expect("Invalid segment");
                    segment[self.registers[b] as usize] = self.registers[c];
                }
                3 => { // Addition
                    let a = ((instruction >> 6) & 0x7) as usize;
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    self.registers[a] = self.registers[b].wrapping_add(self.registers[c]);
                }
                4 => { // Multiplication
                    let a = ((instruction >> 6) & 0x7) as usize;
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    self.registers[a] = self.registers[b].wrapping_mul(self.registers[c]);
                }
                5 => { // Division
                    let a = ((instruction >> 6) & 0x7) as usize;
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    self.registers[a] = self.registers[b] / self.registers[c];
                }
                6 => { // Bitwise NAND
                    let a = ((instruction >> 6) & 0x7) as usize;
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    self.registers[a] = !(self.registers[b] & self.registers[c]);
                }
                7 => { // Halt
                    break;
                }
                8 => { // Map Segment
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    let size = self.registers[c] as usize;
                    let id = self.map_segment(size);
                    self.registers[b] = id;
                }
                9 => { // Unmap Segment
                    let c = (instruction & 0x7) as usize;
                    self.unmap_segment(self.registers[c]);
                }
                10 => { // Output
                    let c = (instruction & 0x7) as usize;
                    let byte = self.registers[c] as u8;
                    print!("{}", byte as char);
                    io::stdout().flush().unwrap();
                }
                11 => { // Input
                    let c = (instruction & 0x7) as usize;
                    let mut buffer = [0];
                    match io::stdin().read_exact(&mut buffer) {
                        Ok(_) => self.registers[c] = buffer[0] as Word,
                        Err(_) => self.registers[c] = !0,
                    }
                }
                12 => { // Load Program
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    if self.registers[b] != 0 {
                        if let Some(seg) = &self.segments[self.registers[b] as usize] {
                            let mut reused = self.segment_pool.pop().unwrap_or_else(|| Vec::with_capacity(seg.len()));
                            reused.clear(); // ensure it's empty
                            reused.extend_from_slice(seg); // copy data
                            self.segments[0] = Some(reused);
                            
                        } else {
                            panic!("Invalid segment");
                        }
                    }
                    self.pc = self.registers[c] as usize;
                }
                13 => { // Load Value
                    let a = ((instruction >> 25) & 0x7) as usize;
                    let value = instruction & 0x1FFFFFF;
                    self.registers[a] = value;
                }
                _ => panic!("Invalid opcode: {}", opcode),
            }
        }
    }
}