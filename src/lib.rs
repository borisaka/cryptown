#[macro_use]
extern crate exonum_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
use exonum::blockchain::{
    ExecutionError, ExecutionResult, Service, Transaction,
    TransactionContext, TransactionSet,
};
use exonum::crypto::{Hash, PublicKey};
use exonum::messages::RawTransaction;
use exonum::storage::{Fork, MapIndex, Snapshot};

mod proto;
mod common_structs;
mod identity;

use common_structs::Token;

const SERVICE_ID: u16 = 1;

pub const SERVICE_NAME: &str = "cryptocurrency";

pub struct CurrencySchema<T> {
    view: T,
}


impl<T: AsRef<Snapshot>> CurrencySchema<T> {
    pub fn new(view: T) -> Self {
        CurrencySchema { view }
    }

    // Utility method to get a list of all the wallets from the storage
    pub fn tokens(&self) -> MapIndex<&Snapshot, String, Token> {
        MapIndex::new("cryptocurrency.tokens", self.view.as_ref())
    }

    // Utility method to quickly get a separate wallet from the storage
    pub fn token(&self, symbol: &str) -> Option<Token> {
        self.tokens().get(symbol)
    }
}

impl<'a> CurrencySchema<&'a mut Fork> {
    pub fn tokens_mut(&mut self) -> MapIndex<&mut Fork, String, Token> {
        MapIndex::new("cryptocurrency.tokens", &mut self.view)
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::cryptocurrency::TxCreateToken")]
pub struct TxCreateToken {
    pub symbol: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum CurrencyTransactions {
    /// Create wallet transaction.
    CreateToken(TxCreateToken),
}

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Token already exists")]
    TokenAlreadyExists = 0
}

// Conversion between service-specific errors and the standard error type
// that can be emitted by transactions.
impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

impl Transaction for TxCreateToken {
    fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
        let author = context.author();
        let view = context.fork();
        let mut schema = CurrencySchema::new(view);
        if schema.token(&self.symbol).is_none() {
            let token = Token::new(&self.symbol, &author);
            println!("Create the token: {:?}", token);
            schema.tokens_mut().put(&self.symbol, token);
            Ok(())
        } else {
            Err(Error::TokenAlreadyExists)?
        }
    }
}

struct CryptocurrencyApi;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenQuery {
    pub symbol: String,
}

impl CryptocurrencyApi {
    /// Endpoint for getting a single wallet.
    pub fn get_token(
        state: &ServiceApiState,
        query: TokenQuery
    ) -> api::Result<Token> {
        let snapshot = state.snapshot();
        let schema = CurrencySchema::new(snapshot);
        schema
            .token(&query.symbol)
            .ok_or_else(|| api::Error::NotFound("\"Token not found\"".to_owned()))
    }

    /// Endpoint for dumping all wallets from the storage.
    pub fn get_tokens(
        state: &ServiceApiState,
        _query: ()
    ) -> api::Result<Vec<Token>> {
        let snapshot = state.snapshot();
        let schema = CurrencySchema::new(snapshot);
        let idx = schema.tokens();
        let tokens = idx.values().collect();
        Ok(tokens)
    }
}

impl CryptocurrencyApi {
    pub fn wire(builder: &mut ServiceApiBuilder) {
        // Binds handlers to the specific routes.
        builder
            .public_scope()
            .endpoint("/token", Self::get_token)
            .endpoint("/tokens", Self::get_tokens);
    }
}

#[derive(Debug)]
pub struct CurrencyService;

impl Service for CurrencyService {
    fn service_name(&self) -> &'static str {
        "cryptocurrency"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    // Implements a method to deserialize transactions coming to the node.
    fn tx_from_raw(
        &self,
        raw: RawTransaction
    ) -> Result<Box<dyn Transaction>, failure::Error> {
        let tx = CurrencyTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
        vec![]
    }

    // Links the service API implementation to Exonum.
    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        CryptocurrencyApi::wire(builder);
    }
}
