use async_graphql::{Request, Response};
use linera_sdk::{
    base::{ContractAbi, Owner, ServiceAbi},
    graphql::GraphQLMutationRoot,
};
use serde::{Deserialize, Serialize};

pub struct NFTabi;

impl ContractAbi for NFTabi {
    type Parameters = ();
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for NFTabi {
    type Parameters = ();
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    // Mint an NFT to itself
    // Add a check only mint to itself not for someone else to prevent spamming
    Mint {
        owner: Owner,
        token_id: u64,
        token_uri: String,
    },

    Transfer {
        token_id: u64,
        new_owner: Owner,
    }
}
