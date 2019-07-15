use std::cell::RefCell;
use std::sync::Arc;

use ethereum_types::{Address, U256};

use cita_vm::state::StateObjectInfo;
use cita_vm::InterpreterType;

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
    let mut vm = FakeVM::new();

    // let db = Arc::new(cita_vm::state::MemoryDB::new(false));
    // let mut state = cita_vm::state::State::new(db.clone()).unwrap();

    // state.new_contract(
    //     &Address::from("0x0000000000000000000000000000000000000001"),
    //     U256::from(100_000_000_000),
    //     U256::from(1),
    //     vec![],
    // );
    // state.new_contract(
    //     &Address::from("0x0000000000000000000000000000000000000002"),
    //     U256::from(200_000_000_000),
    //     U256::from(1),
    //     vec![],
    // );
    // state.commit().unwrap();
    // let root0 = state.root;

    // let block_data_provider: Arc<cita_vm::BlockDataProvider> = Arc::new(cita_vm::BlockDataProviderMock::default());
    // let state_data_provider = Arc::new(RefCell::new(state));
    // let context = cita_vm::Context::default();
    // let config = cita_vm::Config::default();

    // let tx = cita_vm::Transaction {
    //     from: Address::from("0x1000000000000000000000000000000000000000"),
    //     to: Some(Address::from("0x2000000000000000000000000000000000000000")),
    //     value: U256::from(5),
    //     nonce: U256::from(1),
    //     gas_limit: 80000,
    //     gas_price: U256::from(1),
    //     input: hex::decode("").unwrap(),
    //     itype: InterpreterType::EVM,
    // };
    // let _ = cita_vm::exec(
    //     block_data_provider.clone(),
    //     state_data_provider.clone(),
    //     context.clone(),
    //     config.clone(),
    //     tx,
    // );
    // state_data_provider.borrow_mut().commit().unwrap();

    // assert_eq!(
    //     state_data_provider
    //         .borrow_mut()
    //         .balance(&Address::from("0x2000000000000000000000000000000000000000"))
    //         .unwrap(),
    //     U256::from(100_005)
    // );
    // let mut ur_state = cita_vm::state::State::from_existing(db, root0).unwrap();
    // let b = ur_state
    //     .balance(&Address::from("0x2000000000000000000000000000000000000000"))
    //     .unwrap();
    // assert_eq!(b, U256::from(100_000));
}
