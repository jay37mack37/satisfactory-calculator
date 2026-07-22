<script>
  import { iconUrl } from "../engine.js";

  let { rawResources } = $props();

  let entries = $derived(
    Object.entries(rawResources).sort((a, b) => b[1] - a[1])
  );
</script>

<section>
  <h3>⛏️ Raw Resources Needed</h3>
  <div class="grid">
    {#each entries as [resource, rate]}
      <div class="raw-card glass">
        <div class="raw-left">
          {#if iconUrl(resource)}
            <img src={iconUrl(resource)} alt={resource} class="item-icon" />
          {/if}
          <span class="raw-name">{resource}</span>
        </div>
        <span class="raw-rate">{rate.toFixed(2)}/min</span>
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
    grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
    gap: 12px;
    margin-bottom: 28px;
  }

  .raw-card {
    padding: 14px 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
  }

  .raw-left {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .raw-name {
    font-weight: 500;
    font-size: 0.92rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .raw-rate {
    font-family: var(--font-mono);
    color: var(--accent);
    font-size: 0.88rem;
    font-weight: 600;
    white-space: nowrap;
  }
</style>