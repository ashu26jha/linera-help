import {
    gql,
    useMutation,
    useLazyQuery,
    useSubscription,
} from "@apollo/client";
import { useEffect, useState } from "react";
import Navbar from "./navbar";

function Collection({ chainId, owner }) {

    const [count, setCount] = useState(0);
    const [tokenURL, setTokenURL] = useState('');
    const [appID, setAppID] = useState('');

    const GET_BALANCE = gql`
        query Owners{
            tokenOwner(u64: ${count})
        }
    `;
    let [
        keyQuery,
        { data: balanceData, called: balanceCalled, error: balanceError },
    ] = useLazyQuery(GET_BALANCE);

    const TOKEN_URI = gql`
        query URI{
            tokenUri(u64: ${count})
        }
    `

    let [
        URIquery,
        { data: URIdata, called: URIcalled, error: URIerror },
    ] = useLazyQuery(TOKEN_URI);

    if (!balanceCalled) {
        void keyQuery();
    }

    useEffect(() => {
        if (balanceData != null) {
            let owner_nft = (balanceData.tokenOwner).replace(/^User:/, '');
            if (owner_nft == owner) {
                void URIquery();
            }
        }

    }, [balanceData])

    useEffect(() => {
        if (URIdata != null) {
            const tokenURI = URIdata.tokenUri.replace(/^ipfs:\/\//, '');
            const URL = `https://ipfs.io/ipfs/${tokenURI}`;
            setTokenURL(URL)
        }
    }, [URIdata])


    const APPROVE_NFT = gql
        `
  mutation Approve{
    approve(
      tokenId: ${count},
      approvedFor: "Application:${appID}"
    )
  }
  `;

    const [approveNFT, { loading, approvingNFT }] = useMutation(APPROVE_NFT, {
        onCompleted: () => {
        },
    })

    const handleApprove = (event) => {
        event.preventDefault();
        approveNFT({

        }).then(r => console.log('Approved'));
    }

    return (
        <div>
            <Navbar />
                <img src={tokenURL} className="image-display" />
            <div className="approve">
                <h1>Token ID : {count}</h1>
                <h1>
                Application ID: <input onChange={((e) => { setAppID(e.target.value) })} />
                </h1> 
                <br />
                <button onClick={handleApprove} className="approve-btn">
                    Approve
                </button>
            </div>

        </div>
    )
}
export default Collection
