import Test from "./components/test"

function App({ chainId, owner }) {

  return (
    <Test chainId={chainId} owner={owner}/>
  );
}

export default App;
