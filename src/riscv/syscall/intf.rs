use ckb_vm::instructions::Register;

use crate::riscv::syscall::convention::SYSCODE_INTF;

pub struct SyscallIntf {}

impl SyscallIntf {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Mac: ckb_vm::SupportMachine> ckb_vm::Syscalls<Mac> for SyscallIntf {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];
        if code.to_i32() != SYSCODE_INTF {
            return Ok(false);
        }
        let addr = machine.registers()[ckb_vm::registers::A0].to_usize();
        let size = machine.registers()[ckb_vm::registers::A1].to_usize();
        let r_size_addr = machine.registers()[ckb_vm::registers::A2].to_usize();
        println!("Intf: addr={:?} size={:?} r_size_addr={:?}", addr, size, r_size_addr);
        Ok(true)
    }
}
