<script>
  let { result } = $props();

  let cards = $derived([
    {
      label: "Total Power",
      value: result.total_power.toFixed(1),
      unit: "MW",
      icon: "⚡",
      accent: "power",
    },
    {
      label: "Production Steps",
      value: String(result.nodes.length),
      unit: "",
      icon: "🏭",
      accent: "steps",
    },
    {
      label: "Raw Resources",
      value: String(Object.keys(result.raw_resources).length),
      unit: "",
      icon: "⛏️",
      accent: "raw",
    },
  ]);
</script>

<div class="summary">
  {#each cards as c}
    <div class="card glass">
      <div class="card-top">
        <span class="card-icon">{c.icon}</span>
        <span class="card-label">{c.label}</span>
      </div>
      <div class="card-value">
        {c.value}<span class="unit">{c.unit}</span>
      </div>
    </div>
  {/each}
</div>

<style>
  .summary {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    margin-bottom: 32px;
  }

  .card {
    padding: 20px 22px;
    position: relative;
    overflow: hidden;
  }

  .card::before {
    content: "";
    position: absolute;
    inset: 0 0 auto 0;
    height: 3px;
    background: var(--accent-grad);
    opacity: 0.85;
  }

  .card-top {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .card-icon {
    font-size: 1.1rem;
  }

  .card-label {
    font-size: 0.74rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
  }

  .card-value {
    font-size: 2rem;
    font-weight: 800;
    font-family: var(--font-mono);
    color: var(--text-primary);
    line-height: 1;
  }

  .unit {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-muted);
    margin-left: 4px;
  }

  @media (max-width: 560px) {
    .summary {
      grid-template-columns: 1fr;
    }
  }
</style>