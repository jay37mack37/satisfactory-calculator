<script>
  let selectedItem = $state("Rotor");
  let desiredRate = $state(10);
  let result = $state(null);
  let loading = $state(false);
  let error = $state("");
  let items = $state(["Loading..."]);
  let iconMap = $state({});

  // API base URL — configurable via VITE_API_URL env var at build/dev-server time
  const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:3000";

  const beltPresets = [
    { label: "Mk.1", rate: 60 },
    { label: "Mk.2", rate: 120 },
    { label: "Mk.3", rate: 270 },
    { label: "Mk.4", rate: 480 },
    { label: "Mk.5", rate: 780 },
    { label: "Mk.6", rate: 1200 },
  ];

  const pipePresets = [
    { label: "Mk.1", rate: 300 },
    { label: "Mk.2", rate: 600 },
    { label: "Mk.3", rate: 1200 },
  ];

  let dropdownOpen = $state(false);

  function setBeltRate(rate) {
    desiredRate = rate;
  }

  function setPipeRate(rate) {
    desiredRate = rate;
  }

  function selectItem(item) {
    selectedItem = item;
    dropdownOpen = false;
  }

  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
  }

  function closeDropdown(e) {
    if (!e.currentTarget.contains(e.relatedTarget)) {
      dropdownOpen = false;
    }
  }

  function getIcon(item) {
    return iconMap[item] || null;
  }

  // Fetch available items and icon map from the backend API
  async function fetchItems() {
    try {
      const res = await fetch(`${API_BASE}/api/items`);
      if (res.ok) {
        items = await res.json();
        if (items.length > 0 && !items.includes(selectedItem)) {
          selectedItem = items[0];
        }
      }
    } catch (e) {
      console.error("Failed to fetch items:", e);
    }
  }

  async function fetchIcons() {
    try {
      const res = await fetch(`${API_BASE}/api/icons`);
      if (res.ok) {
        iconMap = await res.json();
      }
    } catch (e) {
      console.error("Failed to fetch icons:", e);
    }
  }

  // Fetch both in parallel
  Promise.all([fetchItems(), fetchIcons()]);

  async function calculate() {
    loading = true;
    error = "";
    result = null;

    try {
      const res = await fetch(`${API_BASE}/api/calculate`, {
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
      <label>Target Item</label>
      <div class="custom-select" tabindex="0" on:focusout={closeDropdown}>
        <div class="custom-select-trigger" on:click={toggleDropdown}>
          {#if getIcon(selectedItem)}
            <img src={getIcon(selectedItem)} alt={selectedItem} class="select-icon" />
          {/if}
          <span class="custom-select-text">{selectedItem}</span>
          <span class="custom-select-arrow">▼</span>
        </div>
        {#if dropdownOpen}
          <ul class="custom-select-options">
            {#each items as item}
              <li
                class="custom-select-option"
                class:active={item === selectedItem}
                on:click={() => selectItem(item)}
              >
                {#if getIcon(item)}
                  <img src={getIcon(item)} alt={item} class="option-icon" />
                {/if}
                <span>{item}</span>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
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
      <div class="belt-presets">
        <span class="belt-label">Belt:</span>
        {#each beltPresets as preset}
          <button
            class="belt-btn"
            class:active={desiredRate === preset.rate}
            on:click={() => setBeltRate(preset.rate)}
            title="Conveyor Belt {preset.label} — {preset.rate} items/min"
          >
            {preset.label}
          </button>
        {/each}
      </div>
      <div class="belt-presets">
        <span class="belt-label">Pipe:</span>
        {#each pipePresets as preset}
          <button
            class="pipe-btn"
            class:active={desiredRate === preset.rate}
            on:click={() => setPipeRate(preset.rate)}
            title="Pipeline {preset.label} — {preset.rate} m³/min"
          >
            {preset.label}
          </button>
        {/each}
      </div>
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
              <td class="item-cell">
                {#if getIcon(node.item)}
                  <img
                    src={getIcon(node.item)}
                    alt={node.item}
                    class="item-icon"
                  />
                {/if}
                {node.item}
              </td>
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
            <div class="raw-left">
              {#if getIcon(resource)}
                <img src={getIcon(resource)} alt={resource} class="item-icon" />
              {/if}
              <span class="raw-name">{resource}</span>
            </div>
            <span class="raw-rate">{rate.toFixed(2)}/min</span>
          </div>
        {/each}
      </div>

      {#if result.byproducts && result.byproducts.length > 0}
        <h3>🎁 Byproducts</h3>
        <div class="byproduct-grid">
          {#each result.byproducts as bp}
            <div class="byproduct-card">
              <div class="bp-header">
                <div class="bp-left">
                  {#if getIcon(bp.item)}
                    <img
                      src={getIcon(bp.item)}
                      alt={bp.item}
                      class="item-icon"
                    />
                  {/if}
                  <span class="bp-name">{bp.item}</span>
                </div>
                <span class="bp-rate">+{bp.rate.toFixed(2)}/min</span>
              </div>
              <span class="bp-source"
                >from {bp.source_item} ({bp.source_machine})</span
              >
            </div>
          {/each}
        </div>
      {/if}
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

  .belt-presets {
    display: flex;
    gap: 4px;
    align-items: center;
    margin-top: 6px;
  }

  .belt-label {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-right: 2px;
  }

  .belt-btn {
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 3px 8px;
    color: var(--text-secondary);
    font-size: 0.75rem;
    font-family: var(--font-mono);
    cursor: pointer;
    transition: all 0.15s;
  }

  .belt-btn:hover {
    border-color: var(--accent);
    color: var(--accent);
  }

  .belt-btn.active {
    background: var(--accent);
    border-color: var(--accent);
    color: #000;
    font-weight: 600;
  }

  .pipe-btn {
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 3px 8px;
    color: var(--text-secondary);
    font-size: 0.75rem;
    font-family: var(--font-mono);
    cursor: pointer;
    transition: all 0.15s;
  }

  .pipe-btn:hover {
    border-color: #3b82f6;
    color: #3b82f6;
  }

  .pipe-btn.active {
    background: #3b82f6;
    border-color: #3b82f6;
    color: #fff;
    font-weight: 600;
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

  .byproduct-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 12px;
  }

  .byproduct-card {
    background: var(--bg-card);
    border: 1px solid rgba(34, 197, 94, 0.3);
    border-radius: 8px;
    padding: 14px 16px;
  }

  .bp-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
  }

  .bp-name {
    font-weight: 500;
  }

  .bp-rate {
    font-family: var(--font-mono);
    color: var(--success);
    font-size: 0.9rem;
    font-weight: 600;
  }

  .bp-source {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  /* Item icons */
  .item-icon {
    width: 24px;
    height: 24px;
    object-fit: contain;
    vertical-align: middle;
    margin-right: 6px;
    image-rendering: pixelated;
    flex-shrink: 0;
  }

  /* Custom select dropdown */
  .custom-select {
    position: relative;
    outline: none;
  }

  .custom-select-trigger {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 10px 14px;
    color: var(--text-primary);
    font-size: 1rem;
    font-family: var(--font-sans);
    cursor: pointer;
    transition: border-color 0.2s;
    min-height: 42px;
  }

  .custom-select:focus-within .custom-select-trigger,
  .custom-select-trigger:hover {
    border-color: var(--accent);
  }

  .custom-select-text {
    flex: 1;
  }

  .custom-select-arrow {
    color: var(--text-muted);
    font-size: 0.7rem;
    margin-left: auto;
  }

  .custom-select-options {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    z-index: 100;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    margin-top: 4px;
    max-height: 280px;
    overflow-y: auto;
    list-style: none;
    padding: 4px 0;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  }

  .custom-select-option {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    cursor: pointer;
    color: var(--text-primary);
    font-size: 0.9rem;
    transition: background 0.1s;
  }

  .custom-select-option:hover {
    background: var(--bg-tertiary);
  }

  .custom-select-option.active {
    background: var(--accent-glow);
    color: var(--accent);
    font-weight: 600;
  }

  .select-icon {
    width: 28px;
    height: 28px;
    object-fit: contain;
    image-rendering: pixelated;
    flex-shrink: 0;
  }

  .option-icon {
    width: 24px;
    height: 24px;
    object-fit: contain;
    image-rendering: pixelated;
    flex-shrink: 0;
  }

  .item-cell {
    display: flex;
    align-items: center;
    white-space: nowrap;
  }

  .raw-left {
    display: flex;
    align-items: center;
    min-width: 0;
  }

  .bp-left {
    display: flex;
    align-items: center;
    min-width: 0;
  }
</style>
