use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Data Structures ───────────────────────────────────────────────

/// Defines how an item is produced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub output_item: String,
    pub output_rate: f64,                        // items per minute per machine
    pub machine_name: String,                    // e.g. "Constructor", "Assembler"
    pub power_cost: f64,                         // MW per machine
    pub required_inputs: HashMap<String, f64>,   // item name -> items/min required
}

/// A single node in the production tree result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionNode {
    pub item: String,
    pub rate: f64,
    pub machines_needed: f64,
    pub machine_name: String,
    pub power_cost: f64,
}

/// The full result of a calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub target_item: String,
    pub target_rate: f64,
    pub nodes: Vec<ProductionNode>,
    pub total_power: f64,
    pub raw_resources: HashMap<String, f64>,
}

/// The incoming request from the frontend.
#[derive(Debug, Clone, Deserialize)]
pub struct CalculationRequest {
    pub item: String,
    pub rate: f64,
}

/// Helper struct for deserializing the JSON recipe file.
#[derive(Debug, Deserialize)]
struct RecipeFile {
    recipes: Vec<Recipe>,
}

// ─── Recipe Database ──────────────────────────────────────────────

pub struct RecipeDatabase {
    recipes: HashMap<String, Recipe>,
}

impl RecipeDatabase {
    /// Loads recipes from the embedded JSON file.
    pub fn new() -> Self {
        let json_data = include_str!("recipes.json");
        let file: RecipeFile = serde_json::from_str(json_data)
            .expect("Failed to parse recipes.json");

        let mut recipes = HashMap::new();
        for recipe in file.recipes {
            recipes.insert(recipe.output_item.clone(), recipe);
        }

        RecipeDatabase { recipes }
    }

    /// Returns a sorted list of all craftable item names.
    pub fn items(&self) -> Vec<String> {
        let mut items: Vec<String> = self.recipes.keys().cloned().collect();
        items.sort();
        items
    }

    /// Recursively walks the recipe tree backward from the target item
    /// all the way down to raw ores, returning every production node.
    pub fn calculate_requirements(&self, item: &str, rate: f64) -> CalculationResult {
        let mut nodes: Vec<ProductionNode> = Vec::new();
        let mut raw_resources: HashMap<String, f64> = HashMap::new();
        let mut total_power: f64 = 0.0;

        self.walk_tree(item, rate, &mut nodes, &mut raw_resources, &mut total_power);

        CalculationResult {
            target_item: item.to_string(),
            target_rate: rate,
            nodes,
            total_power,
            raw_resources,
        }
    }

    fn walk_tree(
        &self,
        item: &str,
        rate: f64,
        nodes: &mut Vec<ProductionNode>,
        raw_resources: &mut HashMap<String, f64>,
        total_power: &mut f64,
    ) {
        match self.recipes.get(item) {
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
                });

                // If this recipe has no inputs, it's a raw resource
                if recipe.required_inputs.is_empty() {
                    *raw_resources.entry(item.to_string()).or_insert(0.0) += rate;
                }

                // Recurse into each input
                for (input_item, input_rate_per_machine) in &recipe.required_inputs {
                    let required_rate = input_rate_per_machine * machines_needed;
                    self.walk_tree(input_item, required_rate, nodes, raw_resources, total_power);
                }
            }
            None => {
                // No recipe found → this is a raw resource
                *raw_resources.entry(item.to_string()).or_insert(0.0) += rate;
            }
        }
    }
}