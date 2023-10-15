import { useEffect, useState } from "react";
import Navbar from "./navbar";
import {
    gql,
    useMutation,
    useLazyQuery,
} from "@apollo/client";

function MarketPlace({ chainId, owner }) {
    const [count, setCount] = useState(0);
    const [owner_nft, setOwner] = useState('');
    const [tokenURI, setTokenURI] = useState('');
    const [price, setPrice] = useState(0);
    const [chainID, setchainID] = useState('');

    const listingOwner = gql
    `
        query{
            listingOwner(u64:${count})
        }
    `
    const listingPrice = gql
    `
        query{
            listingsPrice(u64:${count})
        }
    `
    const listingURI = gql
    `
        query{
            lisitngUri(u64:${count})
        }
    `
    const listingChain = gql
    `
        query{
            listingChain(u64:${count})
        }
    `

    let [
        ownerQuery,
        { data: ownerData, called: ownerDataCalled, error: ownerError },
    ] = useLazyQuery(listingOwner);

    if (!ownerDataCalled) {
        void ownerQuery();
    }

    useEffect(()=>{
        if(ownerData!=null){
            console.log(ownerData.listingOwner)
            setOwner(ownerData.listingOwner);
        }
    },[ownerData])

    let [
        priceQuery,
        { data: priceData, called: priceDataCalled, error: priceError },
    ] = useLazyQuery(listingPrice);

    if (!priceDataCalled) {
        void priceQuery();
    }

    useEffect(()=>{
        if(priceData!=null){
            console.log(priceData.listingsPrice)
            setPrice(priceData.listingsPrice);
        }
    },[priceData])

    let [
        uriQuery,
        { data: uriData, called: uriDataCalled, error: uriDataError },
    ] = useLazyQuery(listingURI);

    if (!uriDataCalled) {
        void uriQuery();
    }

    useEffect(()=>{
        if(uriData!=null){
            let url = 'https://ipfs.io/ipfs/'+(uriData.lisitngUri).replace(/^ipfs:\/\//, '');
            console.log(url)
            setTokenURI(url);
        }
    },[uriData])

    let [
        chainQuery,
        { data: chainData, called: chainDataCalled, error: chainError },
    ] = useLazyQuery(listingChain);

    if (!chainDataCalled) {
        void chainQuery();
    }

    useEffect(()=>{
        if(chainData!=null){
            console.log(chainData.listingChain)
            setchainID(chainData.listingChain);
        }
    },[chainData])


    return (
        <div>
            <Navbar/>
            <div className="wrap-market">
                <img src={tokenURI} className="image-display" />
                <div className="price">Listing ID: #{count}</div>
                <div className="owner">Price: {price}00</div>
                <div className="owner">Owner: {owner_nft.substring(5,11)+'...'+owner_nft.substring(owner_nft.length-3)}</div>
                <div className="owner">Chain ID: {chainID.substring(0,6)+'...'+chainID.substring(chainID.length-3)}</div>
            </div>

        </div>
    )
}
export default MarketPlace;
