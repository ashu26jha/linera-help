import List from "./components/list";
function App({ chainId, owner }) {

  return (
    <List chainId={chainId} owner={owner}/>
  );
}

export default App;
