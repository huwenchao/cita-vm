use std::fs;
use std::sync::Arc;

use ethereum_types::{Address, U256};

pub struct FakeVM {
    pub account1: Address,
    pub account2: Address,
    pub executor: cita_vm::Executive<cita_vm::state::MemoryDB>,
}

impl FakeVM {
    pub fn new() -> Self {
        let account1 = Address::from("0x0000000000000000000000000000000000000001");
        let account2 = Address::from("0x0000000000000000000000000000000000000002");

        let db = Arc::new(cita_vm::state::MemoryDB::new(false));
        let mut state = cita_vm::state::State::new(db.clone()).unwrap();
        state.new_contract(&account1, U256::from(100_000_000), U256::from(1), vec![]);
        state.new_contract(&account2, U256::from(200_000_000), U256::from(1), vec![]);
        state.commit().unwrap();

        let block_data_provider: Arc<cita_vm::BlockDataProvider> = Arc::new(cita_vm::BlockDataProviderMock::default());

        Self {
            account1,
            account2,
            executor: cita_vm::Executive::new(block_data_provider, state, cita_vm::Config::default()),
        }
    }
}

fn main() {
    let vm = FakeVM::new();

    let tx = cita_vm::Transaction {
        from: vm.account1.clone(),
        to: None,
        value: U256::from(0),
        nonce: U256::from(1),
        gas_limit: 1_000_000,
        gas_price: U256::from(1),
        input: fs::read("./build/riscv_c_simplestorage").unwrap(),
        itype: cita_vm::InterpreterType::C,
    };
    let r = vm.executor.exec(cita_vm::Context::default(), tx).unwrap();
    println!("{:?}", r);
    let contract_address = match r {
        cita_vm::InterpreterResult::Create(_, _, _, a) => a,
        _ => unreachable!(),
    };

    let tx = cita_vm::Transaction {
        from: vm.account1.clone(),
        to: Some(contract_address),
        value: U256::from(0),
        nonce: U256::from(2),
        gas_limit: 1_000_000,
        gas_price: U256::from(1),
        input: cita_vm::riscv::combine_parameters(
            vec!["set", "everything", "42"]
                .iter()
                .map(|e| String::from(*e))
                .collect(),
        ),
        itype: cita_vm::InterpreterType::C,
    };
    let r = vm.executor.exec(cita_vm::Context::default(), tx).unwrap();
    println!("{:?}", r);

    let tx = cita_vm::Transaction {
        from: vm.account1.clone(),
        to: Some(contract_address),
        value: U256::from(0),
        nonce: U256::from(3),
        gas_limit: 1_000_000,
        gas_price: U256::from(1),
        input: cita_vm::riscv::combine_parameters(vec!["get", "everything"].iter().map(|e| String::from(*e)).collect()),
        itype: cita_vm::InterpreterType::C,
    };
    let r = vm.executor.exec(cita_vm::Context::default(), tx).unwrap();
    println!("{:?}", r);
}
