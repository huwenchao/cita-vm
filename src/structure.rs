use ethereum_types::{Address, H256, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterpreterType {
    EVM,
    C,
    JS,
}

impl Default for InterpreterType {
    fn default() -> InterpreterType {
        InterpreterType::EVM
    }
}

#[derive(Clone, Debug, Default)]
pub struct Contract {
    pub code_address: Address,
    pub code_data: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct InterpreterParams {
    pub origin: Address,   // Who send the transaction
    pub sender: Address,   // Who send the call
    pub receiver: Address, // Who receive the transaction or call
    pub address: Address,  // Which storage used

    pub value: U256,
    pub input: Vec<u8>,
    pub itype: InterpreterType,
    pub nonce: U256,
    pub gas_limit: u64,
    pub gas_price: U256,

    pub read_only: bool,
    pub contract: Contract,
    pub extra: H256,
    pub is_create: bool,
    pub disable_transfer_value: bool,
    pub depth: u64,
}
