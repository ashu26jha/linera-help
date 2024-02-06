Make sure to have NFT the following applications deployed as these applications are required to run the marketplace. Use the following commands. Application IDs are deterministic too. Make sure of the following things: 

1. Running a local Linera server with `linera net up`
2. You have exported the wallet variables

NFT: 
```bash
linera project publish-and-create --json-argument="null"
```

FUNGIBLE: 
``` bash
linera project publish-and-create --json-argument '{ 
    "accounts": {
        "User:$USER1": "10000."
    } 
}'
```

MARKETPLACE: 
``` bash
linera project publish-and-create --json-argument="null" --json-parameters "{\"fungible_app_id\":\"$fungible_app_id\",\"nft_app_id\":\"$nft_app_id\"}" --required-application-ids $fungible_app_id --required-application-ids $nft_app_id
```
