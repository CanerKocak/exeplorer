<script>
  import "../index.scss";
  import { onMount } from "svelte";
  import { Chart, registerables } from "chart.js";
  import loadingIcon from "../../static/hourglass.gif";
  import { backend } from "$lib/canisters";

  Chart.register(...registerables);

  let tokenInfo = {
    name: "",
    symbol: "",
    totalSupply: 0,
    decimals: 0,
    fee: 0,
    minBurnAmount: 0,
  };
  let transactions = [];
  let loading = true;

  onMount(async () => {
    await fetchTokenInfo();
    await fetchTransactions();
    loading = false;
  });

  async function fetchTokenInfo() {
    try {
      const { name, symbol, totalSupply, decimals, fee, minBurnAmount } =
        await backend.get_token_info();
      tokenInfo = {
        name,
        symbol,
        totalSupply,
        decimals,
        fee,
        minBurnAmount,
      };
    } catch (error) {
      console.error("Failed to fetch token info:", error);
    }
  }

  async function fetchTransactions() {
    try {
      const { transactions: txs } = await backend.get_transactions(
        BigInt(0),
        BigInt(10)
      );
      transactions = txs;
    } catch (error) {
      console.error("Failed to fetch transactions:", error);
    }
  }

  function truncateAddress(address) {
    return address.length > 10
      ? `${address.slice(0, 6)}...${address.slice(-4)}`
      : address;
  }

  function formatTimestamp(timestamp) {
    const date = new Date(Number(timestamp) / 1000000);
    return date.toLocaleString();
  }
</script>

<main class="dashboard">
  {#if loading}
    <div class="loading">
      <img src={loadingIcon} alt="Loading..." />
    </div>
  {/if}
  <div class="grid-item token-info">
    <h2>Token Information</h2>
    <p><strong>Name:</strong> {tokenInfo.name}</p>
    <p><strong>Symbol:</strong> {tokenInfo.symbol}</p>
    <p><strong>Total Supply:</strong> {tokenInfo.totalSupply.toString()}</p>
    <p><strong>Decimals:</strong> {tokenInfo.decimals}</p>
    <p><strong>Fee:</strong> {tokenInfo.fee.toString()}</p>
    <p>
      <strong>Min Burn Amount:</strong>
      {tokenInfo.minBurnAmount.toString()}
    </p>
  </div>
  <div class="grid-item transactions">
    <h2>Recent Transactions</h2>
    <table>
      <thead>
        <tr>
          <th>Type</th>
          <th>From</th>
          <th>To</th>
          <th>Amount</th>
          <th>Timestamp</th>
        </tr>
      </thead>
      <tbody>
        {#each transactions as transaction}
          <tr>
            <td>{transaction.kind}</td>
            <td class="tooltip">
              {#if transaction.transfer && transaction.transfer.from && transaction.transfer.from.owner}
                {truncateAddress(transaction.transfer.from.owner.toString())}
                <span class="tooltiptext"
                  >{transaction.transfer.from.owner.toString()}</span
                >
              {:else if transaction.burn && transaction.burn.from && transaction.burn.from.owner}
                {truncateAddress(transaction.burn.from.owner.toString())}
                <span class="tooltiptext"
                  >{transaction.burn.from.owner.toString()}</span
                >
              {:else if transaction.mint}
                Minted
              {/if}
            </td>
            <td class="tooltip">
              {#if transaction.transfer && transaction.transfer.to && transaction.transfer.to.owner}
                {truncateAddress(transaction.transfer.to.owner.toString())}
                <span class="tooltiptext"
                  >{transaction.transfer.to.owner.toString()}</span
                >
              {:else if transaction.burn}
                Burned
              {:else if transaction.mint && transaction.mint.to && transaction.mint.to.owner}
                {truncateAddress(transaction.mint.to.owner.toString())}
                <span class="tooltiptext"
                  >{transaction.mint.to.owner.toString()}</span
                >
              {/if}
            </td>
            <td>
              {#if transaction.transfer && transaction.transfer.amount}
                {transaction.transfer.amount.toString()}
              {:else if transaction.burn && transaction.burn.amount}
                {transaction.burn.amount.toString()}
              {:else if transaction.mint && transaction.mint.amount}
                {transaction.mint.amount.toString()}
              {/if}
            </td>
            <td>{formatTimestamp(transaction.timestamp)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</main>

<style>
  body {
    font-family: "Press Start 2P", cursive;
    background: linear-gradient(45deg, #f06, #f93);
    color: #fff;
    margin: 0;
    padding: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }

  .dashboard {
    background-color: #1a1a1a;
    border: 5px solid #ff00ff;
    padding: 20px;
    width: 90%;
    max-width: 1200px;
    border-radius: 15px;
    box-shadow: 0 0 20px rgba(0, 0, 0, 0.5);
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  .grid-item {
    background-color: #333;
    padding: 20px;
    border-radius: 10px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
    position: relative;
  }

  .loading img {
    width: 100px;
    height: 100px;
  }

  h2 {
    color: #ff00ff;
    text-shadow: 2px 2px #000;
    margin-bottom: 20px;
  }

  p,
  table {
    font-size: 14px;
    line-height: 1.6;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    padding: 10px;
    border: 1px solid #444;
    text-align: left;
  }

  th {
    background-color: #555;
    color: #ff00ff;
    text-shadow: 1px 1px #000;
  }

  td {
    background-color: #222;
    color: #fff;
  }

  .tooltip {
    position: relative;
    display: inline-block;
  }

  .tooltip .tooltiptext {
    visibility: hidden;
    width: 120px;
    background-color: #333;
    color: #fff;
    text-align: center;
    border-radius: 6px;
    padding: 5px;
    position: absolute;
    z-index: 1;
    bottom: 125%;
    left: 50%;
    margin-left: -60px;
    opacity: 0;
    transition: opacity 0.3s;
  }

  .tooltip:hover .tooltiptext {
    visibility: visible;
    opacity: 1;
  }

  @media (max-width: 768px) {
    .dashboard {
      grid-template-columns: 1fr;
    }
  }
</style>
