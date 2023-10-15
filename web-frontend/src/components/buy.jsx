import {
    gql,
    useMutation,
} from "@apollo/client";
import Navbar from "./navbar";
import { useState } from "react";

function Buy({ chainId, owner }) {
    
    const [listing_id, setListingID] = useState(0);
    const [dest_chain_id, set_dest_chain_id] = useState('');
    const [error,setError] = useState('')

    const BUY_NFT = gql
    `
        mutation GetStatus{
            fetchBalance(
            listingId:${listing_id},
            caller: {
                chainId:"${chainId}",
                owner:"User:${owner}"
            }
            chainId:"${dest_chain_id}"
            )
        }
    `;

    const [buy_nft, { loading: paymentLoading }] = useMutation(BUY_NFT, {
        onCompleted: () => {
        },
        onError: (error) => setError("Error: " + error.networkError.result),
    });

    async function handleListNFT(event) {
        event.preventDefault();
        buy_nft({

        }).then(r => console.log('Bought'));
    }
    return (
        <div>
            <Navbar />
            <div className="buy">
                <div>
                <input onChange={((e)=>{setListingID(e.target.value)})} placeholder="Listing ID"/>
                </div>
                <div>
                <input onChange={((e)=>{set_dest_chain_id(e.target.value)})} placeholder="Destination Chain"/>
                </div>
                <button onClick={handleListNFT}>Buy</button>
            </div>
        </div>
    )
}
export default Buy
