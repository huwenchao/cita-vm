use std::cell::RefCell;
use std::fs;
use std::sync::Arc;

use cita_trie;
use cita_vm;
use ethereum_types::{Address, U256};

#[test]
fn test_riscv_c() {
    let d = Arc::new(cita_trie::MemoryDB::new(false));
    let mut state_provider = cita_vm::State::new(d).unwrap();

    let context = cita_vm::Context::default();
    let tx = cita_vm::Transaction {
        from: Address::from("0x1000000000000000000000000000000000000000"),
        to: None,
        value: U256::from(10),
        nonce: U256::from(10),
        gas_limit: 100000,
        gas_price: U256::from(1),
        input: fs::read("./build/riscv_c_sdk").unwrap(),
        itype: cita_vm::InterpreterType::EVM,
    };

    let vm = cita_vm::Executive::new(Arc::new(BlockDataProviderMock::default()), state_provider, cfg);
    let r = vm.exec(context, tx);
    println!("{:?}", r);
}
