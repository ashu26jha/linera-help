linera project publish-and-create --json-argument '{ "accounts": {
        "User:993b65733a2e14119dda9191e7a9ab3e56d4ee26974132ce4454b03635154cb8": "10000."
    } }'


linera project publish-and-create --json-argument "null" --json-parameters='"'e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65000000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65020000000000000000000000'"' --required-application-ids=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65000000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65020000000000000000000000





NFT: `linera project publish-and-create --json-argument="null"`

FUNGIBLE: 
```
linera project publish-and-create --json-argument '{ 
    "accounts": {
        "User:0a203bcf959d4e7c85d1be3e4e9296ec05a122f071367d6a616c420add4447d7": "10000."
    } 
}'
```

fungible_app_id=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65000000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65020000000000000000000000

nft_app_id=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65030000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65050000000000000000000000

MARKETPLACE: `linera project publish-and-create --json-argument="null" --json-parameters "{\"fungible_app_id\":\"$fungible_app_id\",\"nft_app_id\":\"$nft_app_id\"}" --required-application-ids $fungible_app_id --required-application-ids $nft_app_id`
