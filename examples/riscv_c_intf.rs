use std::fs;
use std::sync::Arc;

use cita_trie;
use cita_vm;
use ethereum_types::{Address, U256};

fn main() {

    // let vm = cita_vm::Executive::new(Arc::new(cita_vm::BlockDataProviderMock::default()), state_provider, cfg);

    // let tx = cita_vm::Transaction {
    //     from: Address::from("0x1000000000000000000000000000000000000000"),
    //     to: None,
    //     value: U256::from(10),
    //     nonce: U256::from(1),
    //     gas_limit: 1000000,
    //     gas_price: U256::from(1),
    //     input: fs::read("./build/riscv_c_fibonacci").unwrap(),
    //     itype: cita_vm::InterpreterType::RISCV,
    // };
    // let r = vm.exec(context.clone(), tx).unwrap();
    // println!("{:?}", r);
    // let (_, _, _, addr) = match r {
    //     cita_vm::InterpreterResult::Normal(_, _, _) => unreachable!(),
    //     cita_vm::InterpreterResult::Revert(_, _) => unreachable!(),
    //     cita_vm::InterpreterResult::Create(a, b, c, d) => (a, b, c, d),
    // };

    // let tx = cita_vm::Transaction {
    //     from: Address::from("0x1000000000000000000000000000000000000000"),
    //     to: Some(addr),
    //     value: U256::from(10),
    //     nonce: U256::from(2),
    //     gas_limit: 1000000,
    //     gas_price: U256::from(1),
    //     input: cita_vm::riscv::combine_parameters(vec!["10".into()]),
    //     itype: cita_vm::InterpreterType::RISCV,
    // };
    // let r = vm.exec(context.clone(), tx).unwrap();
    // println!("{:?}", r);
}
