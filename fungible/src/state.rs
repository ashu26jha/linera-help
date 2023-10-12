// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use fungible::{FungibleAccountOwner, InitialState};
use linera_sdk::{
    base::Amount,
    views::{MapView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use log::info;
use thiserror::Error;

/// The application state.
#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct FungibleToken {
    accounts: MapView<FungibleAccountOwner, Amount>,
}

#[allow(dead_code)]
impl FungibleToken {
    /// Initializes the application state with some accounts with initial balances.
    pub(crate) async fn initialize_accounts(&mut self, state: InitialState) {
        log::info!("initialize_accounts called");
        for (k, v) in state.accounts {
            log::info!("INTIALISING AGAIN");
            log::info!("Amount: {}", v);
            self.accounts
                .insert(&k, v)
                .expect("Error in insert statement");
        }
    }

    /// Obtains the balance for an `account`.
    pub(crate) async fn balance(&self, account: &FungibleAccountOwner) -> Amount {
        info!("Account, {:?}", account);
        self.accounts
            .get(account)
            .await
            .expect("Failure in the retrieval")
            .unwrap_or_default()
    }

    /// Credits an `account` with the provided `amount`.
    pub(crate) async fn credit(&mut self, account: &FungibleAccountOwner, amount: Amount) {
        let mut balance = self.balance(&account).await;
        info!("Account, {:?}", account);
        info!("Balance before {}", balance);
        balance.saturating_add_assign(amount);
        info!("Balance after {}", balance);
        self.accounts
            .insert(&account, balance)
            .expect("Failed insert statement");
    }

    /// Tries to debit the requested `amount` from an `account`.
    pub(crate) async fn debit(
        &mut self,
        account: FungibleAccountOwner,
        amount: Amount,
    ) -> Result<(), InsufficientBalanceError> {
        let mut balance = self.balance(&account).await;
        balance
            .try_sub_assign(amount)
            .map_err(|_| InsufficientBalanceError)?;
        self.accounts
            .insert(&account, balance)
            .expect("Failed insertion operation");
        Ok(())
    }
}

/// Attempts to debit from an account with insufficient funds.
#[derive(Clone, Copy, Debug, Error)]
#[error("Insufficient balance for transfer")]
pub struct InsufficientBalanceError;
