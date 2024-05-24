<script>
  import "../index.scss";
  import { onMount } from "svelte";
  import { Chart, registerables } from "chart.js";
  import loadingIcon from "../../static/hourglass.gif";

  Chart.register(...registerables);

  let price = 123.45;
  let totalVolume = 67890;
  let marketCap = 1234567;
  let avgTransactionSize = 1000;
  let activeAddresses = 3456;
  let recentTransactions = [];
  let topWallets = [];
  let searchQuery = "";
  let filteredTransactions = [];
  let filteredWallets = [];
  let exchanges = [
    { name: "ICP Swap", transactions: [], volume: 0, liquidity: 500000 },
    { name: "IC Light", transactions: [], volume: 0, liquidity: 300000 },
    { name: "Sonic", transactions: [], volume: 0, liquidity: 200000 },
    { name: "Helix", transactions: [], volume: 0, liquidity: 400000 },
  ];
  let loading = true;

  onMount(() => {
    setTimeout(() => {
      fetchMockData();
      renderPriceTrend();
      renderVolumeTrend();
      loading = false;
    }, 1000); // Simulate loading time
  });

  function fetchMockData() {
    recentTransactions = [
      {
        id: 1,
        from: "aaaa-bbbb-cccc-dddd-eeee-ffff-gggg-hhhh",
        to: "iiii-jjjj-kkkk-llll-mmmm-nnnn-oooo-pppp",
        amount: 1000,
      },
      {
        id: 2,
        from: "qqqq-rrrr-ssss-tttt-uuuu-vvvv-wwww-xxxx",
        to: "yyyy-zzzz-aaaa-bbbb-cccc-dddd-eeee-ffff",
        amount: 2000,
      },
      // Add more mock transactions
    ];

    topWallets = [
      {
        rank: 1,
        address: "aaaa-bbbb-cccc-dddd-eeee-ffff-gggg-hhhh",
        balance: 10000,
      },
      {
        rank: 2,
        address: "iiii-jjjj-kkkk-llll-mmmm-nnnn-oooo-pppp",
        balance: 9000,
      },
      // Add more mock wallets
    ];

    exchanges.forEach((exchange, index) => {
      exchange.transactions = [
        {
          id: index * 3 + 1,
          from: "aaaa-bbbb-cccc-dddd",
          to: "eeee-ffff-gggg-hhhh",
          amount: 5000,
        },
        {
          id: index * 3 + 2,
          from: "iiii-jjjj-kkkk-llll",
          to: "mmmm-nnnn-oooo-pppp",
          amount: 3000,
        },
        {
          id: index * 3 + 3,
          from: "qqqq-rrrr-ssss-tttt",
          to: "uuuu-vvvv-wwww-xxxx",
          amount: 2000,
        },
      ];
      exchange.volume = exchange.transactions.reduce(
        (acc, tx) => acc + tx.amount,
        0
      );
    });

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

  function truncateAddress(address) {
    return address.length > 10
      ? `${address.slice(0, 6)}...${address.slice(-4)}`
      : address;
  }

  function renderPriceTrend() {
    const ctx = document.getElementById("priceTrendChart").getContext("2d");
    new Chart(ctx, {
      type: "line",
      data: {
        labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
        datasets: [
          {
            label: "Price Trend",
            data: [120, 125, 130, 128, 133, 135],
            backgroundColor: "rgba(255, 0, 255, 0.2)",
            borderColor: "rgba(255, 0, 255, 1)",
            borderWidth: 1,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            beginAtZero: true,
          },
        },
      },
    });
  }

  function renderVolumeTrend() {
    const ctx = document.getElementById("volumeTrendChart").getContext("2d");
    new Chart(ctx, {
      type: "line",
      data: {
        labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
        datasets: [
          {
            label: "Volume Trend",
            data: [60000, 65000, 70000, 68000, 72000, 73000],
            backgroundColor: "rgba(0, 255, 255, 0.2)",
            borderColor: "rgba(0, 255, 255, 1)",
            borderWidth: 1,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            beginAtZero: true,
          },
        },
      },
    });
  }
</script>

<main class="dashboard">
  {#if loading}
    <div class="loading">
      <img src={loadingIcon} alt="Loading..." />
    </div>
  {/if}
  <div class="grid-item search">
    <input
      type="text"
      placeholder="Search by address..."
      bind:value={searchQuery}
      on:input={filterData}
    />
  </div>
  <div class="grid-item price-volume">
    <div class="price">
      <h2>Price</h2>
      <p>${price}</p>
      <div class="chart-container">
        <canvas id="priceTrendChart"></canvas>
      </div>
    </div>
    <div class="volume">
      <h2>Total Volume</h2>
      <p>${totalVolume}</p>
      <div class="chart-container">
        <canvas id="volumeTrendChart"></canvas>
      </div>
    </div>
  </div>
  <div class="grid-item market-cap">
    <h2>Market Cap</h2>
    <p>${marketCap}</p>
  </div>
  <div class="grid-item avg-transaction-size">
    <h2>Avg Transaction Size</h2>
    <p>${avgTransactionSize}</p>
  </div>
  <div class="grid-item active-addresses">
    <h2>Active Addresses</h2>
    <p>${activeAddresses}</p>
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
            <td class="tooltip"
              >{transaction.from}<span class="tooltiptext"
                >{transaction.from}</span
              ></td
            >
            <td class="tooltip"
              >{transaction.to}<span class="tooltiptext">{transaction.to}</span
              ></td
            >
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
            <td class="tooltip"
              >{wallet.address}<span class="tooltiptext">{wallet.address}</span
              ></td
            >
            <td>{wallet.balance}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="grid-item exchanges">
    <h2>Exchanges</h2>
    <div class="exchange-grid">
      {#each exchanges as exchange}
        <div class="exchange">
          <h3>{exchange.name}</h3>
          <p>Volume: {exchange.volume}</p>
          <p>Liquidity: {exchange.liquidity}</p>
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
              {#each exchange.transactions as transaction}
                <tr>
                  <td>{transaction.id}</td>
                  <td class="tooltip"
                    >{transaction.from}<span class="tooltiptext"
                      >{transaction.from}</span
                    ></td
                  >
                  <td class="tooltip"
                    >{transaction.to}<span class="tooltiptext"
                      >{transaction.to}</span
                    ></td
                  >
                  <td>{transaction.amount}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/each}
    </div>
  </div>
</main>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap");

  :global(body) {
    background: linear-gradient(135deg, #20002c 0%, #cbb4d4 100%);
    font-family: "Press Start 2P", cursive;
    color: #fff;
    overflow-x: hidden;
  }

  .dashboard {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    padding: 1rem;
  }

  .grid-item {
    border: 2px solid #ff00ff;
    padding: 1rem;
    border-radius: 10px;
    background: rgba(0, 0, 0, 0.7);
    text-align: center;
  }

  .search {
    grid-column: span 2;
    text-align: center;
  }

  .price-volume {
    display: flex;
    justify-content: space-around;
    grid-column: span 2;
  }

  .price,
  .volume,
  .market-cap,
  .avg-transaction-size,
  .active-addresses {
    font-size: 2rem;
  }

  .transactions,
  .wallets,
  .exchanges {
    grid-column: span 2;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    border: 1px solid #ff00ff;
    padding: 8px;
    text-align: left;
    color: #fff;
  }

  th {
    background-color: #ff00ff;
    color: #000;
  }

  td {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 150px;
  }

  input[type="text"] {
    width: 100%;
    padding: 1rem;
    margin: 1rem 0;
    border: 2px solid #ff00ff;
    border-radius: 5px;
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    font-family: "Press Start 2P", cursive;
  }

  .tooltip {
    position: relative;
    display: inline-block;
  }

  .tooltip .tooltiptext {
    visibility: hidden;
    width: 200px;
    background-color: #ff00ff;
    color: #000;
    text-align: center;
    border-radius: 6px;
    padding: 5px;
    position: absolute;
    z-index: 1;
    bottom: 125%; /* Position the tooltip above the text */
    left: 50%;
    margin-left: -100px;
    opacity: 0;
    transition: opacity 0.3s;
    font-size: 0.75rem;
    word-wrap: break-word;
  }

  .tooltip:hover .tooltiptext {
    visibility: visible;
    opacity: 1;
  }

  .exchange {
    margin-top: 1rem;
    border: 2px solid #ff00ff;
    padding: 1rem;
    border-radius: 10px;
    background: rgba(0, 0, 0, 0.7);
  }

  .exchange-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .exchange h3 {
    color: #ff00ff;
  }

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1000;
  }

  .loading img {
    width: 64px;
    height: 64px;
  }

  .chart-container {
    position: relative;
    height: 200px;
    width: 100%;
  }
</style>
