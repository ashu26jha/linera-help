import Test from "./components/mint"

function App({ chainId, owner }) {

  return (
    <Test chainId={chainId} owner={owner}/>
  );
}

export default App;
