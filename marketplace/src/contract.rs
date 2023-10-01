#![cfg_attr(target_arch = "wasm32", no_main)]
mod state;
use self::state::MarketPlace;
use async_trait::async_trait;
use linera_sdk::{
    base::{ApplicationId, SessionId, WithContractAbi},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use marketplace::Operation;
use nft::{NFTabi, Account};
use thiserror::Error;

linera_sdk::contract!(MarketPlace);

impl WithContractAbi for MarketPlace {
    type Abi = marketplace::MarketPlaceABI;
}

#[async_trait]
impl Contract for MarketPlace {
    type Error = Error;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        _argument: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        _context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Buy { buyer, list_id } => {
                let nft_status = self.get_status(list_id).await;
                if nft_status {
                    self.buy_nft(list_id, buyer).await?;
                }
            }
            Operation::List { token_id, price } => {
                self.add_listings(price, token_id).await;
            }
        }
        Ok(ExecutionResult::default())
    }

    async fn execute_message(
        &mut self,
        _context: &MessageContext,
        _message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        Ok(ExecutionResult::default())
    }

    async fn handle_application_call(
        &mut self,
        _context: &CalleeContext,
        _call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        Ok(ApplicationCallResult::default())
    }

    async fn handle_session_call(
        &mut self,
        _context: &CalleeContext,
        _session: Self::SessionState,
        _call: Self::SessionCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<SessionCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        Ok(SessionCallResult::default())
    }
}

impl MarketPlace {
    fn nft_id() -> Result<ApplicationId<NFTabi>, Error> {
        Self::parameters()
    }

    async fn buy_nft(&mut self, listing_id: u64, new_owner: Account) -> Result<(), Error> {
        let call = nft::ApplicationCall::Transfer {
            token_id: listing_id,
            new_owner: new_owner,
        };
        self.call_application(true, Self::nft_id()?, &call, vec![])
            .await?;

        Ok(())
    }
}
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),
    // Add more error variants here.
    #[error("Already sold")]
    NFTsoldError,
}
