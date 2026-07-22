<script>
  import { iconUrl } from "../engine.js";

  let { nodes } = $props();

  function recipeLabel(name) {
    return name.startsWith("Alternate:")
      ? "⚡ " + name.replace("Alternate: ", "")
      : "Standard";
  }
</script>

<section>
  <h3>🌳 Production Tree</h3>
  <p class="hint">The full dependency chain, indented by depth — from your target down to raw ore.</p>
  <div class="tree glass">
    {#each nodes as node}
      <div class="row" style="--depth: {node.depth}">
        <div class="guides">
          {#each Array(node.depth) as _, i}
            <span class="guide" style="--i: {i}"></span>
          {/each}
        </div>
        <div class="node">
          {#if iconUrl(node.item)}
            <img src={iconUrl(node.item)} alt={node.item} class="item-icon" />
          {/if}
          <div class="node-main">
            <span class="node-name">{node.item}</span>
            <span class="node-recipe" class:alt={node.recipe_name.startsWith("Alternate:")}>
              {recipeLabel(node.recipe_name)} · {node.machine_name}
            </span>
          </div>
          <div class="node-stats">
            <span class="stat" title="Output rate">{node.rate.toFixed(2)}/min</span>
            <span class="stat dim" title="Machines needed">×{node.machines_needed.toFixed(2)}</span>
            <span class="stat dim" title="Power">{node.power_cost.toFixed(1)} MW</span>
          </div>
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

  .tree {
    padding: 10px 8px;
    margin-bottom: 28px;
    overflow-x: auto;
  }

  .row {
    --indent: 22px;
    display: flex;
    align-items: stretch;
  }

  .guides {
    display: flex;
    flex-shrink: 0;
  }

  .guide {
    width: var(--indent);
    position: relative;
  }

  .guide::before {
    /* vertical connector line */
    content: "";
    position: absolute;
    left: 50%;
    top: 0;
    bottom: 0;
    width: 1px;
    background: var(--border-strong);
  }

  .node {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    margin: 2px 0 2px 4px;
    border-radius: var(--radius-sm);
    background: rgba(0, 0, 0, 0.18);
    border: 1px solid var(--border);
    transition: background 0.15s, border-color 0.15s;
  }

  .node:hover {
    background: rgba(255, 255, 255, 0.04);
    border-color: var(--border-strong);
  }

  .node-main {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }

  .node-name {
    font-weight: 600;
    font-size: 0.92rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .node-recipe {
    font-size: 0.74rem;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .node-recipe.alt {
    color: var(--accent);
  }

  .node-stats {
    display: flex;
    gap: 12px;
    align-items: center;
    flex-shrink: 0;
  }

  .stat {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--accent);
    white-space: nowrap;
  }

  .stat.dim {
    color: var(--text-muted);
    font-weight: 500;
  }

  @media (max-width: 640px) {
    .node-stats {
      flex-direction: column;
      align-items: flex-end;
      gap: 0;
    }
  }
</style>