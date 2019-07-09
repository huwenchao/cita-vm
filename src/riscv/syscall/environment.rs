//! Environmental Information
use std::cell::RefCell;
use std::rc::Rc;

use ckb_vm::instructions::Register;
use ckb_vm::memory::Memory;

use crate::evm::Context;
use crate::evm::DataProvider;
use crate::evm::InterpreterParams;
use crate::riscv::syscall::common::get_arr;
use crate::riscv::syscall::convention::{SYSCODE_ADDRESS, SYSCODE_BALANCE, SYSCODE_ORIGIN};

pub struct SyscallEnvironment {
    context: Context,
    iparams: InterpreterParams,
    data: Rc<RefCell<DataProvider>>,
}

impl SyscallEnvironment {
    pub fn new(context: Context, iparams: InterpreterParams, data: Rc<RefCell<DataProvider>>) -> Self {
        Self { context, iparams, data }
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
            SYSCODE_BALANCE => {
                let addr_addr = machine.registers()[ckb_vm::registers::A0].to_usize();
                let v_addr = machine.registers()[ckb_vm::registers::A1].to_usize();

                let addr_byte = get_arr(machine, addr_addr, 20)?;
                let addr_h160 = ethereum_types::Address::from(&addr_byte[..]);

                let v_u256 = self.data.borrow().get_balance(&addr_h160);
                let mut v_byte = [0x00u8; 32];
                v_u256.to_big_endian(&mut v_byte);
                machine.memory_mut().store_bytes(v_addr, &v_byte)?;
                Ok(true)
            }
            SYSCODE_ORIGIN => {
                let addr = machine.registers()[ckb_vm::registers::A0].to_usize();
                machine.memory_mut().store_bytes(addr, &self.iparams.origin)?;
                machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
