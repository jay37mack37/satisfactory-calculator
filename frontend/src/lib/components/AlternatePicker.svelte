<script>
  import { iconUrl } from "../engine.js";

  let { entries, overrides, onchange } = $props();

  function recipeDisplay(name) {
    return name.replace("Alternate: ", "⚡ ");
  }
</script>

<section class="alt-section">
  <h3>🔀 Alternate Recipes</h3>
  <p class="hint">
    Swap in alternate recipes for any step. Default uses the standard recipe —
    re-run Calculate to apply your choices.
  </p>
  <div class="grid">
    {#each entries as entry}
      <div class="alt-card glass">
        <div class="alt-head">
          {#if iconUrl(entry.item)}
            <img src={iconUrl(entry.item)} alt={entry.item} class="item-icon" />
          {/if}
          <span class="alt-name">{entry.item}</span>
        </div>
        <div class="select-wrap">
          <select
            value={overrides[entry.item] || ""}
            onchange={(e) => onchange(entry.item, e.target.value)}
          >
            <option value="">Standard</option>
            {#each entry.options as opt}
              {#if opt.is_alternate}
                <option value={opt.recipe_name}>
                  {recipeDisplay(opt.recipe_name)} · {opt.output_rate}/min · {opt.machine_name}
                </option>
              {/if}
            {/each}
          </select>
          <svg class="chevron" viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
            <path fill="currentColor" d="M7 10l5 5 5-5z" />
          </svg>
        </div>
      </div>
    {/each}
  </div>
</section>

<style>
  h3 {
    font-size: 1.15rem;
    margin: 0 0 6px;
    color: var(--text-primary);
  }

  .hint {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin-bottom: 14px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 12px;
    margin-bottom: 28px;
  }

  .alt-card {
    padding: 14px;
  }

  .alt-head {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .alt-name {
    font-weight: 600;
    font-size: 0.95rem;
  }

  .select-wrap {
    position: relative;
  }

  select {
    width: 100%;
    appearance: none;
    -webkit-appearance: none;
    background: rgba(0, 0, 0, 0.25);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 8px 32px 8px 12px;
    color: var(--text-primary);
    font-size: 0.82rem;
    font-family: var(--font-sans);
    outline: none;
    cursor: pointer;
    transition: border-color 0.2s;
  }

  select:focus {
    border-color: var(--accent);
  }

  .chevron {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted);
    pointer-events: none;
  }
</style>