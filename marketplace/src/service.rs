#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::MarketPlace;
use async_trait::async_trait;
use linera_sdk::{base::WithServiceAbi, QueryContext, Service, ViewStateStorage};
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(MarketPlace);

impl WithServiceAbi for MarketPlace {
    type Abi = marketplace::MarketPlaceABI;
}

#[async_trait]
impl Service for MarketPlace {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;

    async fn query_application(
        self: Arc<Self>,
        _context: &QueryContext,
        _query: Self::Query,
    ) -> Result<(), Self::Error> {
        Err(ServiceError::QueriesNotSupported)
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
