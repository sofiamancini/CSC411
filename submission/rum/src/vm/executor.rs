use anyhow::{anyhow, Result};
use crate::{
    VirtualMachine,
    disasm::decoder::{Instruction, Opcode},
    io::IoHandler,
};

pub struct ExecutionUnit;

impl ExecutionUnit {
    pub fn execute<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        match inst.opcode()? {
            Opcode::CMov => ExecutionUnit::execute_cmov(vm, inst)?,
            Opcode::Load => ExecutionUnit::execute_load(vm, inst)?,
            Opcode::Store => ExecutionUnit::execute_store(vm, inst)?,
            Opcode::Add => ExecutionUnit::execute_add(vm, inst)?,
            Opcode::Mul => ExecutionUnit::execute_mul(vm, inst)?,
            Opcode::Div => ExecutionUnit::execute_div(vm, inst)?,
            Opcode::Nand => ExecutionUnit::execute_nand(vm, inst)?,
            Opcode::Halt => ExecutionUnit::execute_halt(vm),
            Opcode::MapSegment => ExecutionUnit::execute_map_segment(vm, inst)?,
            Opcode::UnmapSegment => ExecutionUnit::execute_unmap_segment(vm, inst)?,
            Opcode::Output => ExecutionUnit::execute_output(vm, inst)?,
            Opcode::Input => ExecutionUnit::execute_input(vm, inst)?,
            Opcode::LoadProgram => ExecutionUnit::execute_load_program(vm, inst)?,
            Opcode::LoadValue => ExecutionUnit::execute_load_value(vm, inst)?,  
            Opcode::Invalid => return Err(anyhow!("Invalid opcode")),   
        }
        Ok(())
    }

    fn execute_cmov<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let c = inst.get_c()?;
        if vm.registers.get(c)? != 0 {
            let b = inst.get_b()?;
            let a = inst.get_a()?;
            let val = vm.registers.get(b)?;
            vm.registers.set(a, val)?;
        }
        Ok(())
    }

    fn execute_load<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let a = inst.get_a()?;
        let base = vm.registers.get(b)?;
        let idx = vm.registers.get(c)?;
        let val = vm.memory.get(base, idx)?;
        vm.registers.set(a, val)?;
        Ok(())
    }

    fn execute_store<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let a = inst.get_a()?;
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let seg = vm.registers.get(a)?;
        let idx = vm.registers.get(b)?;
        let val = vm.registers.get(c)?;
        vm.memory.set(seg, idx, val)?;
        Ok(())
    }

    fn execute_add<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let a = inst.get_a()?;
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let val = vm.registers.get(b)?.wrapping_add(vm.registers.get(c)?);
        vm.registers.set(a, val)?;
        Ok(())
    }

    fn execute_mul<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let a = inst.get_a()?;
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let val = vm.registers.get(b)?.wrapping_mul(vm.registers.get(c)?);
        vm.registers.set(a, val)?;
        Ok(())
    }

    fn execute_div<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let a = inst.get_a()?;
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let divisor = vm.registers.get(c)?;
        if divisor == 0 {
            return Err(anyhow!("Division by zero"));
        }
        let val = vm.registers.get(b)? / divisor;
        vm.registers.set(a, val)?;
        Ok(())
    }

    fn execute_nand<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let a = inst.get_a()?;
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let val = !(vm.registers.get(b)? & vm.registers.get(c)?);
        vm.registers.set(a, val)?;
        Ok(())
    }

    fn execute_halt<IO: IoHandler>(vm: &mut VirtualMachine<IO>) {
        vm.running = false;
    }

    fn execute_map_segment<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let size = vm.registers.get(c)? as usize;
        let seg_id = vm.memory.map_segment(size.try_into().unwrap())?;
        vm.registers.set(b, seg_id)?;
        Ok(())
    }

    fn execute_unmap_segment<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let c = inst.get_c()?;
        let seg_id = vm.registers.get(c)?;
        vm.memory.unmap_segment(seg_id)?;
        Ok(())
    }

    fn execute_output<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let _c = inst.get_c()?;
        // Update to read from register 1 instead of register 0
        let val = vm.registers.get(1)?;  // Always read from register 1 (since your UM binary uses register 1)
        
        if val > 255 {
            return Err(anyhow!("Output value out of bounds: {}", val));
        }
        
        vm.io.write_byte(val as u8)?;
        Ok(())
    }

    fn execute_input<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let c = inst.get_c()?;
        match vm.io.read_byte()? {
            Some(byte) => vm.registers.set(c, byte as u32)?,
            None => vm.registers.set(c, u32::MAX)?, // EOF marker
        }
        Ok(())
    }

    fn execute_load_program<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let b = inst.get_b()?;
        let c = inst.get_c()?;
        let seg_id = vm.registers.get(b)?;
        if seg_id != 0 {
            vm.memory.load_program(seg_id)?;
        }
        vm.pc = vm.registers.get(c)?;
        Ok(())
    }

    fn execute_load_value<IO: IoHandler>(
        vm: &mut VirtualMachine<IO>,
        inst: Instruction
    ) -> Result<()> {
        let a = inst.get_a()?;
        let val = inst.get_value()?;
        vm.registers.set(a, val)?;
        Ok(())
    }
}