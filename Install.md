NFT: `linera project publish-and-create --json-argument="null"`

FUNGIBLE: 
```
linera project publish-and-create --json-argument '{ 
    "accounts": {
        "User:03b12e330f3dd50c15210edea6296276d92dd62d46ffa437afddfbdbb8f32bb3": "10000."
    } 
}'
```

fungible_app_id=
nft_app_id=

MARKETPLACE: `linera project publish-and-create --json-argument="null" --json-parameters "{\"fungible_app_id\":\"$fungible_app_id\",\"nft_app_id\":\"$nft_app_id\"}" --required-application-ids $fungible_app_id --required-application-ids $nft_app_id`
