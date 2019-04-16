// #[macro_use]
// extern crate exonum_derive;
// #[macro_use]
// extern crate failure;
// #[macro_use]
// extern crate serde_derive;
use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
use exonum::blockchain::{
    ExecutionError, ExecutionResult, Service, Transaction,
    TransactionContext, TransactionSet,
};
use exonum::crypto::{Hash, PublicKey};
use exonum::messages::RawTransaction;
use exonum::storage::{Fork, MapIndex, Snapshot};

use crate::proto;

#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::cryptocurrency::Token")]
pub struct Token {
    pub owner: PublicKey,
    pub symbol: String
}

impl Token {
    pub fn new(symbol: &str, &owner: &PublicKey) -> Self {
        Self {
            owner,
            symbol: symbol.to_owned()
        }
    }
}

