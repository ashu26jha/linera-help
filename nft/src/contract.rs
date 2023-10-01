#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::NFTtoken;
use async_trait::async_trait;
use linera_sdk::{
    base::{ApplicationId, Owner, SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use nft::{Account, AccountOwner, ApplicationCall, Message, Operation};
use thiserror::Error;

linera_sdk::contract!(NFTtoken);

impl WithContractAbi for NFTtoken {
    type Abi = nft::NFTabi;
}

#[async_trait]
impl Contract for NFTtoken {
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
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Approve {
                token_id,
                approved_for,
            } => {
                Self::check_account_authentication(
                    &mut self,
                    None,
                    context.authenticated_signer,
                    token_id,
                )
                .await?;

                self.approve_nft(token_id, approved_for).await;
                // Approves the NFT

                Ok(ExecutionResult::default())
            }
            Operation::Burn { token_id } => {
                Self::check_account_authentication(
                    &mut self,
                    None,
                    context.authenticated_signer,
                    token_id,
                )
                .await?;

                self.burn_nft(token_id).await;

                Ok(ExecutionResult::default())
            }

            Operation::Mint {
                owner,
                token_id,
                token_uri,
            } => {
                self.mint_nft(token_id, owner, token_uri).await;
                Ok(ExecutionResult::default())
            }

            Operation::Transfer {
                token_id,
                new_owner,
            } => {
                Self::check_account_authentication(
                    &mut self,
                    None,
                    context.authenticated_signer,
                    token_id,
                )
                .await?;

                self.transfer_nft_account(new_owner, token_id).await;
                Ok(ExecutionResult::default())
            }
        }
    }

    async fn execute_message(
        &mut self,
        _context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match message {
            Message::Transfer {
                token_id,
                target_account,
            } => {
                self.handle_message(token_id, target_account.owner).await;
                Ok(ExecutionResult::default())
            }
            Message::Recieve {
                token_id,
                target_account,
            } => {
                self.transfer_nft(token_id, target_account).await;
                Ok(ExecutionResult::default())
            }
        }
    }

    async fn handle_application_call(
        &mut self,
        context: &CalleeContext,
        call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        match call {
            ApplicationCall::Transfer {
                token_id,
                new_owner,
            } => {
                Self::check_account_authentication(
                    &mut self,
                    context.authenticated_caller_id,
                    context.authenticated_signer,
                    token_id,
                )
                .await?;
                self.transfer_nft_account( new_owner, token_id,).await;
                Ok(ApplicationCallResult::default())
            }
        }
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

impl NFTtoken {
    async fn transfer_nft_account(
        &mut self,
        reciever: Account,
        token_id: u64,
    ) -> ExecutionResult<Message> {

        if reciever.chain_id == system_api::current_chain_id() {
            self.transfer_nft(token_id, reciever.owner).await;
            return ExecutionResult::default();
        }

        let cross_chain_message = Message::Recieve {
            token_id: token_id,
            target_account: reciever.owner,
        };

        // Making change in its own chain
        self.transfer_nft(token_id, reciever.owner).await;
        // Sending the message : Hey change in yours 
        ExecutionResult::default()
            .with_authenticated_message(reciever.chain_id, cross_chain_message)
    }

    async fn check_account_authentication(
        &mut self,
        authenticated_application_id: Option<ApplicationId>,
        authenticated_signer: Option<Owner>,
        token_id: u64,
    ) -> Result<(), Error> {
        let old_owner = self.get_token_owner(token_id).await;
        let approve: AccountOwner = self.get_approvals(token_id).await;

        if let AccountOwner::User(address) = old_owner {
            if authenticated_signer == Some(address) {
                Ok(())
            } else {
                if let AccountOwner::User(address) = approve {
                    if authenticated_signer == Some(address) {
                        Ok(())
                    } else {
                        Err(Error::IncorrectAuthentication)
                    }
                } else {
                    Err(Error::IncorrectAuthentication)
                }
            }
        } else if let AccountOwner::Application(id) = old_owner {
            if authenticated_application_id == Some(id) {
                Ok(())
            } else {
                if let AccountOwner::Application(id) = approve {
                    if authenticated_application_id == Some(id) {
                        return Ok(());
                    } else {
                        return Err(Error::IncorrectAuthentication);
                    }
                }
                Err(Error::IncorrectAuthentication)
            }
        } else {
            Err(Error::IncorrectAuthentication)
        }
    }
}
/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),
    // Add more error variants here.

    // Not allow to perform
    #[error("The requested transfer is not correctly authenticated.")]
    IncorrectAuthentication,
}
