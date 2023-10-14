export default function Login () {

    const [inputtext, setInputtext] = useState("")
	const [imageurl, setImageUrl] = useState("");
	const [isLoading, setIsLoading] = useState(false);
	const [hash, setHash] = useState("");

	const handleInputChange = (event) => {
		setInputtext(event.target.value);
	}
    
    return (
        <div>
            
        </div>
    )
}
