use std::cell::RefCell;
use std::sync::Arc;

use cita_trie::DB;
use ethereum_types::{Address, H256, U256};
use hashbrown::{HashMap, HashSet};
use log::debug;

use crate::evm;
use crate::state::State;

/// BlockDataProvider provides functions to get block's hash from chain.
///
/// Block data(only hash) are required to cita-vm from externalize database.
pub trait BlockDataProvider: Send + Sync {
    /// Function get_block_hash returns the block_hash of the specific block.
    fn get_block_hash(&self, number: &U256) -> H256;
}

/// BlockDataProviderMock is a mock for BlockDataProvider. We could use it in
/// tests or demos.
#[derive(Default)]
pub struct BlockDataProviderMock {
    data: HashMap<U256, H256>,
}

impl BlockDataProviderMock {
    /// Set blockhash for a specific block.
    pub fn set(&mut self, number: U256, hash: H256) {
        self.data.insert(number, hash);
    }
}

/// Impl.
impl BlockDataProvider for BlockDataProviderMock {
    fn get_block_hash(&self, number: &U256) -> H256 {
        *self.data.get(number).unwrap_or(&H256::zero())
    }
}

/// Store storages shared datas.
#[derive(Clone, Default)]
pub struct Store {
    pub refund: HashMap<Address, u64>,                 // For record refunds
    pub origin: HashMap<Address, HashMap<H256, H256>>, // For record origin value
    pub selfdestruct: HashSet<Address>,                // For self destruct
    // Field inused used for garbage collection.
    //
    // Test:
    //   ./tests/jsondata/GeneralStateTests/stSStoreTest/sstore_combinations_initial0.json
    //   ./tests/jsondata/GeneralStateTests/stSStoreTest/sstore_combinations_initial1.json
    //   ./tests/jsondata/GeneralStateTests/stSStoreTest/sstore_combinations_initial2.json
    pub inused: HashSet<Address>,
    pub evm_context: evm::Context,
    pub evm_cfg: evm::InterpreterConf,
}

impl Store {
    /// Merge with sub store.
    pub fn merge(&mut self, other: Arc<RefCell<Self>>) {
        self.refund = other.borrow().refund.clone();
        self.origin = other.borrow().origin.clone();
        self.selfdestruct = other.borrow().selfdestruct.clone();
        self.inused = other.borrow().inused.clone();
    }

    /// When a account has been read or write, record a log
    /// to prove that it has dose.
    pub fn used(&mut self, address: Address) {
        debug!("store used={:?}", address);
        self.inused.insert(address);
    }
}

/// An implemention for evm::DataProvider
pub struct DataProvider<B> {
    pub block_provider: Arc<BlockDataProvider>,
    pub state_provider: Arc<RefCell<State<B>>>,
    pub store: Arc<RefCell<Store>>,
}

impl<B: DB> DataProvider<B> {
    /// Create a new instance. It's obvious.
    pub fn new(b: Arc<BlockDataProvider>, s: Arc<RefCell<State<B>>>, store: Arc<RefCell<Store>>) -> Self {
        DataProvider {
            block_provider: b,
            state_provider: s,
            store,
        }
    }
}