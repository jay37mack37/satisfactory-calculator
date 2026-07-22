<script>
  import { db, iconUrl } from "./lib/engine.js";
  import Header from "./lib/components/Header.svelte";
  import InputPanel from "./lib/components/InputPanel.svelte";
  import SummaryCards from "./lib/components/SummaryCards.svelte";
  import AlternatePicker from "./lib/components/AlternatePicker.svelte";
  import ProductionTable from "./lib/components/ProductionTable.svelte";
  import ProductionTree from "./lib/components/ProductionTree.svelte";
  import RawResources from "./lib/components/RawResources.svelte";
  import Byproducts from "./lib/components/Byproducts.svelte";

  const items = db.items();
  const alternates = db.allAlternates();

  let selectedItem = $state(items.includes("Rotor") ? "Rotor" : items[0]);
  let desiredRate = $state(10);
  let result = $state(null);
  let error = $state("");

  // item -> recipe_name chosen by the user (only alternates stored).
  let recipeOverrides = $state({});

  // Items in the current result that have more than one recipe.
  let itemsWithAlternates = $derived(
    result
      ? result.nodes
          .filter((n) => alternates[n.item] && alternates[n.item].length > 1)
          .map((n) => ({
            item: n.item,
            currentRecipe: n.recipe_name,
            options: alternates[n.item],
          }))
      : []
  );

  function calculate() {
    error = "";
    try {
      result = db.calculateRequirements(selectedItem, desiredRate, recipeOverrides);
    } catch (e) {
      error = e.message;
      result = null;
    }
  }

  function setRecipeOverride(item, recipeName) {
    if (recipeName === "") {
      const { [item]: _omit, ...rest } = recipeOverrides;
      recipeOverrides = rest;
    } else {
      recipeOverrides = { ...recipeOverrides, [item]: recipeName };
    }
  }
</script>

<main class="container">
  <Header />

  <InputPanel
    {items}
    bind:selectedItem
    bind:desiredRate
    oncalculate={calculate}
  />

  {#if error}
    <div class="error-banner">❌ {error}</div>
  {/if}

  {#if result}
    <section class="results">
      <h2>
        <span class="target-name">{result.target_item}</span>
        <span class="target-rate">@ {result.target_rate}/min</span>
      </h2>

      <SummaryCards {result} />

      {#if itemsWithAlternates.length > 0}
        <AlternatePicker
          entries={itemsWithAlternates}
          overrides={recipeOverrides}
          onchange={setRecipeOverride}
        />
      {/if}

      <ProductionTree nodes={result.nodes} />
      <ProductionTable nodes={result.nodes} />
      <RawResources rawResources={result.raw_resources} />

      {#if result.byproducts && result.byproducts.length > 0}
        <Byproducts byproducts={result.byproducts} />
      {/if}
    </section>
  {:else}
    <div class="empty glass">
      <div class="empty-icon">🏭</div>
      <p>Pick an item and hit <strong>Calculate</strong> to see the full production chain.</p>
    </div>
  {/if}
</main>

<style>
  .container {
    max-width: 1040px;
    margin: 0 auto;
    padding: 40px 24px 80px;
  }

  .results {
    animation: fadeIn 0.35s ease;
  }

  h2 {
    font-size: 1.5rem;
    font-weight: 700;
    margin-bottom: 22px;
    display: flex;
    align-items: baseline;
    gap: 10px;
    flex-wrap: wrap;
  }

  .target-name {
    color: var(--text-primary);
  }

  .target-rate {
    font-family: var(--font-mono);
    font-size: 1.1rem;
    color: var(--accent);
    font-weight: 600;
  }

  .error-banner {
    background: rgba(239, 68, 68, 0.12);
    border: 1px solid var(--error);
    border-radius: var(--radius-sm);
    padding: 14px 18px;
    color: var(--error);
    margin-bottom: 24px;
  }

  .empty {
    text-align: center;
    padding: 64px 24px;
    color: var(--text-muted);
  }

  .empty-icon {
    font-size: 2.6rem;
    margin-bottom: 12px;
    opacity: 0.6;
  }

  .empty strong {
    color: var(--accent);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>