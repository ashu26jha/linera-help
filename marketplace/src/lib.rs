use async_graphql::{Request, Response};
use fungible::{Account, FungibleAccountOwner};
use linera_sdk::{
    base::{Amount, ApplicationId, ChainId, ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
pub struct MarketPlaceABI;
use serde::{Deserialize, Serialize};

impl ContractAbi for MarketPlaceABI {
    type Parameters = ApplicationId<fungible::FungibleTokenAbi>;
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for MarketPlaceABI {
    type Parameters = ApplicationId<fungible::FungibleTokenAbi>;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    List {
        token_id: u64,
        price: Amount,
    },

    Buy {
        owner: FungibleAccountOwner,
        amount: Amount,
        destination: Account,
    },

    FetchBalance {
        listing_id: u64,
        caller: Account,
        chain_id: ChainId,
    },
}
#[derive(Debug, Deserialize, Serialize)]

pub enum Message {
    FetchBalance { listing_id: u64, caller: Account },
}
