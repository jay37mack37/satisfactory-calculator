<script>
  import { iconUrl } from "../engine.js";

  let { byproducts } = $props();
</script>

<section>
  <h3>🎁 Byproducts</h3>
  <div class="grid">
    {#each byproducts as bp}
      <div class="bp-card">
        <div class="bp-head">
          <div class="bp-left">
            {#if iconUrl(bp.item)}
              <img src={iconUrl(bp.item)} alt={bp.item} class="item-icon" />
            {/if}
            <span class="bp-name">{bp.item}</span>
          </div>
          <span class="bp-rate">+{bp.rate.toFixed(2)}/min</span>
        </div>
        <span class="bp-source">from {bp.source_item} · {bp.source_machine}</span>
      </div>
    {/each}
  </div>
</section>

<style>
  h3 {
    font-size: 1.15rem;
    margin: 0 0 14px;
    color: var(--text-primary);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(230px, 1fr));
    gap: 12px;
    margin-bottom: 28px;
  }

  .bp-card {
    background: rgba(34, 197, 94, 0.08);
    border: 1px solid var(--success-glow);
    border-radius: var(--radius);
    padding: 14px 16px;
    backdrop-filter: blur(10px);
  }

  .bp-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
    margin-bottom: 6px;
  }

  .bp-left {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .bp-name {
    font-weight: 500;
    font-size: 0.92rem;
  }

  .bp-rate {
    font-family: var(--font-mono);
    color: var(--success);
    font-size: 0.88rem;
    font-weight: 700;
    white-space: nowrap;
  }

  .bp-source {
    font-size: 0.74rem;
    color: var(--text-muted);
  }
</style>