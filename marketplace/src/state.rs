use linera_sdk::{
    base::Amount,
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
}

impl MarketPlace {
    pub async fn add_listings(&mut self, price: Amount, token_id: u64) {
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

        self.listing_counter.set(curr_count + 1);
    }
}
