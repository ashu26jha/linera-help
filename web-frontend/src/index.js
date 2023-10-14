import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import {
  BrowserRouter,
  Route,
  Routes,
  useParams,
  useSearchParams,
} from "react-router-dom";
import GraphQLProvider from "./GraphQLProvider";
import Mint from "./components/mint";
import List from "./components/list";
import Home from "./components/home";
import Collection from "./components/collection";

const root = ReactDOM.createRoot(document.getElementById("root"));

root.render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path=":id" element={<GraphQLApp />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);

function GraphQLApp() {
  const { id } = useParams();
  console.log(id)

  let chain_id = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65"
  const [searchParams] = useSearchParams();
  let app = searchParams.get("app");
  let owner = searchParams.get("owner");
  let port = searchParams.get("port");
  if (app == null) {
    app="e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65000000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65020000000000000000000000"
  }
  if (owner == null) {
    owner="5fa369830519fd6b38e4001cbe3c8dd6c56a19b1540f460f751ffffaae110bdf"
  }
  if (port == null) {
    port = 8080;
  }
  if (id === 'mint') {
    return (
      <GraphQLProvider chainId={chain_id} applicationId={app} port={port}>
        <Routes>
          <Route path="/" element={<Mint chainId={chain_id} owner={owner} />} />
        </Routes>
      </GraphQLProvider>
    );
  } else if (id === 'list') {
    return (
      <GraphQLProvider chainId={chain_id} applicationId={app} port={port}>
        <Routes>
          <Route path="/" element={<List chainId={chain_id} owner={owner} />} />
        </Routes>
      </GraphQLProvider>
    );
  } else if(id==="home"){
    return (
        <Routes>
          <Route path="/" element={<Home/>} />
        </Routes>
    );
  } else if (id==="collections"){
    return (
      <GraphQLProvider chainId={chain_id} applicationId={app} port={port}>
        <Routes>
          <Route path="/" element={<Collection chainId={chain_id} owner={owner} />} />
        </Routes>
      </GraphQLProvider>
    );
  }
  else {
    return <div>Unknown id: {id}</div>;
  }
}
