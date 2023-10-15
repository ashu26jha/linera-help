NFT: `linera project publish-and-create --json-argument="null"`

FUNGIBLE: 
```
linera project publish-and-create --json-argument '{ 
    "accounts": {
        "User:c09edddf47b9b0538e497d2b2d53861790cc34536756fb5d535320c0bf689f50": "10000."
    } 
}'
```

fungible_app_id=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65000000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65020000000000000000000000
nft_app_id=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65030000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65050000000000000000000000

MARKETPLACE: `linera project publish-and-create --json-argument="null" --json-parameters "{\"fungible_app_id\":\"$fungible_app_id\",\"nft_app_id\":\"$nft_app_id\"}" --required-application-ids $fungible_app_id --required-application-ids $nft_app_id`
