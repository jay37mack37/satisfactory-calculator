<script>
  let selectedItem = "Rotor";
  let desiredRate = 10;
  let result = null;
  let loading = false;
  let error = "";

  const items = [
    "Iron Ore",
    "Copper Ore",
    "Iron Ingot",
    "Copper Ingot",
    "Iron Plate",
    "Iron Rod",
    "Copper Wire",
    "Rotor",
    "Reinforced Iron Plate",
    "Modular Frame",
  ];

  async function calculate() {
    loading = true;
    error = "";
    result = null;

    try {
      const res = await fetch("http://localhost:3000/api/calculate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ item: selectedItem, rate: desiredRate }),
      });

      if (!res.ok) throw new Error(`Server error: ${res.statusText}`);
      result = await res.json();
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }
</script>

<main class="container">
  <header>
    <h1>⚡ Satisfactory Calculator</h1>
    <p class="subtitle">
      Production chain planner — pick an item, set the rate, get the full
      breakdown
    </p>
  </header>

  <section class="input-panel">
    <div class="field">
      <label for="item-select">Target Item</label>
      <select id="item-select" bind:value={selectedItem}>
        {#each items as item}
          <option value={item}>{item}</option>
        {/each}
      </select>
    </div>

    <div class="field">
      <label for="rate-input">Rate (items/min)</label>
      <input
        id="rate-input"
        type="number"
        min="0.1"
        step="0.1"
        bind:value={desiredRate}
      />
    </div>

    <button class="calc-btn" on:click={calculate} disabled={loading}>
      {loading ? "Calculating…" : "Calculate"}
    </button>
  </section>

  {#if error}
    <div class="error-banner">❌ {error}</div>
  {/if}

  {#if result}
    <section class="results">
      <h2>
        Production Tree for {result.target_item} @ {result.target_rate}/min
      </h2>

      <div class="summary-cards">
        <div class="card">
          <span class="card-label">Total Power</span>
          <span class="card-value">{result.total_power.toFixed(1)} MW</span>
        </div>
        <div class="card">
          <span class="card-label">Production Steps</span>
          <span class="card-value">{result.nodes.length}</span>
        </div>
        <div class="card">
          <span class="card-label">Raw Resources</span>
          <span class="card-value"
            >{Object.keys(result.raw_resources).length}</span
          >
        </div>
      </div>

      <h3>All Production Nodes</h3>
      <table>
        <thead>
          <tr>
            <th>Item</th>
            <th>Rate (/min)</th>
            <th>Machine</th>
            <th>Machines</th>
            <th>Power (MW)</th>
          </tr>
        </thead>
        <tbody>
          {#each result.nodes as node}
            <tr>
              <td>{node.item}</td>
              <td>{node.rate.toFixed(2)}</td>
              <td>{node.machine_name}</td>
              <td>{node.machines_needed.toFixed(2)}</td>
              <td>{node.power_cost.toFixed(2)}</td>
            </tr>
          {/each}
        </tbody>
      </table>

      <h3>Raw Resources Needed</h3>
      <div class="raw-grid">
        {#each Object.entries(result.raw_resources) as [resource, rate]}
          <div class="raw-card">
            <span class="raw-name">{resource}</span>
            <span class="raw-rate">{rate.toFixed(2)}/min</span>
          </div>
        {/each}
      </div>
    </section>
  {/if}
</main>

<style>
  .container {
    max-width: 960px;
    margin: 0 auto;
    padding: 40px 24px;
  }

  header {
    text-align: center;
    margin-bottom: 40px;
  }

  h1 {
    font-size: 2.5rem;
    color: var(--accent);
    margin-bottom: 8px;
  }

  .subtitle {
    color: var(--text-secondary);
    font-size: 1.1rem;
  }

  .input-panel {
    display: flex;
    gap: 16px;
    align-items: flex-end;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 24px;
    margin-bottom: 32px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
  }

  label {
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-weight: 500;
  }

  select,
  input {
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 10px 14px;
    color: var(--text-primary);
    font-size: 1rem;
    font-family: var(--font-sans);
    outline: none;
    transition: border-color 0.2s;
  }

  select:focus,
  input:focus {
    border-color: var(--accent);
  }

  .calc-btn {
    background: var(--accent);
    color: #000;
    border: none;
    border-radius: 8px;
    padding: 10px 28px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition:
      background 0.2s,
      transform 0.1s;
    white-space: nowrap;
  }

  .calc-btn:hover:not(:disabled) {
    background: var(--accent-hover);
    transform: translateY(-1px);
  }

  .calc-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .error-banner {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid var(--error);
    border-radius: 8px;
    padding: 12px 16px;
    color: var(--error);
    margin-bottom: 24px;
  }

  .results {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  h2 {
    font-size: 1.5rem;
    margin-bottom: 20px;
    color: var(--text-primary);
  }

  h3 {
    font-size: 1.15rem;
    margin: 28px 0 14px;
    color: var(--text-secondary);
  }

  .summary-cards {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    margin-bottom: 28px;
  }

  .card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 20px;
    text-align: center;
  }

  .card-label {
    display: block;
    font-size: 0.8rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 6px;
  }

  .card-value {
    display: block;
    font-size: 1.6rem;
    font-weight: 700;
    color: var(--accent);
    font-family: var(--font-mono);
  }

  table {
    width: 100%;
    border-collapse: collapse;
    background: var(--bg-secondary);
    border-radius: 10px;
    overflow: hidden;
    border: 1px solid var(--border);
  }

  thead {
    background: var(--bg-tertiary);
  }

  th {
    padding: 12px 16px;
    text-align: left;
    font-size: 0.8rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 1px solid var(--border);
  }

  td {
    padding: 12px 16px;
    font-size: 0.95rem;
    border-bottom: 1px solid var(--border);
    font-family: var(--font-mono);
    font-size: 0.9rem;
  }

  tr:last-child td {
    border-bottom: none;
  }

  tr:hover td {
    background: var(--bg-tertiary);
  }

  .raw-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 12px;
  }

  .raw-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 14px 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .raw-name {
    font-weight: 500;
  }

  .raw-rate {
    font-family: var(--font-mono);
    color: var(--accent);
    font-size: 0.9rem;
  }
</style>
