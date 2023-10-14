const Home = () => {
    const [inputtext, setInputtext] = useState("")
    const [imageurl, setImageUrl] = useState("");
    const [isLoading, setIsLoading] = useState(false);
    const [hash, setHash] = useState("");

    async function hello() {
        setIsLoading(true);
        const input = inputtext;
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
        setHash(res.data.IpfsHash);
        setImageUrl(helloIMG);
        console.log(res.data.IpfsHash);
    }

    return (
        <div>
            Hello
        </div>
    )
}
export default Home;
