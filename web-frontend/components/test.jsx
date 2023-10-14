import { useState } from "react";
import {
  gql,
  useMutation,
  useLazyQuery,
  useSubscription,
} from "@apollo/client";
import tw from "tailwind-styled-components";

const GET_BALANCE = gql`
  query Accounts($owner: AccountOwner) {
    accounts(accountOwner: $owner)
  }
`;

const MAKE_PAYMENT = gql`
  mutation Transfer($owner: AccountOwner, $amount: Amount, $targetAccount: Account) {
    transfer(owner: $owner, amount: $amount, targetAccount: $targetAccount)
  }
`;

const NOTIFICATION_SUBSCRIPTION = gql`
  subscription Notifications($chainId: ID!) {
    notifications(chainId: $chainId)
  }
`;

// Styled components
const Container = tw.div`
  max-w-2xl mx-auto my-8
`;

const Card = tw.div`
  bg-white rounded-lg shadow-md p-6 mb-6
`;

const Label = tw.label`
  block mb-2 text-gray-700 font-bold
`;

const Input = tw.input`
  w-full border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline
`;

const Button = tw.button`
  bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline
`;

const ErrorMessage = tw.div`
  text-red-500 text-sm italic mt-2
`;

// App component
function Test({ chainId, owner }) {
  console.log(chainId)
  console.log(owner)
  const [recipient, setRecipient] = useState("");
  const [targetChain, setTargetChain] = useState("");
  const [amount, setAmount] = useState("");
  const [error, setError] = useState("");
  let [
    balanceQuery,
    { data: balanceData, called: balanceCalled, error: balanceError },
  ] = useLazyQuery(GET_BALANCE, {
    fetchPolicy: "network-only",
    variables: { owner: `User:${owner}` },
  });
  const [makePayment, { loading: paymentLoading }] = useMutation(MAKE_PAYMENT, {
    onError: (error) => setError("Error: " + error.networkError.result),
    onCompleted: () => {
      setRecipient("");
      setAmount("");
    },
  });

  if (!balanceCalled) {
    void balanceQuery();
  }

  useSubscription(NOTIFICATION_SUBSCRIPTION, {
    variables: { chainId: chainId },
    onData: () => balanceQuery(),
  });

  // Event handlers
  const handleRecipientChange = (event) => {
    setRecipient(event.target.value);
  };

  const handleAmountChange = (event) => {
    setAmount(event.target.value);
  };

  const handleTargetChainChange = (event) => {
    setTargetChain(event.target.value);
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    makePayment({
      variables: {
        owner: `User:${owner}`,
        amount,
        targetAccount: {
          chainId: targetChain,
          owner: `User:${recipient}`,
        },
      },
    }).then(r => console.log("payment made: " + r));
  };

  // Render
  return (
    <Test />
  );
}

export default Test;
