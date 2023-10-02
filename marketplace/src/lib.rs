use async_graphql::{Request, Response};
use fungible::{FungibleAccountOwner, Account};
use linera_sdk::{
    base::{Amount, ApplicationId, ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
pub struct MarketPlaceABI;
use serde::{Deserialize, Serialize};

impl ContractAbi for MarketPlaceABI {
    type Parameters = ApplicationId<fungible::FungibleTokenAbi>;
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = ();
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
}
