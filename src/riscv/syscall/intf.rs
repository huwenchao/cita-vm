use std::cell::RefCell;
use std::rc::Rc;

use ckb_vm::instructions::Register;

use crate::riscv::syscall::convention::SYSCODE_INTF;

pub struct Snapshot<R> {
    pub registers: Vec<R>,
}

impl<R: ckb_vm::Register> Snapshot<R> {
    pub fn new() -> Self {
        Self { registers: vec![] }
    }
}

pub struct SyscallIntf {
    snapshot: Rc<RefCell<Snapshot<u64>>>,
}

impl SyscallIntf {
    pub fn new(snapshot: Rc<RefCell<Snapshot<u64>>>) -> Self {
        Self { snapshot }
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
        for e in machine.registers() {
            self.snapshot.borrow_mut().registers.push(e.to_u64());
        }
        let addr = machine.registers()[ckb_vm::registers::A0].to_usize();
        let size = machine.registers()[ckb_vm::registers::A1].to_usize();
        let r_size_addr = machine.registers()[ckb_vm::registers::A2].to_usize();
        println!("Intf: addr={:?} size={:?} r_size_addr={:?}", addr, size, r_size_addr);
        Ok(true)
    }
}
