use async_graphql::{Request, Response};
use fungible::{Account, FungibleAccountOwner};
use linera_sdk::{
    base::{Amount, ApplicationId, ChainId, ContractAbi, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
pub struct MarketPlaceABI;
use serde::{Deserialize, Serialize};

impl ContractAbi for MarketPlaceABI {
    type Parameters = MarketPlaceParameters;
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for MarketPlaceABI {
    type Parameters = MarketPlaceParameters;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketPlaceParameters {
    pub fungible_app_id: ApplicationId<fungible::FungibleTokenAbi>,
    pub nft_app_id: ApplicationId<nft::NFTabi>,
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    List {
        token_id: u64,
        price: Amount,
        owner: FungibleAccountOwner,
        chain_id: ChainId,
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
    FetchBalance {
        listing_id: u64,
        caller: Account,
    },
    Price {
        caller: Account,
        listing_id: u64,
        owner: Account,
        price: Amount,
    },
}
