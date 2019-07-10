use std::error;
use std::fmt;

use ckb_vm;

use crate::evm;
use crate::state::Error as StateError;

#[derive(Debug)]
pub enum Error {
    Evm(evm::Error),
    Riscv(ckb_vm::Error),
    Secp256k1(secp256k1::Error),
    State(StateError),
    Str(String),
    NotEnoughBaseGas,
    NotEnoughBalance,
    InvalidNonce,
    ContractAlreadyExist,
    ExccedMaxCodeSize,
    ExccedMaxBlockGasLimit,
    ExccedMaxCallDepth,
    CreateInStaticCall,
    ExitCodeError,
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Evm(e) => return write!(f, "{}", e),
            Error::Riscv(e) => return write!(f, "{:?}", e),
            Error::Secp256k1(e) => return write!(f, "{:?}", e),
            Error::State(e) => return write!(f, "{}", e),
            Error::Str(e) => return write!(f, "{:?}", e),
            Error::NotEnoughBaseGas => return write!(f, "NotEnoughBaseGas"),
            Error::NotEnoughBalance => return write!(f, "NotEnoughBalance"),
            Error::InvalidNonce => return write!(f, "InvalidNonce"),
            Error::ContractAlreadyExist => return write!(f, "ContractAlreadyExist"),
            Error::ExccedMaxCodeSize => return write!(f, "ExccedMaxCodeSize"),
            Error::ExccedMaxBlockGasLimit => return write!(f, "ExccedMaxBlockGasLimit"),
            Error::ExccedMaxCallDepth => return write!(f, "ExccedMaxCallDepth"),
            Error::CreateInStaticCall => return write!(f, "CreateInStaticCall"),
            Error::ExitCodeError => return write!(f, "ExitCodeError"),
        };
    }
}

impl From<evm::Error> for Error {
    fn from(error: evm::Error) -> Self {
        Error::Evm(error)
    }
}

impl From<StateError> for Error {
    fn from(error: StateError) -> Self {
        Error::State(error)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(error: secp256k1::Error) -> Self {
        Error::Secp256k1(error)
    }
}

impl From<ckb_vm::Error> for Error {
    fn from(error: ckb_vm::Error) -> Self {
        Error::Riscv(error)
    }
}
