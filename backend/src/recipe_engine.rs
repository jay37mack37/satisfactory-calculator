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

// ─── Recipe Database ──────────────────────────────────────────────

pub struct RecipeDatabase {
    recipes: HashMap<String, Recipe>,
}

impl RecipeDatabase {
    pub fn new() -> Self {
        let mut recipes = HashMap::new();

        // ── Raw Resources (no inputs) ─────────────────────────
        recipes.insert("Iron Ore".to_string(), Recipe {
            output_item: "Iron Ore".to_string(),
            output_rate: 30.0,
            machine_name: "Miner Mk.1".to_string(),
            power_cost: 5.0,
            required_inputs: HashMap::new(),
        });
        recipes.insert("Copper Ore".to_string(), Recipe {
            output_item: "Copper Ore".to_string(),
            output_rate: 30.0,
            machine_name: "Miner Mk.1".to_string(),
            power_cost: 5.0,
            required_inputs: HashMap::new(),
        });

        // ── Smelted Ingots ────────────────────────────────────
        recipes.insert("Iron Ingot".to_string(), Recipe {
            output_item: "Iron Ingot".to_string(),
            output_rate: 30.0,
            machine_name: "Smelter".to_string(),
            power_cost: 4.0,
            required_inputs: HashMap::from([("Iron Ore".to_string(), 30.0)]),
        });
        recipes.insert("Copper Ingot".to_string(), Recipe {
            output_item: "Copper Ingot".to_string(),
            output_rate: 30.0,
            machine_name: "Smelter".to_string(),
            power_cost: 4.0,
            required_inputs: HashMap::from([("Copper Ore".to_string(), 30.0)]),
        });

        // ── Constructed Parts ─────────────────────────────────
        recipes.insert("Iron Plate".to_string(), Recipe {
            output_item: "Iron Plate".to_string(),
            output_rate: 20.0,
            machine_name: "Constructor".to_string(),
            power_cost: 4.0,
            required_inputs: HashMap::from([("Iron Ingot".to_string(), 30.0)]),
        });
        recipes.insert("Iron Rod".to_string(), Recipe {
            output_item: "Iron Rod".to_string(),
            output_rate: 15.0,
            machine_name: "Constructor".to_string(),
            power_cost: 4.0,
            required_inputs: HashMap::from([("Iron Ingot".to_string(), 15.0)]),
        });
        recipes.insert("Copper Wire".to_string(), Recipe {
            output_item: "Copper Wire".to_string(),
            output_rate: 30.0,
            machine_name: "Constructor".to_string(),
            power_cost: 4.0,
            required_inputs: HashMap::from([("Copper Ingot".to_string(), 15.0)]),
        });

        // ── Assembled Parts ───────────────────────────────────
        recipes.insert("Rotor".to_string(), Recipe {
            output_item: "Rotor".to_string(),
            output_rate: 4.0,
            machine_name: "Assembler".to_string(),
            power_cost: 15.0,
            required_inputs: HashMap::from([
                ("Iron Rod".to_string(), 12.0),
                ("Copper Wire".to_string(), 16.0),
            ]),
        });
        recipes.insert("Reinforced Iron Plate".to_string(), Recipe {
            output_item: "Reinforced Iron Plate".to_string(),
            output_rate: 3.0,
            machine_name: "Assembler".to_string(),
            power_cost: 15.0,
            required_inputs: HashMap::from([
                ("Iron Plate".to_string(), 18.0),
                ("Iron Rod".to_string(), 12.0),
            ]),
        });

        // ── Manufacturer Parts ───────────────────────────────
        recipes.insert("Modular Frame".to_string(), Recipe {
            output_item: "Modular Frame".to_string(),
            output_rate: 2.0,
            machine_name: "Manufacturer".to_string(),
            power_cost: 55.0,
            required_inputs: HashMap::from([
                ("Reinforced Iron Plate".to_string(), 7.5),
                ("Iron Rod".to_string(), 15.0),
            ]),
        });

        RecipeDatabase { recipes }
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