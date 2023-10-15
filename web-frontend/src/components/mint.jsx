import { useEffect, useState } from "react";
import {
  gql,
  useMutation,
} from "@apollo/client";
import axios from 'axios';
import Navbar from "./navbar";

function Mint({ chainId, owner }) {
  console.log("SSS")

  const [inputtext, setInputtext] = useState("")
  const [imageurl, setImageUrl] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [hash, setHash] = useState("");

  const [tokenUri, setTokenUri] = useState('ipfs://demo');
  const [tokenId, setTokenID] = useState(0);
  const [error, setError] = useState('');

  const [approveId, setApproveId] = useState('');

  const JWT = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI0MjQyZTQzYi0zODNiLTRhYjUtYWE1NC04YTc1MzIzYTY4NDQiLCJlbWFpbCI6ImFzaHV0b3NoMjZqaGFAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsInBpbl9wb2xpY3kiOnsicmVnaW9ucyI6W3siaWQiOiJGUkExIiwiZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjF9LHsiaWQiOiJOWUMxIiwiZGVzaXJlZFJlcGxpY2F0aW9uQ291bnQiOjF9XSwidmVyc2lvbiI6MX0sIm1mYV9lbmFibGVkIjpmYWxzZSwic3RhdHVzIjoiQUNUSVZFIn0sImF1dGhlbnRpY2F0aW9uVHlwZSI6InNjb3BlZEtleSIsInNjb3BlZEtleUtleSI6ImI2NzI4ZDNmNWRjNjhjMzRhNWY4Iiwic2NvcGVkS2V5U2VjcmV0IjoiZGM3OTJkYjZhNzVkOTI5Y2MyNDllOGZkZDE2MGFhZDI3OGQwMmI1MmJmY2Y2OTQ1NTM4NDM4MjJkMjBiOTQwOSIsImlhdCI6MTY5MjgwNzQ2M30.c4mAp57G4DXIOvuMnYCtheJl6oO2MXMqJse49KSrXYo';
  const API_TOKEN = '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'

  const handleInputChange = (event) => {
    setInputtext(event.target.value);
  }

  const MINT_NFT = gql`
    mutation Mint{
      mint(
          owner:"User:${owner}",
          tokenId:${tokenId},
          tokenUri:"${tokenUri}"
      )
    }
  `;

  const [mintNFT, { loading: paymentLoading }] = useMutation(MINT_NFT, {
    onCompleted: () => {
      console.log("Completed")
    },
    onError: (error) => setError("Error: " + error.networkError.result),
  });

  const handleSubmit = (event) => {
    event.preventDefault();
    mintNFT({

    }).then(r => console.log('Minted'));
  }

  async function hello() {
    setIsLoading(true);
    const input = inputtext;
    console.log("Running AI");
    const response = await fetch(
      "https://api-inference.huggingface.co/models/prompthero/openjourney",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${API_TOKEN}`,
        },
        body: JSON.stringify({ inputs: input }),
      }
    );

    if (!response.ok) {
      throw new Error("Failed to generate image");
    }
    const blobData = await response.blob();

    console.log("Putting on IPFS");

    const formData = new FormData();
    formData.append('file', blobData);
    const pinataMetadata = JSON.stringify({
      name: 'IMAGE',
    });
    formData.append('pinataMetadata', pinataMetadata);
    const pinataOptions = JSON.stringify({
      cidVersion: 0,
    })
    formData.append('pinataOptions', pinataOptions);

    const res = await axios.post("https://api.pinata.cloud/pinning/pinFileToIPFS", formData, {
      maxBodyLength: "Infinity",
      headers: {
        'Content-Type': `multipart/form-data; boundary=${formData._boundary}`,
        'Authorization': `Bearer ${JWT}`
      }
    });
    const helloIMG = `https://ipfs.io/ipfs/${res.data.IpfsHash}`;
    console.log('IPFS: ', helloIMG);
    setTokenUri(`ipfs://${res.data.IpfsHash}`)
    setHash(res.data.IpfsHash);
    setImageUrl(helloIMG);
    setIsLoading(false);
  }

  // Render
  return (
    <div>
      <Navbar />
      <div className="mint-wrapper">
        <input onChange={handleInputChange} placeholder="AI prompt" autoComplete="off" />
        <br />
        <button onClick={hello}>
          Generate
        </button>

        {isLoading && (<img width={100} height={100} src="spinner.gif" className="spinner" />)}
        <img src={imageurl} className="image-display" />
        {imageurl !== "" && (
          <button onClick={handleSubmit}>
            Mint
          </button>
        )}
        <div className="banner">
        <div className="banner-text">
          Write a prompt
        </div>
        <div className="banner-text">
          &
        </div>
        <div className="banner-text">
          Mint the NFT
        </div>
      </div>
        
      </div>
      
    </div>
  );
}

export default Mint;
