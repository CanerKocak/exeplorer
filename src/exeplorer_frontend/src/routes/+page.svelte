<script>
  import "../index.scss";
  import { backend } from "$lib/canisters";
  import { onMount } from "svelte";

  let price = 123.45;
  let volume = 67890;
  let recentTransactions = [];
  let topWallets = [];
  let searchQuery = "";
  let filteredTransactions = [];
  let filteredWallets = [];

  onMount(() => {
    fetchMockData();
  });

  function fetchMockData() {
    recentTransactions = [
      {
        id: 1,
        from: "aaaa-bbbb-cccc-dddd",
        to: "eeee-ffff-gggg-hhhh",
        amount: 1000,
      },
      {
        id: 2,
        from: "iiii-jjjj-kkkk-llll",
        to: "mmmm-nnnn-oooo-pppp",
        amount: 2000,
      },
      // Add more mock transactions
    ];

    topWallets = [
      { rank: 1, address: "aaaa-bbbb-cccc-dddd", balance: 10000 },
      { rank: 2, address: "eeee-ffff-gggg-hhhh", balance: 9000 },
      // Add more mock wallets
    ];

    filterData();
  }

  function filterData() {
    const query = searchQuery.toLowerCase();
    filteredTransactions = recentTransactions.filter(
      (tx) =>
        tx.from.toLowerCase().includes(query) ||
        tx.to.toLowerCase().includes(query)
    );
    filteredWallets = topWallets.filter((wallet) =>
      wallet.address.toLowerCase().includes(query)
    );
  }
</script>

<main class="dashboard">
  <div class="grid-item search">
    <input
      type="text"
      placeholder="Search by address..."
      bind:value={searchQuery}
      on:input={filterData}
    />
  </div>
  <div class="grid-item price">
    <h2>Price</h2>
    <p>${price}</p>
  </div>
  <div class="grid-item volume">
    <h2>Volume</h2>
    <p>${volume}</p>
  </div>
  <div class="grid-item transactions">
    <h2>Recent Transactions</h2>
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>From</th>
          <th>To</th>
          <th>Amount</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredTransactions as transaction}
          <tr>
            <td>{transaction.id}</td>
            <td>{transaction.from}</td>
            <td>{transaction.to}</td>
            <td>{transaction.amount}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="grid-item wallets">
    <h2>Top 100 Wallets</h2>
    <table>
      <thead>
        <tr>
          <th>Rank</th>
          <th>Address</th>
          <th>Balance</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredWallets as wallet}
          <tr>
            <td>{wallet.rank}</td>
            <td>{wallet.address}</td>
            <td>{wallet.balance}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</main>

<style>
  .dashboard {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: auto auto 1fr auto;
    gap: 1rem;
    padding: 1rem;
  }

  .grid-item {
    border: 1px solid #ccc;
    padding: 1rem;
    border-radius: 5px;
    background: #f9f9f9;
  }

  .search {
    grid-column: span 2;
    text-align: center;
  }

  .price,
  .volume {
    text-align: center;
  }

  .transactions,
  .wallets {
    grid-column: span 2;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
  }

  th {
    background-color: #f2f2f2;
  }

  input[type="text"] {
    width: 80%;
    padding: 0.5rem;
    margin: 1rem 0;
    border: 1px solid #ccc;
    border-radius: 5px;
  }
</style>
