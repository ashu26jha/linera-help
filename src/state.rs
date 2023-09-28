use linera_sdk::{
    base::Owner,
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct NFTtoken {
    token_counter: RegisterView<u64>,
    token_owner: MapView<u64, Owner>,
    token_uri: MapView<u64,String>
}

#[allow(dead_code)]

impl NFTtoken {
    pub async fn get_token_owner(&self, token: u64) -> Owner {
        
        self.token_owner
            .get(&token)
            .await
            .expect("Failure in the retrieval")
            .unwrap()
    }

    pub async fn mint_nft(&mut self, token: u64, minter: Owner, token_uri: String) {

        self.token_owner
            .insert(&token, minter)
            .expect("Couldn't insert in Token Owner");

        self.token_uri
            .insert(&token, token_uri)
            .expect("Couldn't insert in Token URI")

    }

    pub async fn transfer_nft(&mut self, token: u64, new_owner: Owner){
        self.token_owner
            .insert(&token, new_owner)
            .expect("Couldn't transfer the NFT")
    }
}
