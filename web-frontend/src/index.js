import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import Test from "./components/mint";
import {
  BrowserRouter,
  Route,
  Routes,
  useParams,
  useSearchParams,
} from "react-router-dom";
import GraphQLProvider from "./GraphQLProvider";

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
  let chain_id = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65"
  const [searchParams] = useSearchParams();
  let app = searchParams.get("app");
  let owner = searchParams.get("owner");
  let port = searchParams.get("port");
  if (app == null) {
    app="e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65000000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65020000000000000000000000"
  }
  if (owner == null) {
    owner="4b10913a8a238f550894ebbbb59ea05c1c26ae3de8fb00e1ed9c8cc08acb2b60"
  }
  if (port == null) {
    port = 8080;
  }
  return (
    <GraphQLProvider chainId={chain_id} applicationId={app} port={port}>
      <Routes>
         <Route path="/" element={<App chainId={chain_id} owner={owner}/>}> </Route>
         <Route path="/mint" element={<Test chainId={chain_id} owner={owner} />}>  </Route>
      </Routes>

    </GraphQLProvider>
  );
}
