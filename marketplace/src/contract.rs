#![cfg_attr(target_arch = "wasm32", no_main)]
mod state;
use self::state::MarketPlace;
use async_trait::async_trait;
use fungible::{Account, Destination, FungibleAccountOwner};
use linera_sdk::{
    base::{Amount, ApplicationId, ChainId, Owner, SessionId, WithContractAbi},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use log::info;
use marketplace::{Message, Operation};
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
                amount,
                destination,
            } => {
                let destination = Destination::Account(destination);
                self.execute_pledge_with_account(owner, amount, destination)
                    .await?;
                Ok(ExecutionResult::default())
            }
            Operation::List {
                token_id,
                price,
                owner,
                chain_id,
                token_uri,
            } => {
                self.add_listings(price, token_id, owner, chain_id, token_uri)
                    .await;
                Ok(ExecutionResult::default())
            }
            Operation::FetchBalance {
                listing_id,
                caller,
                chain_id,
            } => Ok(self.price_helper(listing_id, chain_id, caller).await),
        }
    }

    async fn execute_message(
        &mut self,
        _context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        info!("Message Recieved BC");
        match message {
            Message::FetchBalance { listing_id, caller } => {
                let price = self.get_price(listing_id).await;
                let chain_id = self.get_chain_id(listing_id).await;
                let account_owner = self.get_owner(listing_id).await;
                let owner = Account {
                    chain_id: chain_id,
                    owner: account_owner,
                };
                let message = Message::Price {
                    caller,
                    listing_id,
                    owner,
                    price,
                };
                Ok(ExecutionResult::default().with_authenticated_message(caller.chain_id, message))
            }

            Message::Price {
                caller,
                listing_id,
                owner,
                price,
            } => {
                let destination = Destination::Account(owner);

                let _ = self.transfer_funds(caller.owner, price, destination).await;
                let token_id: u64 = listing_id;

                let owner = self.extract_owner(caller.owner);
                let acc = nft::AccountOwner::User(owner);

                let new_owner = nft::Account {
                    chain_id: caller.chain_id,
                    owner: acc,
                };

                let _ = self.transfer_nft(new_owner, token_id).await;

                Ok(ExecutionResult::default())
            }
        }
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
        Err(Error::SessionError)
    }
}

impl MarketPlace {
    fn extract_owner(&mut self, owner_enum: FungibleAccountOwner) -> Owner {
        match owner_enum {
            FungibleAccountOwner::User(owner) => owner,
            _ => panic!("Trying to transfer NFT to contract which is not allowed"),
        }
    }

    fn fungible_id() -> Result<ApplicationId<fungible::FungibleTokenAbi>, Error> {
        Ok(Self::parameters()?.fungible_app_id)
    }

    fn nft_id() -> Result<ApplicationId<nft::NFTabi>, Error> {
        Ok(Self::parameters()?.nft_app_id)
    }

    async fn transfer_nft(&mut self, new_owner: nft::Account, token_id: u64) -> Result<(), Error> {
        info!("Transfering NFT");
        let transfer = nft::ApplicationCall::Transfer {
            token_id,
            new_owner,
        };
        self.call_application(true, Self::nft_id()?, &transfer, vec![])
            .await?;
        Ok(())
    }

    async fn transfer_funds(
        &mut self,
        owner: FungibleAccountOwner,
        amount: Amount,
        destination: Destination,
    ) -> Result<(), Error> {
        let transfer = fungible::ApplicationCall::Transfer {
            owner: owner,
            amount: amount,
            destination: destination,
        };

        self.call_application(true, Self::fungible_id()?, &transfer, vec![])
            .await?;

        Ok(())
    }

    async fn price_helper(
        &mut self,
        listing_id: u64,
        chain_id: ChainId,
        caller: Account,
    ) -> ExecutionResult<Message> {
        info!("Sending message");
        let message = Message::FetchBalance { listing_id, caller };
        ExecutionResult::default().with_message(chain_id, message)
    }

    async fn receive_from_account(
        &mut self,
        owner: FungibleAccountOwner,
        amount: Amount,
        destination: Destination,
    ) -> Result<(), Error> {
        let transfer = fungible::ApplicationCall::Transfer {
            owner,
            amount,
            destination,
        };
        info!("Calling other application");
        self.call_application(true, Self::fungible_id()?, &transfer, vec![])
            .await?;
        Ok(())
    }

    async fn execute_pledge_with_account(
        &mut self,
        owner: FungibleAccountOwner,
        amount: Amount,
        destination: Destination,
    ) -> Result<(), Error> {
        self.receive_from_account(owner, amount, destination)
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

    #[error("Session call not supported")]
    SessionError,
}
