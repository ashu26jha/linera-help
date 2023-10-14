import { useState } from "react";
import {
  gql,
  useMutation,
  useLazyQuery,
  useSubscription,
} from "@apollo/client";

const TOKEN_QUERY = gql`
  query Hello {
    tokenOwner(u64:0)
  }
`
function Test({ chainId, owner }) {

  let [
    TokenQuery,
    { data: tokenData, called: tokenDataCalled },
  ] = useLazyQuery(TOKEN_QUERY);

  if(!tokenDataCalled){
    void TokenQuery();
    console.log(tokenData)
  }

  function Help(){
    console.log(tokenData)
  }

  // Render
  return (
    <div>
      <button onClick={Help}>
        Test
      </button>
    </div>      
  );
}

export default Test;
