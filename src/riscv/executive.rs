use crate::evm;
use crate::riscv;

use ckb_vm::machine::SupportMachine;

pub fn exec(
    max_cycles: u64,
    vm_context: evm::Context,
    vm_iparams: evm::InterpreterParams,
    state: std::rc::Rc<std::cell::RefCell<evm::DataProvider>>,
    code: bytes::Bytes,
    args: Vec<bytes::Bytes>,
) -> Result<u64, riscv::err::Error> {
    let ret_data = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let core_machine = ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new_with_max_cycles(max_cycles);
    let mut machine =
        ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::SparseMemory<u64>>>::new(core_machine)
            .instruction_cycle_func(Box::new(riscv::cost_model::instruction_cycles))
            .syscall(Box::new(riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(riscv::SyscallEnvironment::new(
                vm_context.clone(),
                vm_iparams.clone(),
                state.clone(),
            )))
            .syscall(Box::new(riscv::SyscallRet::new(ret_data.clone())))
            .syscall(Box::new(riscv::SyscallStorage::new(vm_iparams.address, state.clone())))
            .build();

    machine.load_program(&code, &args[..]).unwrap();
    let exitcode = machine.run()?;
    if exitcode != 0x00 {
        return Err(riscv::err::Error::ExitCodeError);
    }
    Ok(machine.cycles())
}
