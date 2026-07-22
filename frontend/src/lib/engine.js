// JavaScript port of backend/src/recipe_engine.rs.
// Mirrors the Rust RecipeDatabase / walk_tree logic so the frontend can run
// entirely client-side (no backend) and deploy as a static site.
//
// One intentional addition over the Rust struct: each ProductionNode carries
// a `depth` field (0 at the target, +1 per recursion level) so the UI can
// render an indented production tree. It is excluded from Rust-parity checks.

import recipesData from "./data/recipes.json";
import iconMapData from "./data/iconMap.json";

/** @typedef {{recipe_name:string, output_item:string, output_rate:number, machine_name:string, power_cost:number, required_inputs:Object<string,number>, byproducts:Object<string,number>}} Recipe */
/** @typedef {{item:string, rate:number, machines_needed:number, machine_name:string, power_cost:number, recipe_name:string, depth:number}} ProductionNode */
/** @typedef {{item:string, rate:number, source_item:string, source_machine:string}} ByproductNode */
/** @typedef {{target_item:string, target_rate:number, nodes:ProductionNode[], byproducts:ByproductNode[], total_power:number, raw_resources:Object<string,number>}} CalculationResult */

/** A recipe is an alternate iff its name is prefixed with "Alternate:" (matches Rust `is_alternate`). */
export function isAlternate(recipe) {
  return recipe.recipe_name.startsWith("Alternate:");
}

export class RecipeDatabase {
  /** @param {Recipe[]} recipes */
  constructor(recipes) {
    /** @type {Map<string, Recipe>} */
    this.recipesByName = new Map();
    /** @type {Map<string, string[]>} output_item -> recipe_name[] */
    this.recipesByOutput = new Map();

    for (const recipe of recipes) {
      this.recipesByName.set(recipe.recipe_name, recipe);
      const list = this.recipesByOutput.get(recipe.output_item) ?? [];
      list.push(recipe.recipe_name);
      this.recipesByOutput.set(recipe.output_item, list);
    }
  }

  /** Sorted list of all craftable item names (mirrors `items()`). */
  items() {
    return [...this.recipesByOutput.keys()].sort();
  }

  /** The default (non-alternate) recipe for an item, if one exists. */
  defaultRecipe(item) {
    const names = this.recipesByOutput.get(item);
    if (!names) return undefined;
    for (const name of names) {
      const r = this.recipesByName.get(name);
      if (r && !isAlternate(r)) return r;
    }
    return undefined;
  }

  /** Recipe to use for an item, honoring user overrides (mirrors `get_recipe`). */
  getRecipe(item, overrides) {
    const overrideName = overrides[item];
    if (overrideName) return this.recipesByName.get(overrideName);
    return this.defaultRecipe(item);
  }

  /** All recipes that produce `item` (standard + alternates). */
  alternateRecipes(item) {
    const names = this.recipesByOutput.get(item);
    if (!names) return [];
    const result = [];
    for (const name of names) {
      const r = this.recipesByName.get(name);
      if (!r) continue;
      result.push({
        recipe_name: r.recipe_name,
        output_item: r.output_item,
        output_rate: r.output_rate,
        machine_name: r.machine_name,
        is_alternate: isAlternate(r),
        required_inputs: { ...r.required_inputs },
        byproducts: { ...r.byproducts },
      });
    }
    return result;
  }

  /** Map of item -> recipes, for every item that has more than one recipe. */
  allAlternates() {
    const result = {};
    for (const item of this.recipesByOutput.keys()) {
      const alts = this.alternateRecipes(item);
      if (alts.length > 1) result[item] = alts;
    }
    return result;
  }

  /**
   * Recursively walk the recipe tree backward from `item` down to raw ores,
   * returning every production node (mirrors `calculate_requirements`).
   * @param {string} item
   * @param {number} rate
   * @param {Object<string,string>} overrides item -> recipe_name
   * @returns {CalculationResult}
   */
  calculateRequirements(item, rate, overrides = {}) {
    const nodes = [];
    const byproducts = [];
    const rawResources = {};
    let totalPower = 0;

    this.#walkTree(item, rate, overrides, 0, nodes, byproducts, rawResources, (p) => (totalPower += p));

    return {
      target_item: item,
      target_rate: rate,
      nodes,
      byproducts,
      total_power: totalPower,
      raw_resources: rawResources,
    };
  }

  /**
   * @param {string} item
   * @param {number} rate
   * @param {Object<string,string>} overrides
   * @param {number} depth
   * @param {ProductionNode[]} nodes
   * @param {ByproductNode[]} byproducts
   * @param {Object<string,number>} rawResources
   * @param {(power:number)=>void} addPower
   */
  #walkTree(item, rate, overrides, depth, nodes, byproducts, rawResources, addPower) {
    const recipe = this.getRecipe(item, overrides);
    if (!recipe) {
      // No recipe found → raw resource (mirrors Rust `None` arm).
      rawResources[item] = (rawResources[item] ?? 0) + rate;
      return;
    }

    const machinesNeeded = rate / recipe.output_rate;
    const power = machinesNeeded * recipe.power_cost;
    addPower(power);

    nodes.push({
      item,
      rate,
      machines_needed: machinesNeeded,
      machine_name: recipe.machine_name,
      power_cost: power,
      recipe_name: recipe.recipe_name,
      depth,
    });

    // Byproducts from this step.
    for (const [bpItem, bpRatePerMachine] of Object.entries(recipe.byproducts)) {
      byproducts.push({
        item: bpItem,
        rate: bpRatePerMachine * machinesNeeded,
        source_item: item,
        source_machine: recipe.machine_name,
      });
    }

    // A recipe with no inputs is itself a raw resource.
    if (Object.keys(recipe.required_inputs).length === 0) {
      rawResources[item] = (rawResources[item] ?? 0) + rate;
    }

    // Recurse into each input.
    for (const [inputItem, inputRatePerMachine] of Object.entries(recipe.required_inputs)) {
      const requiredRate = inputRatePerMachine * machinesNeeded;
      this.#walkTree(inputItem, requiredRate, overrides, depth + 1, nodes, byproducts, rawResources, addPower);
    }
  }
}

/** Module-singleton database built from the bundled recipe data. */
export const db = new RecipeDatabase(recipesData.recipes);

/** Bundled item -> "/icons/<File>.png" map. */
export const iconMap = /** @type {Object<string,string>} */ (iconMapData);

/**
 * Resolve an item's icon URL, accounting for the Vite base path so icons load
 * correctly under the GitHub Pages subpath. Returns null when no icon exists.
 */
export function iconUrl(item) {
  const path = iconMap[item];
  if (!path) return null;
  const base = import.meta.env.BASE_URL;
  return `${base}${path.replace(/^\//, "")}`;
}