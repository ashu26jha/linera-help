use linera_sdk::views::{MapView, RegisterView, ViewStorageContext};
use linera_views::views::{GraphQLView, RootView};
use nft::AccountOwner;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct NFTtoken {
    token_counter: RegisterView<u64>,
    token_owner: MapView<u64, AccountOwner>,
    token_uri: MapView<u64, String>,
    token_approval: MapView<u64, AccountOwner>,
}

#[allow(dead_code)]

impl NFTtoken {
    pub async fn get_token_owner(&mut self, token: u64) -> AccountOwner {
        self.token_owner
            .get(&token)
            .await
            .expect("Couldn't retrieve")
            .unwrap()
    }

    pub async fn mint_nft(&mut self, token: u64, minter: AccountOwner, token_uri: String) {
        // Increaments the token_id
        let a = self.token_counter.get();
        self.token_counter.set(*a + 1);

        self.token_owner
            .insert(&token, minter)
            .expect("Couldn't insert in Token Owner");

        self.token_uri
            .insert(&token, token_uri)
            .expect("Couldn't insert in Token URI")
    }

    pub async fn transfer_nft(&mut self, token: u64, new_owner: AccountOwner) {
        // Transfer the NFT
        self.token_owner
            .insert(&token, new_owner)
            .expect("Couldn't transfer the NFT")
    }

    pub async fn approve_nft(&mut self, token: u64, to: AccountOwner) {
        // Approve the `to` to carry out the transaction
        self.token_approval
            .insert(&token, to)
            .expect("Couldn't insert in approve")
    }

    pub async fn get_approvals(&mut self, token: u64) -> AccountOwner {
        let a = self.token_approval.get(&token).await.expect("Cant get");

        // Actually it panics when there is no approver
        if a != None {
            return self
                .token_approval
                .get(&token)
                .await
                .expect("Cant get")
                .unwrap();
        }

        // Returning owner will also work as `if` block will fail because wrong approver wouldn't be an owner!
        self.token_owner
            .get(&token)
            .await
            .expect("Failure in the retrieval")
            .unwrap()
    }

    pub async fn burn_nft(&mut self, token: u64) {
        self.token_owner
            .remove(&token)
            .expect("Couldn't remove owner from map");

        self.token_uri
            .remove(&token)
            .expect("Couldn't remove token URI")
    }

    pub async fn handle_message(&mut self, token: u64, owner: AccountOwner) {
        self.token_owner.insert(&token, owner).expect("S")
    }
}
