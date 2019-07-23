use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::rc::Rc;

use bytes::Bytes;
use cita_vm;

fn main() {
    // Load binary
    let mut file = fs::File::open("./build/riscv_c_intf").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let snapshot = Rc::new(RefCell::new(cita_vm::riscv::Snapshot::new()));
    let mut machine =
        ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::SparseMemory<u64>>>::default()
            .syscall(Box::new(cita_vm::riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(cita_vm::riscv::SyscallIntf::new(snapshot.clone())))
            .build();

    machine.load_program(&buffer, &vec!["riscv_c_main".into()]).unwrap();
    let result = machine.run().unwrap();

    println!("snapshot={:?}", snapshot.borrow().registers);
    println!("memory_size={:?}", snapshot.borrow().memory.len());

    let mut machine =
        ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::SparseMemory<u64>>>::default()
            .syscall(Box::new(cita_vm::riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(cita_vm::riscv::SyscallIntf::new(snapshot.clone())))
            .build();

    println!("exit={:#02x}", result);
}
