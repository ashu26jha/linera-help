# Li - NFT

### Description

Introducing NFT-Fi on the Linera network, where we've meticulously crafted NFT contracts with `ERC-721` standards in mind.

Mint, Sell, and Chat

Create unique NFTs with a single prompt, mint the ones you love, and sell them on our marketplace to earn real rewards. Plus, showcase your style with your NFT avatar in your chats.

To be able to implement an NFT marketplace few of the following things had to be done:

Fetching the price, owner, token ID, tokenURI, this was achieved by sending and recieving message. The buyer chain sends a message asking the details of NFT, The sender chain responds with a message. On recieving the message on the Buyer chain, it destructures the message.
It Transfer the funds from its chain (Sending a cross application Transfer to Fungible application, as discussed in the workshop) to the seller chain.
In the similar fashion NFT is transfered, making changes in both seller and buyer chains

### Running 

1. Spine up `linera net up` in a separate terminal
2. Export wallet and storage to other terminals
3. `linera wallet show` Let the owner of main chain be User1
4. Publish and create `fungible` application using and copy its application ID let it fungible_app_id
   ``` bash
   linera project publish-and-create --json-argument '{
   
    "accounts": {
   
        "User1": "10000."
   
    }
    }'
   ```
   
5. Publish and create `nft` application using ```linera project publish-and-create --json-argument="null"```

6. Now publish and create `marketplace` application using
   ```bash
   linera project publish-and-create --json-argument="null" --json-parameters "{\"fungible_app_id\":\"$fungible_app_id\",\"nft_app_id\":\"$nft_app_id\"}" --required-application-ids $fungible_app_id --required-application-ids $nft_app_id
   ```

8. Now to  start frontend, npm start

9. Make sure to have changed the chain ID, owner in `index.js`

10. Also from graphql perform a mutation `requestApplication` to get the application into another chain, and perform cross-chain application
