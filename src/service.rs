#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::NFTtoken;
use async_graphql::{EmptySubscription, Request, Response, Schema};
use async_trait::async_trait;
use linera_sdk::{
    base::WithServiceAbi, graphql::GraphQLMutationRoot, QueryContext, Service, ViewStateStorage,
};
use nft::Operation;
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(NFTtoken);

impl WithServiceAbi for NFTtoken {
    type Abi = nft::NFTabi;
}

#[async_trait]
impl Service for NFTtoken {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;

    async fn query_application(
        self: Arc<Self>,
        _context: &QueryContext,
        request: Request,
    ) -> Result<Response, Self::Error> {
        let schema =
            Schema::build(self.clone(), Operation::mutation_root(), EmptySubscription).finish();
        let response = schema.execute(request).await;
        Ok(response)
    }
}

/// An error that can occur while querying the service.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Query not supported by the application.
    #[error("Queries not supported by application")]
    QueriesNotSupported,

    /// Invalid query argument; could not deserialize request.
    #[error("Invalid query argument; could not deserialize request")]
    InvalidQuery(#[from] serde_json::Error),
    // Add error variants here.
}
