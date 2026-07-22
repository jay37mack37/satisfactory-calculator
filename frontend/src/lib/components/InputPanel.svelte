<script>
  import { iconUrl } from "../engine.js";

  let {
    items,
    selectedItem = $bindable(),
    desiredRate = $bindable(),
    loading = false,
    oncalculate,
  } = $props();

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

  function selectItem(e) {
    selectedItem = e.target.value;
  }
</script>

<section class="panel glass">
  <div class="field">
    <label for="item-select">Target Item</label>
    <div class="select-wrap">
      <select id="item-select" value={selectedItem} onchange={selectItem}>
        {#each items as item}
          <option value={item}>{item}</option>
        {/each}
      </select>
      <svg class="chevron" viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
        <path fill="currentColor" d="M7 10l5 5 5-5z" />
      </svg>
    </div>
    {#if iconUrl(selectedItem)}
      <div class="preview">
        <img src={iconUrl(selectedItem)} alt={selectedItem} class="preview-icon" />
        <span>{selectedItem}</span>
      </div>
    {/if}
  </div>

  <div class="field">
    <label for="rate-input">Rate (items/min)</label>
    <input id="rate-input" type="number" min="0.1" step="0.1" bind:value={desiredRate} />

    <div class="presets">
      <span class="preset-label belt">Belt</span>
      {#each beltPresets as p}
        <button
          class="pill belt"
          class:active={desiredRate === p.rate}
          onclick={() => (desiredRate = p.rate)}
          title={`Conveyor Belt ${p.label} — ${p.rate} items/min`}
        >
          {p.label}
        </button>
      {/each}
    </div>
    <div class="presets">
      <span class="preset-label pipe">Pipe</span>
      {#each pipePresets as p}
        <button
          class="pill pipe"
          class:active={desiredRate === p.rate}
          onclick={() => (desiredRate = p.rate)}
          title={`Pipeline ${p.label} — ${p.rate} m³/min`}
        >
          {p.label}
        </button>
      {/each}
    </div>
  </div>

  <button class="calc-btn" onclick={oncalculate} disabled={loading}>
    {#if loading}
      <span class="spinner"></span> Calculating…
    {:else}
      ⚡ Calculate
    {/if}
  </button>
</section>

<style>
  .panel {
    display: flex;
    gap: 20px;
    align-items: flex-end;
    padding: 24px;
    margin-bottom: 32px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  label {
    font-size: 0.78rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .select-wrap {
    position: relative;
  }

  select,
  input {
    width: 100%;
    background: rgba(0, 0, 0, 0.25);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 12px 14px;
    color: var(--text-primary);
    font-size: 1rem;
    font-family: var(--font-sans);
    outline: none;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  select {
    appearance: none;
    -webkit-appearance: none;
    padding-right: 40px;
    cursor: pointer;
  }

  .chevron {
    position: absolute;
    right: 12px;
    bottom: 12px;
    color: var(--text-muted);
    pointer-events: none;
  }

  select:focus,
  input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-glow);
  }

  .preview {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
  }

  .preview-icon {
    width: 40px;
    height: 40px;
    object-fit: contain;
    image-rendering: -webkit-optimize-contrast;
  }

  .preview span {
    font-weight: 600;
  }

  .presets {
    display: flex;
    gap: 6px;
    align-items: center;
    flex-wrap: wrap;
  }

  .preset-label {
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 3px 8px;
    border-radius: 999px;
  }
  .preset-label.belt {
    color: var(--accent);
    background: var(--accent-glow);
  }
  .preset-label.pipe {
    color: var(--pipe);
    background: rgba(59, 130, 246, 0.15);
  }

  .pill {
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 4px 10px;
    color: var(--text-secondary);
    font-size: 0.78rem;
    font-family: var(--font-mono);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }

  .pill.belt:hover {
    border-color: var(--accent);
    color: var(--accent);
  }
  .pill.belt.active {
    background: var(--accent-grad);
    border-color: transparent;
    color: #1a1102;
    font-weight: 700;
  }

  .pill.pipe:hover {
    border-color: var(--pipe);
    color: var(--pipe);
  }
  .pill.pipe.active {
    background: var(--pipe-grad);
    border-color: transparent;
    color: #fff;
    font-weight: 700;
  }

  .calc-btn {
    background: var(--accent-grad);
    color: #1a1102;
    border: none;
    border-radius: var(--radius-sm);
    padding: 14px 30px;
    font-size: 1rem;
    font-weight: 700;
    font-family: var(--font-sans);
    cursor: pointer;
    white-space: nowrap;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    transition: transform 0.1s, box-shadow 0.2s, filter 0.2s;
    box-shadow: 0 6px 20px var(--accent-glow);
  }

  .calc-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    filter: brightness(1.05);
  }

  .calc-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  .calc-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(0, 0, 0, 0.25);
    border-top-color: #1a1102;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @media (max-width: 720px) {
    .panel {
      flex-direction: column;
      align-items: stretch;
    }
    .calc-btn {
      width: 100%;
      justify-content: center;
    }
  }
</style>