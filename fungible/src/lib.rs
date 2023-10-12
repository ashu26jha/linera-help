use async_graphql::{scalar, InputObject, Request, Response};
use linera_sdk::{
    base::{Amount, ApplicationId, ChainId, ContractAbi, Owner, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeMap, str::FromStr};
#[cfg(all(any(test, feature = "test"), not(target_arch = "wasm32")))]
use {
    async_graphql::InputType,
    futures::{stream, StreamExt},
    linera_sdk::{
        base::BytecodeId,
        test::{ActiveChain, TestValidator},
    },
};

pub struct FungibleTokenAbi;

impl ContractAbi for FungibleTokenAbi {
    type InitializationArgument = InitialState;
    type Parameters = ();
    type ApplicationCall = ApplicationCall;
    type Operation = Operation;
    type Message = Message;
    type SessionCall = SessionCall;
    type Response = Amount;
    type SessionState = Amount;
}

impl ServiceAbi for FungibleTokenAbi {
    type Query = Request;
    type QueryResponse = Response;
    type Parameters = ();
}

/// An operation.
#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    /// A transfer from a (locally owned) account to a (possibly remote) account.
    Transfer {
        owner: FungibleAccountOwner,
        amount: Amount,
        target_account: Account,
    },
    /// Same as transfer but the source account may be remote. Depending on its
    /// configuration (see also #464), the target chain may take time or refuse to process
    /// the message.
    Claim {
        source_account: Account,
        amount: Amount,
        target_account: Account,
    },

    CreditSomeone {
        target_account: Account,
        caller_chain: ChainId
    },
}

/// A message.
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    /// Credit the given account.
    Credit {
        owner: FungibleAccountOwner,
        amount: Amount,
    },

    /// Withdraw from the given account and starts a transfer to the target account.
    Withdraw {
        owner: FungibleAccountOwner,
        amount: Amount,
        target_account: Account,
    },

    FetchBalance {
        owner: FungibleAccountOwner,
        caller_chain: ChainId
    },

    Balance {
        amount: Amount,
    },
}

/// A cross-application call.
#[derive(Debug, Deserialize, Serialize)]
pub enum ApplicationCall {
    /// A request for an account balance.
    Balance { owner: FungibleAccountOwner },
    /// A transfer from an account.
    Transfer {
        owner: FungibleAccountOwner,
        amount: Amount,
        destination: Destination,
    },
    /// Same as transfer but the source account may be remote.
    Claim {
        source_account: Account,
        amount: Amount,
        target_account: Account,
    },
}

/// A cross-application call into a session.
#[derive(Debug, Deserialize, Serialize)]
pub enum SessionCall {
    /// A request for the session's balance.
    Balance,
    // /// A transfer from the session.
    // Transfer {
    //     amount: Amount,
    //     destination: Destination
    // },
}

scalar!(FungibleAccountOwner);

/// An account owner.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FungibleAccountOwner {
    /// An account owned by a user.
    User(Owner),
    /// An account for an application.
    Application(ApplicationId),
}

impl Serialize for FungibleAccountOwner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FungibleAccountOwner::User(owner) => {
                let key = format!("User:{}", owner);
                serializer.serialize_str(&key)
            }
            FungibleAccountOwner::Application(app_id) => {
                let key = format!("Application:{}", app_id);
                serializer.serialize_str(&key)
            }
        }
    }
}

impl<'de> Deserialize<'de> for FungibleAccountOwner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FungibleAccountOwnerVisitor;

        impl<'de> serde::de::Visitor<'de> for FungibleAccountOwnerVisitor {
            type Value = FungibleAccountOwner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing an FungibleAccountOwner")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let parts: Vec<&str> = value.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err(Error::custom("string does not contain colon"));
                }

                match parts[0] {
                    "User" => {
                        let owner = Owner::from_str(parts[1]).map_err(|_| {
                            Error::custom(format!(
                                "failed to parse Owner from string: {}",
                                parts[1]
                            ))
                        })?;
                        Ok(FungibleAccountOwner::User(owner))
                    }
                    "Application" => {
                        let app_id = ApplicationId::from_str(parts[1]).map_err(|_| {
                            Error::custom(format!(
                                "failed to parse ApplicationId from string: {}",
                                parts[1]
                            ))
                        })?;
                        Ok(FungibleAccountOwner::Application(app_id))
                    }
                    _ => Err(Error::unknown_variant(parts[0], &["User", "Application"])),
                }
            }
        }
        deserializer.deserialize_str(FungibleAccountOwnerVisitor)
    }
}

impl<T> From<T> for FungibleAccountOwner
where
    T: Into<Owner>,
{
    fn from(owner: T) -> Self {
        FungibleAccountOwner::User(owner.into())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub accounts: BTreeMap<FungibleAccountOwner, Amount>,
}

/// An account.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, InputObject,
)]
pub struct Account {
    pub chain_id: ChainId,
    pub owner: FungibleAccountOwner,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Destination {
    Account(Account),
    NewSession,
}

scalar!(Destination);

/// A builder type for constructing the initial state of the application.
#[derive(Debug, Default)]
pub struct InitialStateBuilder {
    account_balances: BTreeMap<FungibleAccountOwner, Amount>,
}

impl InitialStateBuilder {
    /// Adds an account to the initial state of the application.
    pub fn with_account(
        mut self,
        account: FungibleAccountOwner,
        balance: impl Into<Amount>,
    ) -> Self {
        self.account_balances.insert(account, balance.into());
        self
    }

    /// Returns the serialized initial state of the application, ready to used as the
    /// initialization argument.
    pub fn build(&self) -> InitialState {
        InitialState {
            accounts: self.account_balances.clone(),
        }
    }
}
