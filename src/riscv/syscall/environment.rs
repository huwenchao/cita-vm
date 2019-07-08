//! Environmental Information
use ckb_vm::instructions::Register;
use ckb_vm::memory::Memory;

use crate::evm::Context;
use crate::evm::InterpreterParams;
use crate::riscv::syscall::convention::SYSCODE_ADDRESS;

pub struct SyscallEnvironment {
    context: Context,
    iparams: InterpreterParams,
}

impl SyscallEnvironment {
    pub fn new(context: Context, iparams: InterpreterParams) -> Self {
        Self { context, iparams }
    }
}

impl<Mac: ckb_vm::SupportMachine> ckb_vm::Syscalls<Mac> for SyscallEnvironment {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];
        match code.to_i32() {
            SYSCODE_ADDRESS => {
                let addr = machine.registers()[ckb_vm::registers::A0].to_usize();
                machine.memory_mut().store_bytes(addr, &self.iparams.address)?;
                machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));
                Ok(true)
            }
            _ => Ok(false)
        }
    }
}
