use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Read;
use std::rc::Rc;

use bytes::Bytes;
use cita_vm;
use ckb_vm;

fn main() {
    // Load binary
    let mut file = std::fs::File::open("./build/duktape").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    // Load js
    let js = std::fs::read("./examples/riscv_duktape.js").unwrap();
    let js: Bytes = js.into();

    // Initialize ret data
    let ret_data = Rc::new(RefCell::new(Vec::new()));

    // Initialize params
    let mut vm_params = cita_vm::evm::InterpreterParams::default();
    vm_params.address = ethereum_types::Address::from("0x0000000000000000000000000000000000000001");
    vm_params.origin = ethereum_types::Address::from("0x0000000000000000000000000000000000000002");
    vm_params.sender = ethereum_types::Address::from("0x0000000000000000000000000000000000000003");

    // Initialize context
    let vm_context = cita_vm::evm::Context::default();

    // Initialize storage
    let state = Rc::new(RefCell::new(cita_vm::evm::extmock::DataProviderMock::default()));
    let acc1 = ethereum_types::Address::from("0x0000000000000000000000000000000000000001");
    state.borrow_mut().db.insert(
        acc1,
        cita_vm::evm::extmock::Account {
            balance: ethereum_types::U256::from(10),
            code: vec![],
            nonce: ethereum_types::U256::from(0),
            storage: BTreeMap::new(),
        },
    );

    let mut machine =
        ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::SparseMemory<u64>>>::default()
            .syscall(Box::new(cita_vm::riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(cita_vm::riscv::SyscallEnvironment::new(
                vm_context.clone(),
                vm_params.clone(),
                state.clone(),
            )))
            .syscall(Box::new(cita_vm::riscv::SyscallRet::new(ret_data.clone())))
            .syscall(Box::new(cita_vm::riscv::SyscallStorage::new(
                vm_params.address.clone(),
                state.clone(),
            )))
            .build();

    machine.load_program(&buffer, &vec!["duktape".into(), js]).unwrap();
    let result = machine.run().unwrap();
    println!("exit={:#02x} ret={:?}", result, ret_data.borrow());
}
