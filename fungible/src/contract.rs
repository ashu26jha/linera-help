// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;
use self::state::FungibleToken;
use async_trait::async_trait;
use fungible::{
    Account, ApplicationCall, Destination, FungibleAccountOwner, Message, Operation, SessionCall,
};
use linera_sdk::{
    base::{Amount, ApplicationId, Owner, SessionId, WithContractAbi, ChainId},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use log::info;
use std::str::FromStr;
use thiserror::Error;

linera_sdk::contract!(FungibleToken);

impl WithContractAbi for FungibleToken {
    type Abi = fungible::FungibleTokenAbi;
}

#[async_trait]
impl Contract for FungibleToken {
    type Error = Error;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        context: &OperationContext,
        mut state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        // If initial accounts are empty, creator gets 1M tokens to act like a faucet.
        log::info!("Initialisation arg called");
        if state.accounts.is_empty() {
            if let Some(owner) = context.authenticated_signer {
                log::info!("State empty");
                state.accounts.insert(
                    FungibleAccountOwner::User(owner),
                    Amount::from_str("1000000").unwrap(),
                );
            }
        }
        self.initialize_accounts(state).await;
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Transfer {
                owner,
                amount,
                target_account,
            } => {
                Self::check_account_authentication(None, context.authenticated_signer, owner)?;
                self.debit(owner, amount).await?;
                Ok(self
                    .finish_transfer_to_account(amount, target_account)
                    .await)
            }

            Operation::Claim {
                source_account,
                amount,
                target_account,
            } => {
                Self::check_account_authentication(
                    None,
                    context.authenticated_signer,
                    source_account.owner,
                )?;
                self.claim(source_account, amount, target_account).await
            }

            Operation::CreditSomeone {
                target_account,
                caller_chain
            } => {
                Ok(self.check_shit(target_account, caller_chain).await)
            }
        }
    }

    async fn execute_message(
        &mut self,
        context: &MessageContext,
        message: Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        info!("Message Recieved BC");
        match message {
            Message::Credit { owner, amount } => {
                info!("Crediting");
                self.credit(&owner, amount).await;
                info!("Credited");
                Ok(ExecutionResult::default())
            }
            Message::Withdraw {
                owner,
                amount,
                target_account,
            } => {
                Self::check_account_authentication(None, context.authenticated_signer, owner)?;
                self.debit(owner, amount).await?;
                Ok(self
                    .finish_transfer_to_account(amount, target_account)
                    .await)
            }

            Message::FetchBalance {
                owner ,
                caller_chain
            } => {
                
                info!("Fetch balance: {}", system_api::current_chain_id());

                let bal = self.balance(&owner).await;
                let message = Message::Balance { amount: bal };
                Ok(ExecutionResult::default())
            }

            Message::Balance { amount } => {
                info!("Interestingly called");
                Ok(ExecutionResult::default())
            }
        }
    }

    async fn handle_application_call(
        &mut self,
        context: &CalleeContext,
        call: ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        match call {
            ApplicationCall::Balance { owner } => {
                let mut result = ApplicationCallResult::default();
                let balance = self.balance(&owner).await;
                result.value = balance;
                Ok(result)
            }

            ApplicationCall::Transfer {
                owner,
                amount,
                destination,
            } => {
                // Self::check_account_authentication(
                //     context.authenticated_caller_id,
                //     None,
                //     owner,
                // )?;
                info!("Debiting");
                self.debit(owner, amount).await?;
                Ok(self
                    .finish_transfer_to_destination(amount, destination)
                    .await)
            }

            ApplicationCall::Claim {
                source_account,
                amount,
                target_account,
            } => {
                Self::check_account_authentication(
                    None,
                    context.authenticated_signer,
                    source_account.owner,
                )?;
                let execution_result = self.claim(source_account, amount, target_account).await?;
                Ok(ApplicationCallResult {
                    execution_result,
                    ..Default::default()
                })
            }
        }
    }

    async fn handle_session_call(
        &mut self,
        _context: &CalleeContext,
        state: Self::SessionState,
        request: SessionCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<SessionCallResult<Self::Message, Amount, Self::SessionState>, Self::Error> {
        match request {
            SessionCall::Balance => self.handle_session_balance(state),
        }
    }
}

impl FungibleToken {
    async fn get_balance(&self, account: FungibleAccountOwner) -> Amount {
        let bal = self.balance(&account).await;
        info!("{}", bal);
        bal
    }

    // async fn helper(
    //     &self,
    //     account_owner: FungibleAccountOwner,
    //     caller: ChainId,
    // ) -> ExecutionResult<Message> {
    //     info!("Sending a cross chain message");
    //     let message = Message::FetchBalance {
    //         account_owner: account_owner,
    //         caller: caller,
    //     };
    //     ExecutionResult::default().with_message(caller, message)
    // }

    /// Verifies that a transfer is authenticated for this local account.
    fn check_account_authentication(
        authenticated_application_id: Option<ApplicationId>,
        authenticated_signer: Option<Owner>,
        owner: FungibleAccountOwner,
    ) -> Result<(), Error> {
        match owner {
            FungibleAccountOwner::User(address) if authenticated_signer == Some(address) => Ok(()),
            FungibleAccountOwner::Application(id) if authenticated_application_id == Some(id) => {
                Ok(())
            }
            _ => Err(Error::IncorrectAuthentication),
        }
    }

    /// Handles a session balance request sent by an application.
    fn handle_session_balance(
        &self,
        balance: Amount,
    ) -> Result<SessionCallResult<Message, Amount, Amount>, Error> {
        let application_call_result = ApplicationCallResult {
            value: balance,
            execution_result: ExecutionResult::default(),
            create_sessions: vec![],
        };
        let session_call_result = SessionCallResult {
            inner: application_call_result,
            new_state: Some(balance),
        };
        Ok(session_call_result)
    }

    async fn claim(
        &mut self,
        source_account: Account,
        amount: Amount,
        target_account: Account,
    ) -> Result<ExecutionResult<Message>, Error> {
        if source_account.chain_id == system_api::current_chain_id() {
            self.debit(source_account.owner, amount).await?;
            Ok(self
                .finish_transfer_to_account(amount, target_account)
                .await)
        } else {
            let message = Message::Withdraw {
                owner: source_account.owner,
                amount,
                target_account,
            };
            Ok(ExecutionResult::default()
                .with_authenticated_message(source_account.chain_id, message))
        }
    }

    /// Executes the final step of a transfer where the tokens are sent to the destination.
    async fn finish_transfer_to_destination(
        &mut self,
        amount: Amount,
        destination: Destination,
    ) -> ApplicationCallResult<Message, Amount, Amount> {
        let mut result = ApplicationCallResult::default();
        match destination {
            Destination::Account(account) => {
                info!("Destination of type account");
                result.execution_result = self.finish_transfer_to_account(amount, account).await;
            }
            Destination::NewSession => {
                result.create_sessions.push(amount);
            }
        }
        result
    }
    async fn check_shit(&mut self, account: Account, caller_chain: ChainId) -> ExecutionResult<Message> {
        info!("Pray");
        let message = Message::FetchBalance {
            owner: account.owner,
            caller_chain

        };
        ExecutionResult::default().with_message(account.chain_id, message)
    }
    /// Executes the final step of a transfer where the tokens are sent to the destination.
    async fn finish_transfer_to_account(
        &mut self,
        amount: Amount,
        account: Account,
    ) -> ExecutionResult<Message> {
        if account.chain_id == system_api::current_chain_id() {
            info!("Same chain");
            self.credit(&account.owner, amount).await;
            ExecutionResult::default()
        } else {
            info!("Different chain");
            let message = Message::Credit {
                owner: account.owner,
                amount,
            };
            ExecutionResult::default().with_message(account.chain_id, message)
        }
    }
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum Error {
    /// Insufficient balance in source account.
    #[error("Source account does not have sufficient balance for transfer")]
    InsufficientBalance(#[from] state::InsufficientBalanceError),

    /// Insufficient balance in session.
    #[error("Session does not have sufficient balance for transfer")]
    InsufficientSessionBalance,

    /// Requested transfer does not have permission on this account.
    #[error("The requested transfer is not correctly authenticated.")]
    IncorrectAuthentication,

    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),
}
