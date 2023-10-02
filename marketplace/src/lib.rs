use async_graphql::{Request, Response};
use fungible::Destination;
use linera_sdk::{
    base::{Amount, ApplicationId, ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
pub struct MarketPlaceABI;
use nft::Account;
use serde::{Deserialize, Serialize};

impl ContractAbi for MarketPlaceABI {
    type Parameters = (
        ApplicationId<nft::NFTabi>,
        ApplicationId<fungible::FungibleTokenAbi>,
    );
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for MarketPlaceABI {
    type Parameters = (
        ApplicationId<nft::NFTabi>,
        ApplicationId<fungible::FungibleTokenAbi>,
    );
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
        list_id: u64,
        buyer: Account,
        destination: Destination,
    },
}
