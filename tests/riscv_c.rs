use std::fs;
use std::sync::Arc;

use cita_trie;
use cita_vm;
use ethereum_types::{Address, U256};

#[test]
fn test_riscv_c() {
    let d = Arc::new(cita_trie::MemoryDB::new(false));
    let mut state_provider = cita_vm::State::new(d).unwrap();
    state_provider.new_contract(
        &Address::from("0x1000000000000000000000000000000000000000"),
        U256::from(100000000),
        U256::from(1),
        vec![],
    );

    let context = cita_vm::Context::default();
    let cfg = cita_vm::Config::default();
    let vm = cita_vm::Executive::new(Arc::new(cita_vm::BlockDataProviderMock::default()), state_provider, cfg);

    let tx = cita_vm::Transaction {
        from: Address::from("0x1000000000000000000000000000000000000000"),
        to: None,
        value: U256::from(10),
        nonce: U256::from(1),
        gas_limit: 1000000,
        gas_price: U256::from(1),
        input: fs::read("./build/riscv_c_sdk").unwrap(),
        itype: cita_vm::InterpreterType::EVM,
    };
    let r = vm.exec(context.clone(), tx).unwrap();
    println!("{:?}", r);
    let (_, _, _, addr) = match r {
        cita_vm::InterpreterResult::Normal(_, _, _) => unreachable!(),
        cita_vm::InterpreterResult::Revert(_, _) => unreachable!(),
        cita_vm::InterpreterResult::Create(a, b, c, d) => (a, b, c, d),
    };

    let tx = cita_vm::Transaction {
        from: Address::from("0x1000000000000000000000000000000000000000"),
        to: Some(addr),
        value: U256::from(10),
        nonce: U256::from(2),
        gas_limit: 1000000,
        gas_price: U256::from(1),
        input: vec![],
        itype: cita_vm::InterpreterType::C,
    };
    let r = vm.exec(context.clone(), tx).unwrap();
    println!("{:?}", r);
}
