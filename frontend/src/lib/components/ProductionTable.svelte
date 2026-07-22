<script>
  import { iconUrl } from "../engine.js";

  let { nodes } = $props();

  function recipeCell(name) {
    if (name.startsWith("Alternate:")) {
      return { text: "⚡ " + name.replace("Alternate: ", ""), alt: true };
    }
    return { text: "Standard", alt: false };
  }
</script>

<section>
  <h3>All Production Nodes</h3>
  <div class="table-wrap glass">
    <table>
      <thead>
        <tr>
          <th>Item</th>
          <th>Recipe</th>
          <th class="num">Rate /min</th>
          <th>Machine</th>
          <th class="num">Machines</th>
          <th class="num">Power MW</th>
        </tr>
      </thead>
      <tbody>
        {#each nodes as node}
          <tr>
            <td class="item-cell">
              {#if iconUrl(node.item)}
                <img src={iconUrl(node.item)} alt={node.item} class="item-icon" />
              {/if}
              <span>{node.item}</span>
            </td>
            <td class="recipe-cell" class:alt={recipeCell(node.recipe_name).alt}>
              {recipeCell(node.recipe_name).text}
            </td>
            <td class="num">{node.rate.toFixed(2)}</td>
            <td>{node.machine_name}</td>
            <td class="num">{node.machines_needed.toFixed(2)}</td>
            <td class="num">{node.power_cost.toFixed(2)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

<style>
  h3 {
    font-size: 1.15rem;
    margin: 0 0 14px;
    color: var(--text-primary);
  }

  .table-wrap {
    overflow-x: auto;
    margin-bottom: 28px;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    min-width: 640px;
  }

  thead th {
    text-align: left;
    padding: 14px 16px;
    font-size: 0.72rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border-strong);
    background: rgba(0, 0, 0, 0.2);
  }

  th.num,
  td.num {
    text-align: right;
    font-family: var(--font-mono);
  }

  tbody td {
    padding: 12px 16px;
    font-size: 0.9rem;
    border-bottom: 1px solid var(--border);
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  tbody tr {
    transition: background 0.15s;
  }

  tbody tr:hover {
    background: rgba(255, 255, 255, 0.03);
  }

  .item-cell {
    display: flex;
    align-items: center;
    gap: 8px;
    white-space: nowrap;
    font-weight: 500;
  }

  .recipe-cell {
    font-size: 0.82rem;
    color: var(--text-secondary);
  }

  .recipe-cell.alt {
    color: var(--accent);
    font-weight: 600;
  }
</style>