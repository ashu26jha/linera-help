import {
    gql,
    useMutation,
    useLazyQuery,
    useSubscription,
} from "@apollo/client";
import { useState } from "react"

function List({ chainId, owner }) {
    const [tokenId, setTokenID] = useState(0);
    const [price, setPrice] = useState("");
    const [error, setError] = useState("")

    const LIST_NFT = gql
        `
    mutation List {
        list(
          tokenId:${tokenId},
          price:"${price}",
          owner:"User:${owner}",
          chainId:"${chainId}"
        )
    }
    `

    const [listNFT, { loading: paymentLoading }] = useMutation(LIST_NFT, {
        onCompleted: () => {
        },
        onError: (error) => setError("Error: " + error.networkError.result),
    });

    async function handleListNFT (event) {
        event.preventDefault();
        listNFT({

        }).then(r => console.log('Listed'));
    }

    return (
        <div>
            Bhe
            <input onChange={((e) => { setPrice(e.target.value) })} />
            <button onClick={handleListNFT}>
                List NFT
            </button>
        </div>
    )
}

export default List
