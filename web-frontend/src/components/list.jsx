import {
    gql,
    useMutation,
} from "@apollo/client";
import { useState } from "react"
import Navbar from "./navbar";

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

    async function handleListNFT(event) {
        event.preventDefault();
        listNFT({

        }).then(r => console.log('Listed'));
    }

    return (
        <div>
            <Navbar />
            <div className="list">
                <input onChange={((e) => { setTokenID(e.target.value) })} placeholder="Token ID" />
                <br/>
                <input onChange={((e) => { setPrice(e.target.value) })} placeholder="Price" />
                <br/>
                <button onClick={handleListNFT}>
                    List NFT
                </button>
            </div>

        </div>
    )
}

export default List
