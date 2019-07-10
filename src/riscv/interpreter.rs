use std::cell::RefCell;
use std::rc::Rc;

use bytes::Bytes;
use ckb_vm::machine::SupportMachine;

use crate::evm;
use crate::riscv;
use crate::{Context, InterpreterParams, InterpreterResult};

pub struct Interpreter {
    pub context: Context,
    pub iparams: InterpreterParams,
    pub data_provider: Rc<RefCell<evm::DataProvider>>,
}

impl Interpreter {
    pub fn new(context: Context, iparams: InterpreterParams, data_provider: Rc<RefCell<evm::DataProvider>>) -> Self {
        Self {
            context,
            iparams,
            data_provider,
        }
    }

    pub fn run(&mut self) -> Result<InterpreterResult, ckb_vm::Error> {
        let code = Bytes::from(self.iparams.contract.code_data.clone());
        let args = self.iparams.input.as_slice()[8..].to_vec();
        let args: Vec<Bytes> = args.split(|e| *e == 0x00).map(Bytes::from).collect();

        let ret_data = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let core_machine =
            ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new_with_max_cycles(self.iparams.gas_limit);
        let mut machine =
            ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::SparseMemory<u64>>>::new(
                core_machine,
            )
            .instruction_cycle_func(Box::new(riscv::cost_model::instruction_cycles))
            .syscall(Box::new(riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(riscv::SyscallEnvironment::new(
                self.context.clone(),
                self.iparams.clone(),
                self.data_provider.clone(),
            )))
            .syscall(Box::new(riscv::SyscallRet::new(ret_data.clone())))
            .syscall(Box::new(riscv::SyscallStorage::new(
                self.iparams.address,
                self.data_provider.clone(),
            )))
            .build();

        machine.load_program(&code, &args[..]).unwrap();
        let exitcode = machine.run()?;
        let cycles = machine.cycles();
        if exitcode != 0x00 {
            Ok(InterpreterResult::Revert(ret_data.borrow().to_vec(), cycles))
        } else {
            Ok(InterpreterResult::Normal(ret_data.borrow().to_vec(), cycles, vec![]))
        }
    }
}

// pub fn exec(
//     context: Context,
//     iparams: InterpreterParams,
//     state: std::rc::Rc<std::cell::RefCell<evm::DataProvider>>,
//     code: bytes::Bytes,
//     args: Vec<bytes::Bytes>,
// ) -> Result<u64, riscv::err::Error> {
//     let ret_data = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
//     let core_machine =
//         ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new_with_max_cycles(iparams.gas_limit);
//     let mut machine =
//         ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::SparseMemory<u64>>>::new(core_machine)
//             .instruction_cycle_func(Box::new(riscv::cost_model::instruction_cycles))
//             .syscall(Box::new(riscv::SyscallDebug::new("riscv:", std::io::stdout())))
//             .syscall(Box::new(riscv::SyscallEnvironment::new(
//                 context.clone(),
//                 iparams.clone(),
//                 state.clone(),
//             )))
//             .syscall(Box::new(riscv::SyscallRet::new(ret_data.clone())))
//             .syscall(Box::new(riscv::SyscallStorage::new(iparams.address, state.clone())))
//             .build();

//     machine.load_program(&code, &args[..]).unwrap();
//     let exitcode = machine.run()?;
//     if exitcode != 0x00 {
//         return Err(riscv::err::Error::ExitCodeError);
//     }
//     Ok(machine.cycles())
// }
