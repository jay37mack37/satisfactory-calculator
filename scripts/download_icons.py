"""
Download Satisfactory item icons from the wiki for all items in recipes.json.
Icons are 256x256 PNGs from https://satisfactory.wiki.gg/images/{Item_Name}.png
"""
import json
import urllib.request
import urllib.error
import os
import time
import sys

RECIPES_PATH = os.path.join(os.path.dirname(
    __file__), '..', 'backend', 'src', 'recipes.json')
ICONS_DIR = os.path.join(os.path.dirname(
    __file__), '..', 'frontend', 'public', 'icons')
MAPPING_PATH = os.path.join(os.path.dirname(
    __file__), '..', 'frontend', 'src', 'icon_map.json')

BASE_URL = "https://satisfactory.wiki.gg/images/{}.png?format=original"

# Manual overrides for items whose wiki filename differs from the item name
OVERRIDES = {
    "Crude Oil": "Crude_Oil",
    "Heavy Oil Residue": "Heavy_Oil_Residue",
    "Alumina Solution": "Alumina_Solution",
    "Sulfuric Acid": "Sulfuric_Acid",
    "Petroleum Coke": "Petroleum_Coke",
    "Polymer Resin": "Polymer_Resin",
    "Raw Quartz": "Raw_Quartz",
    "Caterium Ore": "Caterium_Ore",
    "Copper Ore": "Copper_Ore",
    "Iron Ore": "Iron_Ore",
    "Copper Ingot": "Copper_Ingot",
    "Iron Ingot": "Iron_Ingot",
    "Caterium Ingot": "Caterium_Ingot",
    "Steel Ingot": "Steel_Ingot",
    "Aluminum Ingot": "Aluminum_Ingot",
    "Copper Sheet": "Copper_Sheet",
    "Copper Powder": "Copper_Powder",
    "Iron Plate": "Iron_Plate",
    "Iron Rod": "Iron_Rod",
    "Reinforced Iron Plate": "Reinforced_Iron_Plate",
    "Modular Frame": "Modular_Frame",
    "Heavy Modular Frame": "Heavy_Modular_Frame",
    "Fused Modular Frame": "Fused_Modular_Frame",
    "Encased Industrial Beam": "Encased_Industrial_Beam",
    "Steel Beam": "Steel_Beam",
    "Steel Pipe": "Steel_Pipe",
    "Aluminum Casing": "Aluminum_Casing",
    "Alclad Aluminum Sheet": "Alclad_Aluminum_Sheet",
    "Aluminum Scrap": "Aluminum_Scrap",
    "AI Limiter": "AI_Limiter",
    "High-Speed Connector": "High-Speed_Connector",
    "Circuit Board": "Circuit_Board",
    "Crystal Oscillator": "Crystal_Oscillator",
    "Electromagnetic Control Rod": "Electromagnetic_Control_Rod",
    "Adaptive Control Unit": "Adaptive_Control_Unit",
    "Radio Control Unit": "Radio_Control_Unit",
    "Supercomputer": "Supercomputer",
    "Assembly Director System": "Assembly_Director_System",
    "Magnetic Field Generator": "Magnetic_Field_Generator",
    "Pressure Conversion Cube": "Pressure_Conversion_Cube",
    "Thermal Propulsion Rocket": "Thermal_Propulsion_Rocket",
    "Turbo Motor": "Turbo_Motor",
    "Modular Engine": "Modular_Engine",
    "Smart Plating": "Smart_Plating",
    "Versatile Framework": "Versatile_Framework",
    "Automated Wiring": "Automated_Wiring",
    "Heavy Oil Residue": "Heavy_Oil_Residue",
    "Quartz Crystal": "Quartz_Crystal",
    "Black Powder": "Black_Powder",
    "Nuclear Pasta": "Nuclear_Pasta",
    "Uranium Fuel Rod": "Uranium_Fuel_Rod",
    "Uranium Waste": "Uranium_Waste",
    "Encased Uranium Cell": "Encased_Uranium_Cell",
    "Encased Plutonium Cell": "Encased_Plutonium_Cell",
    "Plutonium Fuel Rod": "Plutonium_Fuel_Rod",
    "Plutonium Pellet": "Plutonium_Pellet",
    "Plutonium Waste": "Plutonium_Waste",
    "Non-Fissile Uranium": "Non-Fissile_Uranium",
    "Heat Sink": "Heat_Sink",
    "Cooling System": "Cooling_System",
    "Fused Modular Frame": "Fused_Modular_Frame",
    "Biochemical Sculptor": "Biochemical_Sculptor",
    "Superposition Oscillator": "Superposition_Oscillator",
    "Neural-Quantum Processor": "Neural-Quantum_Processor",
    "AI Expansion Server": "AI_Expansion_Server",
    "Singularity Cell": "Singularity_Cell",
    "Ballistic Warp Drive": "Ballistic_Warp_Drive",
    "Ficsonium Fuel Rod": "Ficsonium_Fuel_Rod",
    "Dark Matter Crystal": "Dark_Matter_Crystal",
    "Dark Matter Residue": "Dark_Matter",
    "Time Crystal": "Time_Crystal",
    "Excited Photonic Matter": "Excited_Photonic_Matter",
    "Ficsite Ingot": "Ficsite_Ingot",
    "Ficsite Trigon": "Ficsite_Trigon",
    "SAM Fluctuator": "SAM_Fluctuator",
    "Alien Power Matrix": "Alien_Power_Matrix",
    "Alien DNA Capsule": "Alien_DNA_Capsule",
    "Alien Protein": "Alien_Protein",
    "Power Shard": "Power_Shard",
    "Reinforced Steel Plate": "Reinforced_Steel_Plate",
}


def item_to_filename(item_name):
    """Convert item name to wiki filename (spaces -> underscores)."""
    if item_name in OVERRIDES:
        return OVERRIDES[item_name]
    return item_name.replace(" ", "_")


def main():
    # Load recipes
    with open(RECIPES_PATH, 'r', encoding='utf-8') as f:
        data = json.load(f)

    # Collect all unique item names
    items = set()
    for recipe in data['recipes']:
        items.add(recipe['output_item'])
        for inp in recipe.get('required_inputs', {}):
            items.add(inp)
        for bp in recipe.get('byproducts', {}):
            items.add(bp)

    items = sorted(items)
    print(f"Found {len(items)} unique items in recipes.json")

    # Create icons directory
    os.makedirs(ICONS_DIR, exist_ok=True)

    # Download icons
    icon_map = {}
    success = 0
    failed = []

    for item in items:
        filename = item_to_filename(item)
        local_name = filename + ".png"
        local_path = os.path.join(ICONS_DIR, local_name)

        # Skip if already downloaded
        if os.path.exists(local_path):
            icon_map[item] = f"/icons/{local_name}"
            success += 1
            print(f"  ✓ {item} (cached)")
            continue

        url = BASE_URL.format(filename)
        try:
            req = urllib.request.Request(url, headers={
                'User-Agent': 'SatisfactoryCalculator/1.0 (https://github.com/jay37mack37/satisfactory-calculator)'
            })
            with urllib.request.urlopen(req, timeout=15) as resp:
                if resp.status == 200:
                    with open(local_path, 'wb') as f:
                        f.write(resp.read())
                    icon_map[item] = f"/icons/{local_name}"
                    success += 1
                    print(f"  ✓ {item}")
                else:
                    failed.append(item)
                    print(f"  ✗ {item} (HTTP {resp.status})")
        except (urllib.error.HTTPError, urllib.error.URLError, Exception) as e:
            failed.append(item)
            print(f"  ✗ {item} ({e})")

        time.sleep(0.3)  # Be polite to the wiki

    # Save icon mapping
    with open(MAPPING_PATH, 'w', encoding='utf-8') as f:
        json.dump(icon_map, f, indent=2, ensure_ascii=False)

    print(f"\n{'='*50}")
    print(f"Downloaded: {success}/{len(items)}")
    if failed:
        print(f"Failed: {failed}")
    print(f"Icon map saved to: {MAPPING_PATH}")
    print(f"Icons saved to: {ICONS_DIR}")


if __name__ == '__main__':
    main()
