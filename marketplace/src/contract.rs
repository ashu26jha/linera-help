#![cfg_attr(target_arch = "wasm32", no_main)]
mod state;
use self::state::MarketPlace;
use async_trait::async_trait;
use fungible::{Destination, FungibleAccountOwner, FungibleTokenAbi, Account};
use linera_sdk::{
    base::{ApplicationId, SessionId, WithContractAbi, Amount},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage, contract::system_api,
};
use marketplace::Operation;
// use nft::{Account, AccountOwner, NFTabi};
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
            Operation::Buy { 
                owner,
                amount 
            } => {
                self.execute_pledge_with_account(owner, amount).await?;
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
    fn fungible_id() -> Result<ApplicationId<FungibleTokenAbi>, Error> {
        let hello = Self::parameters();

        hello
    }

    // fn nft_id() -> Result<ApplicationId<NFTabi>, Error> {
    //     let hello: Result<(ApplicationId<NFTabi>, ApplicationId<FungibleTokenAbi>), Error> =
    //         Self::parameters();
    //     let result_nft_id: Result<ApplicationId<NFTabi>, Error> = hello.map(|(nft_id, _)| nft_id);
    //     result_nft_id
    // }

    async fn receive_from_account(
        &mut self,
        owner: FungibleAccountOwner,
        amount: Amount,
    ) -> Result<(), Error> {
        let account = Account {
            chain_id: system_api::current_chain_id(),
            owner: FungibleAccountOwner::Application(system_api::current_application_id()),
        };
        let destination = Destination::Account(account);
        let transfer = fungible::ApplicationCall::Transfer {
            owner,
            amount,
            destination,
        };
        self.call_application(true, Self::fungible_id()?, &transfer, vec![])
            .await?;
        Ok(())
    }

    async fn execute_pledge_with_account(
        &mut self,
        owner: FungibleAccountOwner,
        amount: Amount,
    )->Result<(), Error>  {
        self.receive_from_account(owner, amount).await?;
        Ok(())
    }

    // async fn transfer_money(
    //     &mut self,
    //     buyer: FungibleAccountOwner,
    //     destination: Destination,
    //     listing_id: u64,
    // ) -> Result<(), Error> {
    //     let owner: FungibleAccountOwner = buyer;
    //     let price = self.get_price(listing_id).await;
    //     let call = fungible::ApplicationCall::Transfer {
    //         owner: owner,
    //         amount: price,
    //         destination: destination,
    //     };

    //     self.call_application(true, Self::fungible_id()?, &call, vec![])
    //         .await?;
    //     Ok(())
    // }

    // async fn buy_nft(&mut self, listing_id: u64, new_owner: Account) -> Result<(), Error> {
    //     let call = nft::ApplicationCall::Transfer {
    //         token_id: listing_id,
    //         new_owner: new_owner,
    //     };
    //     self.call_application(true, Self::nft_id()?, &call, vec![])
    //         .await?;

    //     Ok(())
    // }
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
