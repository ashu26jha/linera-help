use fungible::FungibleAccountOwner;
use linera_sdk::{
    base::{Amount, ChainId},
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct MarketPlace {
    listing_counter: RegisterView<u64>,
    listings_price: MapView<u64, Amount>,
    listings_token_id: MapView<u64, u64>,
    listing_sold: MapView<u64, bool>,
    listing_owner: MapView<u64, FungibleAccountOwner>,
    listing_chain: MapView<u64, ChainId>,
    lisitng_uri: MapView<u64, String>,
}

impl MarketPlace {
    pub async fn add_listings(
        &mut self,
        price: Amount,
        token_id: u64,
        owner: FungibleAccountOwner,
        chain_id: ChainId,
        token_uri: String,
    ) {
        let curr_count = self.listing_counter.get().clone();
        // Add price
        self.listings_price
            .insert(&curr_count, price)
            .expect("Couldn't insert price");

        // Add Token ID
        self.listings_token_id
            .insert(&curr_count, token_id)
            .expect("Couldn't insert ");

        // Add Boolean
        self.listing_sold
            .insert(&curr_count, false)
            .expect("Couldn't set sold");

        self.listing_owner
            .insert(&curr_count, owner)
            .expect("Can't insert owner");

        self.listing_chain
            .insert(&curr_count, chain_id)
            .expect("Couldn't insert chain ID");

        self.lisitng_uri
            .insert(&curr_count, token_uri)
            .expect("Couldn't insert URI");

        self.listing_counter.set(curr_count + 1);
    }

    pub async fn get_status(&self, listing_id: u64) -> bool {
        self.listing_sold
            .get(&listing_id)
            .await
            .expect("Couldnt fetch status")
            .unwrap()
    }

    pub async fn get_price(&self, listing_id: u64) -> Amount {
        self.listings_price
            .get(&listing_id)
            .await
            .expect("Couldn't fetch price")
            .unwrap()
    }

    pub async fn get_owner(&self, listing_id: u64) -> FungibleAccountOwner {
        self.listing_owner
            .get(&listing_id)
            .await
            .expect("Couldn't get owner")
            .unwrap()
    }

    pub async fn get_chain_id(&self, listing_id: u64) -> ChainId {
        self.listing_chain
            .get(&listing_id)
            .await
            .expect("Couldn't get chain ID")
            .unwrap()
    }

    pub async fn get_token_id(&self, listing_id: u64) -> u64 {
        self.listings_token_id
            .get(&listing_id)
            .await
            .expect("Couldn't get token ID")
            .unwrap()
    }
}
