use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bitpack::bitpack;

type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
pub enum Opcode {
    CMov = 0,
    Load = 1,
    Store = 2,
    Add = 3,
    Mul = 4,
    Div = 5,
    Nand = 6,
    Halt = 7,
    MapSegment = 8,
    UnmapSegment = 9,
    Output = 10,
    Input = 11,
    LoadProgram = 12,
    LoadValue = 13,
}

/// Extracts a field from an instruction
pub fn get(field: &Field, instruction: Umi) -> u32 {
    bitpack::getu(instruction as u64, field.width as u64, field.lsb as u64)
        .unwrap() as u32
}

/// Extracts the opcode from an instruction
fn op(instruction: Umi) -> Option<Opcode> {
    let opcode_num = bitpack::getu(instruction as u64, OP.width as u64, OP.lsb as u64)
        .unwrap() as u32;
    FromPrimitive::from_u32(opcode_num)
}

/// Disassembles a single UM instruction into its assembly representation
pub fn disassemble(inst: Umi) -> String {
    match op(inst) {
        Some(Opcode::CMov) => {
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::Load) => {
            format!(
                "r{} := m[r{}][r{}];",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Store) => {
            format!(
                "m[r{}][r{}] := r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Add) => {
            format!(
                "r{} := r{} + r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Mul) => {
            format!(
                "r{} := r{} * r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Div) => {
            format!(
                "r{} := r{} / r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Nand) => {
            format!(
                "r{} := ~(r{} & r{});",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Halt) => "halt".to_string(),
        Some(Opcode::MapSegment) => {
            format!(
                "r{} := map_segment(r{});",
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::UnmapSegment) => {
            format!("unmap_segment(r{});", get(&RC, inst))
        }
        Some(Opcode::Output) => {
            format!("output(r{});", get(&RC, inst))
        }
        Some(Opcode::Input) => {
            format!("r{} := input();", get(&RC, inst))
        }
        Some(Opcode::LoadProgram) => {
            format!(
                "load_program(m[r{}]); if (r{} != 0) set_program_counter(r{});",
                get(&RB, inst),
                get(&RC, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::LoadValue) => {
            let value = get(&VL, inst);
            format!("r{} := {};", get(&RL, inst), value)
        }
        None => format!("unknown opcode: {:#x}", inst),
    }
}