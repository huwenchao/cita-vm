use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use bytes::Bytes;
use ckb_vm::machine::SupportMachine;

use crate::evm::DataProvider;
use crate::riscv;
use crate::{Context, InterpreterParams, InterpreterResult, InterpreterType};

pub struct Interpreter {
    pub context: Context,
    pub iparams: InterpreterParams,
    pub data_provider: Rc<RefCell<DataProvider>>,
}

impl Interpreter {
    pub fn new(context: Context, iparams: InterpreterParams, data_provider: Rc<RefCell<DataProvider>>) -> Self {
        Self {
            context,
            iparams,
            data_provider,
        }
    }

    pub fn run(&mut self) -> Result<InterpreterResult, ckb_vm::Error> {
        let contract_code = Bytes::from(self.iparams.contract.code_data.clone());
        let contract_args: Vec<Bytes> = self
            .iparams
            .input
            .split(|e| *e == 0x00)
            .filter(|e| !e.is_empty())
            .map(Bytes::from)
            .collect();

        let (code, args) = match self.iparams.itype {
            InterpreterType::C => {
                let code = contract_code.clone();
                let mut args = contract_args.clone();
                args.insert(0, Bytes::from("main"));
                (code, args)
            }
            InterpreterType::JS => {
                let code = Bytes::from(fs::read("./build/duktape").unwrap());
                let mut args = contract_args.clone();
                args.insert(0, contract_code.clone());
                args.insert(0, Bytes::from("main"));
                (code, args)
            }
            _ => unreachable!(),
        };

        let ret_data = Rc::new(RefCell::new(Vec::new()));
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
