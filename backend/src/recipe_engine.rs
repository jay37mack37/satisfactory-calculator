use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Data Structures ───────────────────────────────────────────────

/// Defines how an item is produced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub recipe_name: String,                     // unique name, e.g. "Iron Ingot" or "Alternate: Pure Iron Ingot"
    pub output_item: String,
    pub output_rate: f64,                        // items per minute per machine
    pub machine_name: String,                    // e.g. "Constructor", "Assembler"
    pub power_cost: f64,                         // MW per machine
    pub required_inputs: HashMap<String, f64>,   // item name -> items/min required
    #[serde(default)]
    pub byproducts: HashMap<String, f64>,        // byproduct item -> rate per machine
}

impl Recipe {
    pub fn is_alternate(&self) -> bool {
        self.recipe_name.starts_with("Alternate:")
    }
}

/// A single node in the production tree result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionNode {
    pub item: String,
    pub rate: f64,
    pub machines_needed: f64,
    pub machine_name: String,
    pub power_cost: f64,
    pub recipe_name: String,
}

/// A byproduct output from a production step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByproductNode {
    pub item: String,
    pub rate: f64,
    pub source_item: String,
    pub source_machine: String,
}

/// The full result of a calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub target_item: String,
    pub target_rate: f64,
    pub nodes: Vec<ProductionNode>,
    pub byproducts: Vec<ByproductNode>,
    pub total_power: f64,
    pub raw_resources: HashMap<String, f64>,
}

/// The incoming request from the frontend.
#[derive(Debug, Clone, Deserialize)]
pub struct CalculationRequest {
    pub item: String,
    pub rate: f64,
    /// Map of item name -> recipe_name to use for that item.
    /// If an item is not present, the default (non-alternate) recipe is used.
    #[serde(default)]
    pub recipe_overrides: HashMap<String, String>,
}

/// Info about available recipes for an item (for the alternates API).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeInfo {
    pub recipe_name: String,
    pub output_item: String,
    pub output_rate: f64,
    pub machine_name: String,
    pub is_alternate: bool,
    pub required_inputs: HashMap<String, f64>,
    pub byproducts: HashMap<String, f64>,
}

/// Helper struct for deserializing the JSON recipe file.
#[derive(Debug, Deserialize)]
struct RecipeFile {
    recipes: Vec<Recipe>,
}

// ─── Recipe Database ──────────────────────────────────────────────

pub struct RecipeDatabase {
    /// All recipes indexed by recipe_name
    recipes_by_name: HashMap<String, Recipe>,
    /// Maps output_item -> list of recipe_names that produce it
    recipes_by_output: HashMap<String, Vec<String>>,
}

impl RecipeDatabase {
    /// Loads recipes from the embedded JSON file.
    pub fn new() -> Self {
        let json_data = include_str!("recipes.json");
        let file: RecipeFile = serde_json::from_str(json_data)
            .expect("Failed to parse recipes.json");

        let mut recipes_by_name = HashMap::new();
        let mut recipes_by_output: HashMap<String, Vec<String>> = HashMap::new();

        for recipe in file.recipes {
            let name = recipe.recipe_name.clone();
            let output = recipe.output_item.clone();

            recipes_by_name.insert(name.clone(), recipe);
            recipes_by_output.entry(output).or_default().push(name);
        }

        RecipeDatabase { recipes_by_name, recipes_by_output }
    }

    /// Returns a sorted list of all craftable item names.
    pub fn items(&self) -> Vec<String> {
        let mut items: Vec<String> = self.recipes_by_output.keys().cloned().collect();
        items.sort();
        items
    }

    /// Returns the default (non-alternate) recipe for an item, if one exists.
    fn default_recipe(&self, item: &str) -> Option<&Recipe> {
        self.recipes_by_output.get(item).and_then(|names| {
            names.iter().find_map(|name| {
                let r = self.recipes_by_name.get(name)?;
                if r.is_alternate() { None } else { Some(r) }
            })
        })
    }

    /// Returns the recipe to use for an item, considering overrides.
    fn get_recipe(&self, item: &str, overrides: &HashMap<String, String>) -> Option<&Recipe> {
        if let Some(recipe_name) = overrides.get(item) {
            self.recipes_by_name.get(recipe_name)
        } else {
            self.default_recipe(item)
        }
    }

    /// Returns all alternate recipes for a given item.
    pub fn alternate_recipes(&self, item: &str) -> Vec<RecipeInfo> {
        let mut result = Vec::new();
        if let Some(names) = self.recipes_by_output.get(item) {
            for name in names {
                if let Some(recipe) = self.recipes_by_name.get(name) {
                    result.push(RecipeInfo {
                        recipe_name: recipe.recipe_name.clone(),
                        output_item: recipe.output_item.clone(),
                        output_rate: recipe.output_rate,
                        machine_name: recipe.machine_name.clone(),
                        is_alternate: recipe.is_alternate(),
                        required_inputs: recipe.required_inputs.clone(),
                        byproducts: recipe.byproducts.clone(),
                    });
                }
            }
        }
        result
    }

    /// Returns all recipes for all items that have alternates.
    pub fn all_alternates(&self) -> HashMap<String, Vec<RecipeInfo>> {
        let mut result = HashMap::new();
        for (item, _names) in &self.recipes_by_output {
            let alts = self.alternate_recipes(item);
            if alts.len() > 1 {
                result.insert(item.clone(), alts);
            }
        }
        result
    }

    /// Recursively walks the recipe tree backward from the target item
    /// all the way down to raw ores, returning every production node.
    pub fn calculate_requirements(&self, item: &str, rate: f64, overrides: &HashMap<String, String>) -> CalculationResult {
        let mut nodes: Vec<ProductionNode> = Vec::new();
        let mut byproducts: Vec<ByproductNode> = Vec::new();
        let mut raw_resources: HashMap<String, f64> = HashMap::new();
        let mut total_power: f64 = 0.0;

        self.walk_tree(item, rate, overrides, &mut nodes, &mut byproducts, &mut raw_resources, &mut total_power);

        CalculationResult {
            target_item: item.to_string(),
            target_rate: rate,
            nodes,
            byproducts,
            total_power,
            raw_resources,
        }
    }

    fn walk_tree(
        &self,
        item: &str,
        rate: f64,
        overrides: &HashMap<String, String>,
        nodes: &mut Vec<ProductionNode>,
        byproducts: &mut Vec<ByproductNode>,
        raw_resources: &mut HashMap<String, f64>,
        total_power: &mut f64,
    ) {
        match self.get_recipe(item, overrides) {
            Some(recipe) => {
                let machines_needed = rate / recipe.output_rate;
                let power = machines_needed * recipe.power_cost;
                *total_power += power;

                nodes.push(ProductionNode {
                    item: item.to_string(),
                    rate,
                    machines_needed,
                    machine_name: recipe.machine_name.clone(),
                    power_cost: power,
                    recipe_name: recipe.recipe_name.clone(),
                });

                // Track byproducts from this recipe
                for (bp_item, bp_rate_per_machine) in &recipe.byproducts {
                    let bp_total_rate = bp_rate_per_machine * machines_needed;
                    byproducts.push(ByproductNode {
                        item: bp_item.clone(),
                        rate: bp_total_rate,
                        source_item: item.to_string(),
                        source_machine: recipe.machine_name.clone(),
                    });
                }

                // If this recipe has no inputs, it's a raw resource
                if recipe.required_inputs.is_empty() {
                    *raw_resources.entry(item.to_string()).or_insert(0.0) += rate;
                }

                // Recurse into each input
                for (input_item, input_rate_per_machine) in &recipe.required_inputs {
                    let required_rate = input_rate_per_machine * machines_needed;
                    self.walk_tree(input_item, required_rate, overrides, nodes, byproducts, raw_resources, total_power);
                }
            }
            None => {
                // No recipe found → this is a raw resource
                *raw_resources.entry(item.to_string()).or_insert(0.0) += rate;
            }
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> RecipeDatabase {
        RecipeDatabase::new()
    }

    #[test]
    fn items_is_sorted_and_nonempty() {
        let items = db().items();
        assert!(!items.is_empty(), "item list should not be empty");
        let mut sorted = items.clone();
        sorted.sort();
        assert_eq!(items, sorted, "items() should return a sorted list");
        assert!(items.contains(&"Rotor".to_string()));
    }

    #[test]
    fn default_recipe_is_not_an_alternate() {
        let r = db().default_recipe("Iron Ingot").expect("Iron Ingot has a default recipe");
        assert!(!r.is_alternate());
        assert_eq!(r.recipe_name, "Iron Ingot");
    }

    #[test]
    fn rotor_calculation_has_power_and_raw_ores() {
        let result = db().calculate_requirements("Rotor", 10.0, &HashMap::new());
        assert_eq!(result.target_item, "Rotor");
        assert_eq!(result.target_rate, 10.0);
        assert!(!result.nodes.is_empty(), "should produce production nodes");
        assert!(result.total_power > 0.0, "total power should be positive");
        // The standard Rotor chain bottoms out at Iron Ore.
        assert!(result.raw_resources.contains_key("Iron Ore"));
        let iron_ore = result.raw_resources["Iron Ore"];
        assert!(iron_ore > 0.0);
        // No byproducts in the standard Rotor chain.
        assert!(result.byproducts.is_empty());
    }

    #[test]
    fn alternate_override_changes_the_tree() {
        let d = db();
        let standard = d.calculate_requirements("Rotor", 10.0, &HashMap::new());

        let mut overrides = HashMap::new();
        overrides.insert("Screws".to_string(), "Alternate: Cast Screws".to_string());
        let overridden = d.calculate_requirements("Rotor", 10.0, &overrides);

        // The Screws node should now use the alternate recipe.
        let screws = overridden
            .nodes
            .iter()
            .filter(|n| n.item == "Screws")
            .collect::<Vec<_>>();
        assert_eq!(screws.len(), 1);
        assert_eq!(screws[0].recipe_name, "Alternate: Cast Screws");

        // Cast Screws skips the Iron Rod step, so there is one fewer node
        // and the total power differs.
        assert_eq!(standard.nodes.len(), 8);
        assert_eq!(overridden.nodes.len(), 7);
        assert!((overridden.total_power - standard.total_power).abs() > 1e-6);
    }

    #[test]
    fn byproducts_are_recorded() {
        let result = db().calculate_requirements("Heavy Oil Residue", 40.0, &HashMap::new());
        assert_eq!(result.byproducts.len(), 1);
        let bp = &result.byproducts[0];
        assert_eq!(bp.item, "Polymer Resin");
        assert!((bp.rate - 30.0).abs() < 1e-6, "Polymer Resin rate should be 30");
        assert_eq!(bp.source_item, "Heavy Oil Residue");
        assert_eq!(bp.source_machine, "Refinery");
        // Crude Oil is the raw input.
        assert!(result.raw_resources.contains_key("Crude Oil"));
    }

    #[test]
    fn all_alternates_only_lists_items_with_multiple_recipes() {
        let all = db().all_alternates();
        assert!(!all.is_empty());
        for opts in all.values() {
            assert!(opts.len() > 1, "every item in all_alternates needs >1 recipe");
        }
        // Rotor has standard + Copper Rotor + Steel Rotor.
        assert!(all.contains_key("Rotor"));
        assert_eq!(all["Rotor"].len(), 3);
    }
}