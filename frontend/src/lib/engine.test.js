import { describe, it, expect } from "vitest";
import { db, RecipeDatabase, isAlternate, iconUrl } from "./engine.js";
import recipesData from "./data/recipes.json";

// ─── Database construction ──────────────────────────────────────────

describe("RecipeDatabase", () => {
  it("is built from the bundled recipe data", () => {
    const local = new RecipeDatabase(recipesData.recipes);
    expect(local.items().length).toBeGreaterThan(60); // 71 unique craftable items
    expect(local.items()).toEqual(db.items());
  });

  it("items() returns a sorted, non-empty list of craftable items", () => {
    const items = db.items();
    expect(items.length).toBeGreaterThan(0);
    expect(items.includes("Rotor")).toBe(true);
    const sorted = [...items].sort();
    expect(items).toEqual(sorted);
  });

  it("defaultRecipe() returns the non-alternate recipe for an item", () => {
    const r = db.defaultRecipe("Iron Ingot");
    expect(r).toBeDefined();
    expect(r.recipe_name).toBe("Iron Ingot");
    expect(isAlternate(r)).toBe(false);
  });

  it("alternateRecipes() lists every recipe producing an item", () => {
    const alts = db.alternateRecipes("Iron Ingot");
    expect(alts.length).toBe(5); // standard + 4 alternates
    expect(alts.filter((a) => a.is_alternate).length).toBe(4);
    expect(alts.find((a) => a.recipe_name === "Alternate: Pure Iron Ingot")).toBeDefined();
  });

  it("allAlternates() only includes items with more than one recipe", () => {
    const all = db.allAlternates();
    expect(Object.keys(all).length).toBeGreaterThan(0);
    expect(all["Iron Ingot"].length).toBe(5);
    // Rotor has standard + Copper Rotor + Steel Rotor alternates.
    expect(all["Rotor"]).toBeDefined();
    expect(all["Rotor"].length).toBe(3);
    // Every entry in the map has at least two recipes (the invariant).
    for (const opts of Object.values(all)) {
      expect(opts.length).toBeGreaterThan(1);
    }
  });
});

// ─── Calculation: Rotor @ 10/min (no overrides) ─────────────────────
// Hand-traced from recipes.json:
//   Rotor (4/min, 15 MW) <- Iron Rod 10, Screws 24
//   Iron Rod (15/min, 4 MW) <- Iron Ingot 15
//   Iron Ingot (30/min, 4 MW) <- Iron Ore 30
//   Iron Ore (60/min, 5 MW) <- (raw)
//   Screws (40/min, 4 MW) <- Iron Rod 10
// DFS node order: Rotor, Iron Rod(25), Iron Ingot(25), Iron Ore(25),
//                  Screws(60), Iron Rod(15), Iron Ingot(15), Iron Ore(15)

describe("calculateRequirements — Rotor @ 10/min", () => {
  const result = db.calculateRequirements("Rotor", 10);

  it("returns the target item and rate", () => {
    expect(result.target_item).toBe("Rotor");
    expect(result.target_rate).toBe(10);
  });

  it("produces 8 production nodes in DFS order", () => {
    expect(result.nodes.length).toBe(8);
    const names = result.nodes.map((n) => n.item);
    expect(names).toEqual([
      "Rotor",
      "Iron Rod",
      "Iron Ingot",
      "Iron Ore",
      "Screws",
      "Iron Rod",
      "Iron Ingot",
      "Iron Ore",
    ]);
  });

  it("computes the root node correctly", () => {
    const rotor = result.nodes[0];
    expect(rotor.recipe_name).toBe("Rotor");
    expect(rotor.machine_name).toBe("Assembler");
    expect(rotor.machines_needed).toBeCloseTo(2.5, 5);
    expect(rotor.power_cost).toBeCloseTo(37.5, 5);
    expect(rotor.depth).toBe(0);
  });

  it("assigns increasing depth along the tree", () => {
    // Rotor(0) -> Iron Rod(1) -> Iron Ingot(2) -> Iron Ore(3)
    expect(result.nodes[0].depth).toBe(0);
    expect(result.nodes[1].depth).toBe(1);
    expect(result.nodes[2].depth).toBe(2);
    expect(result.nodes[3].depth).toBe(3);
    // Screws is a sibling of the first Iron Rod, so depth 1.
    expect(result.nodes[4].depth).toBe(1);
    expect(result.nodes[5].depth).toBe(2);
  });

  it("sums total power across all nodes", () => {
    // 37.5 + 6.6667 + 3.3333 + 2.0833 + 6 + 4 + 2 + 1.25 ≈ 62.8333
    expect(result.total_power).toBeCloseTo(62.8333, 3);
  });

  it("accumulates raw resources (only Iron Ore)", () => {
    expect(Object.keys(result.raw_resources)).toEqual(["Iron Ore"]);
    expect(result.raw_resources["Iron Ore"]).toBeCloseTo(40, 5);
  });

  it("has no byproducts for the standard Rotor chain", () => {
    expect(result.byproducts).toEqual([]);
  });
});

// ─── Override: use Alternate: Cast Screws ───────────────────────────
// Cast Screws (50/min, 4 MW) <- Iron Ingot 12.5  (no Iron Rod!)
// So Screws no longer pulls an Iron Rod branch.

describe("calculateRequirements — alternate override", () => {
  const standard = db.calculateRequirements("Rotor", 10);
  const overridden = db.calculateRequirements("Rotor", 10, {
    Screws: "Alternate: Cast Screws",
  });

  it("uses the overridden recipe for Screws", () => {
    const screwsNodes = overridden.nodes.filter((n) => n.item === "Screws");
    expect(screwsNodes.length).toBe(1);
    expect(screwsNodes[0].recipe_name).toBe("Alternate: Cast Screws");
  });

  it("removes the second Iron Rod branch that Screws used to pull", () => {
    const standardRodCount = standard.nodes.filter((n) => n.item === "Iron Rod").length;
    const overriddenRodCount = overridden.nodes.filter((n) => n.item === "Iron Rod").length;
    expect(standardRodCount).toBe(2);
    expect(overriddenRodCount).toBe(1);
  });

  it("changes the total power and node count", () => {
    // Cast Screws skips the Iron Rod step, so there is one fewer node
    // (7 vs 8) and less power (no Iron Rod machine for the screws branch).
    expect(overridden.nodes.length).toBe(7);
    expect(standard.nodes.length).toBe(8);
    expect(overridden.total_power).toBeCloseTo(57.6333, 3);
    expect(standard.total_power).toBeCloseTo(62.8333, 3);
  });

  it("still ends up at the same Iron Ore total (40/min)", () => {
    // Coincidence of the numbers: both paths consume 15/min of iron-equivalent
    // for the screws branch, so the raw ore total is unchanged.
    expect(overridden.raw_resources["Iron Ore"]).toBeCloseTo(40, 5);
  });
});

// ─── Byproducts: Heavy Oil Residue @ 40/min ────────────────────────
// Heavy Oil Residue (40/min, 30 MW) <- Crude Oil 60, byproduct Polymer Resin 30/machine.

describe("calculateRequirements — byproducts", () => {
  const result = db.calculateRequirements("Heavy Oil Residue", 40);

  it("records the byproduct with rate and source", () => {
    expect(result.byproducts.length).toBe(1);
    const bp = result.byproducts[0];
    expect(bp.item).toBe("Polymer Resin");
    expect(bp.rate).toBeCloseTo(30, 5); // 30/machine * 1 machine
    expect(bp.source_item).toBe("Heavy Oil Residue");
    expect(bp.source_machine).toBe("Refinery");
  });

  it("counts Crude Oil as a raw resource", () => {
    expect(result.raw_resources["Crude Oil"]).toBeCloseTo(60, 5);
  });
});

// ─── Icons ──────────────────────────────────────────────────────────

describe("iconUrl", () => {
  it("resolves a known item to a base-relative path", () => {
    const url = iconUrl("Rotor");
    expect(url).toMatch(/icons\/Rotor\.png$/);
    expect(url.startsWith("/")).toBe(true); // BASE_URL defaults to "/" in tests
  });

  it("returns null for items without an icon", () => {
    expect(iconUrl("Nonexistent Item")).toBeNull();
  });
});