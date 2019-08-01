use std::fs;
use std::time::SystemTime;

use ethereum_types::U256;
use rand::prelude::*;

fn main() {
    let vm = cita_vm::FakeVM::new();

    let tx = cita_vm::Transaction {
        from: vm.account1.clone(),
        to: None,
        value: U256::from(0),
        nonce: U256::from(1),
        gas_limit: 1_000_000_000,
        gas_price: U256::from(1),
        input: fs::read("./build/riscv_c_dex").unwrap(),
        itype: cita_vm::InterpreterType::RISCV,
    };
    let r = vm.executor.exec(cita_vm::Context::default(), tx).unwrap();
    println!("{:?}", r);
    let contract_address = match r {
        cita_vm::InterpreterResult::Create(_, _, _, a) => a,
        _ => unreachable!(),
    };

    let test_clear_records_num: u32 = 3000;
    let one_struct_bytes = 60;
    let byte_num = test_clear_records_num * one_struct_bytes;
    let mut rng = rand::thread_rng();
    let clear_params = test_clear_records_num.to_le_bytes().to_vec();
    let data = (0..byte_num).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

    let tx = cita_vm::Transaction {
        from: vm.account1.clone(),
        to: Some(contract_address),
        value: U256::from(0),
        nonce: U256::from(2),
        gas_limit: 1_000_000_000,
        gas_price: U256::from(1),
        input: cita_vm::riscv::combine_parameters(vec!["clear".as_bytes().to_owned(), clear_params, data]),
        itype: cita_vm::InterpreterType::RISCV,
    };
    let now = SystemTime::now();
    let r = vm.executor.exec(cita_vm::Context::default(), tx).unwrap();
    let d = now.elapsed().unwrap().as_millis();
    println!("{:?}", r);
    println!(
        "Finish clearing {} records in {:?}s",
        test_clear_records_num,
        d as f64 / 1000.0,
    );
}
