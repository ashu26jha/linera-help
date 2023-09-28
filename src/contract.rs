#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::NFTtoken;
use async_trait::async_trait;
use linera_sdk::{
    base::{Owner, SessionId, WithContractAbi},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use nft::Operation;
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
                    context.authenticated_signer,
                    token_id,
                )
                .await?;
                self.transfer_nft(token_id, new_owner).await;
                Ok(ExecutionResult::default())
            }

            Operation::Approve {
                token_id,
                approved_for,
            } => {
                Self::check_account_authentication(
                    &mut self,
                    context.authenticated_signer,
                    token_id,
                )
                .await?;
                // Checks for auth

                self.approve_nft(token_id, approved_for).await;
                // Approves the NFT

                Ok(ExecutionResult::default())
            }

            Operation::Burn { token_id } => {
                Self::check_account_authentication(
                    &mut self,
                    context.authenticated_signer,
                    token_id,
                )
                .await?;

                self.burn_nft(token_id).await;

                Ok(ExecutionResult::default())
            }
        }
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

#[allow(dead_code)]

impl NFTtoken {
    async fn check_account_authentication(
        &mut self,
        authenticated_signed: Option<Owner>,
        token_id: u64,
    ) -> Result<(), Error> {
        let old_owner: Owner = self.get_token_owner(token_id).await;
        let approve = self.get_approvals(token_id).await;

        if authenticated_signed == Some(old_owner) {
            return Ok(());
        }

        if authenticated_signed == Some(approve) {
            return Ok(());
        }

        Err(Error::IncorrectAuthentication)
    }
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),

    #[error("Incorrect Authentication")]
    IncorrectAuthentication, // Add more error variants here.

    #[error("Sessions not supported")]
    SessionsNotSupported,
}
