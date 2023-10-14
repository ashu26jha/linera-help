NFT: `linera project publish-and-create --json-argument="null"`

FUNGIBLE: 
```
linera project publish-and-create --json-argument '{ 
    "accounts": {
        "User:8ccdcb61d098b140cb91035d5d592b179b7c91b82b348a8404c6c602d4049d06": "10000."
    } 
}'
```

fungible_app_id=
nft_app_id=

MARKETPLACE: `linera project publish-and-create --json-argument="null" --json-parameters "{\"fungible_app_id\":\"$fungible_app_id\",\"nft_app_id\":\"$nft_app_id\"}" --required-application-ids $fungible_app_id --required-application-ids $nft_app_id`
