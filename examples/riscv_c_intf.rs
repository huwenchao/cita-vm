use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::rc::Rc;

use bytes::Bytes;
use cita_vm;
use ckb_vm::machine::CoreMachine;
use ckb_vm::memory::Memory;
use ckb_vm::Register;

fn main() {
    // Load binary
    let mut file = fs::File::open("./build/riscv_c_intf").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();

    let snapshot = Rc::new(RefCell::new(cita_vm::riscv::Snapshot::new()));
    let mut machine =
        ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::FlatMemory<u64>>>::default()
            .syscall(Box::new(cita_vm::riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(cita_vm::riscv::SyscallIntf::new(snapshot.clone())))
            .build();

    machine.load_program(&buffer, &vec!["riscv_c_main".into()]).unwrap();
    let _ = machine.run().unwrap();
    println!("snapshot={:?}", snapshot.borrow().registers);
    println!("memory_size={:?}", snapshot.borrow().memory.len());

    let mut machine =
        ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::FlatMemory<u64>>>::default()
            .syscall(Box::new(cita_vm::riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(cita_vm::riscv::SyscallIntf::new(snapshot.clone())))
            .build();

    machine.set_pc(snapshot.borrow().pc + 4);
    machine.set_register(0, 0);
    for i in 1..32 {
        machine.set_register(i, snapshot.borrow().registers[i]);
    }
    for i in 0..ckb_vm::RISCV_MAX_MEMORY {
        machine
            .memory_mut()
            .store8(&(i as u64), &u64::from(snapshot.borrow().memory[i]))
            .unwrap();
    }

    let addr = machine.registers()[ckb_vm::registers::A0].to_usize();
    let _ = machine.registers()[ckb_vm::registers::A1].to_usize();
    let r_size_addr = machine.registers()[ckb_vm::registers::A2].to_usize();

    let r = String::from("Hello World!");
    machine.memory_mut().store_bytes(addr, r.as_bytes()).unwrap();
    machine
        .memory_mut()
        .store_bytes(r_size_addr, &12u64.to_le_bytes()[..])
        .unwrap();

    let result = machine.run().unwrap();
    println!("exit={:#02x}", result);
}
