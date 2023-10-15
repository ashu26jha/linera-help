NFT: `linera project publish-and-create --json-argument="null"`

FUNGIBLE: 
```
linera project publish-and-create --json-argument '{ 
    "accounts": {
        "User:8b6b11102aea5bb5534edd49fd4f2913783ecf18d24c4df4e351e5c934be7562": "10000."
    } 
}'
```

fungible_app_id=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65000000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65020000000000000000000000
nft_app_id=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65030000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65050000000000000000000000

MARKETPLACE: `linera project publish-and-create --json-argument="null" --json-parameters "{\"fungible_app_id\":\"$fungible_app_id\",\"nft_app_id\":\"$nft_app_id\"}" --required-application-ids $fungible_app_id --required-application-ids $nft_app_id`
