use crate::input_files::read_content;
use std::collections::HashMap;
use regex::Regex;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum Material {
    Ore,
    Fuel,
    Intermediate(String)
}

impl Material {
    fn parse(content: &String) -> Material {
        if content == "ORE" {
            Material::Ore
        } else if content == "FUEL" {
            Material::Fuel
        } else {
            Material::Intermediate(content.clone())
        }
    }
}

#[derive(Debug)]
struct Reaction {
    inputs: HashMap<Material, usize>,
    output: Material,
    output_multiplier: usize,
}

impl Reaction {
    fn parse(content: &String) -> Option<Reaction> {
        let main_parts: Vec<&str> = content.trim().split(" => ").collect();
        assert!(main_parts.len() == 2, "Could not split '{}'", content);
        let mut inputs = HashMap::new();
        let re = Regex::new(r"^(\d+) (\w+)$").unwrap();
        for input in main_parts.get(0)?.split(", ") {
            let matched = re.captures(input)?;
            let count = matched[1].parse::<usize>().ok()?;
            let material = Material::parse(&matched[2].to_string());
            inputs.insert(material, count);
        }
        let matched = re.captures(main_parts.get(1)?)?;
        let count = matched[1].parse::<usize>().ok()?;
        let material = Material::parse(&matched[2].to_string());
        
        Some(Reaction {
            inputs: inputs,
            output: material,
            output_multiplier: count,
        })
    }
}

#[derive(Debug)]
struct Nanofactory {
    recipes: HashMap<Material, Reaction>,
    ranks: HashMap<Material, usize>,
}

impl Nanofactory {
    fn new(content: &String) -> Option<Nanofactory> {
        let mut recipes = HashMap::new();
        for line in content.lines() {
            let reaction = Reaction::parse(&line.to_string())?;
            let output_material = reaction.output.clone();
            recipes.insert(output_material, reaction);
        }
        let mut ranks = HashMap::new();
        ranks.insert(Material::Ore, 0);

        fn determine_rank(material: &Material, recipes: &HashMap<Material, Reaction>, ranks: &mut HashMap<Material, usize>) -> usize {
            if ranks.contains_key(material) {
                return *ranks.get(material).unwrap();
            }
            let recipe = recipes.get(material).unwrap();
            let mut max_precursor_rank = 0;
            for precursor_material in recipe.inputs.clone().keys() {
                max_precursor_rank = max_precursor_rank.max(determine_rank(precursor_material, recipes, ranks));
            }
            ranks.insert(material.clone(), max_precursor_rank + 1);
            return max_precursor_rank + 1;
        }

        determine_rank(&Material::Fuel, &recipes, &mut ranks);

        Some(Nanofactory {
            recipes,
            ranks,
        })
    }

    fn find_highest_rank_material(&self, materials: &HashMap<Material, usize>) -> Option<Material> {
        let mut highest_rank_material = None;
        let mut highest_rank = 0;
        for material in materials.keys() {
            let rank = self.ranks.get(material)?;
            if rank > &highest_rank {
                highest_rank_material = Some(material.clone());
                highest_rank = *rank;
            }
        }
        
        highest_rank_material
    }

    fn calculate_fuel(&self) -> usize {
        let mut needed_materials = HashMap::from([(Material::Fuel, 1)]);
        
        while needed_materials.len() > 1 || !needed_materials.contains_key(&Material::Ore) {
            let material = self.find_highest_rank_material(&needed_materials).unwrap();
            let amount_needed = needed_materials.get(&material).unwrap();
            let reaction = self.recipes.get(&material).unwrap();
            let reaction_multiplier = (amount_needed + reaction.output_multiplier - 1) / reaction.output_multiplier;
            for (input_material, input_amount) in reaction.inputs.iter() {
                needed_materials.insert(input_material.clone(), needed_materials.get(&input_material).unwrap_or(&0) + input_amount * reaction_multiplier);
            }
            needed_materials.remove(&material);
        }
        return *needed_materials.get(&Material::Ore).unwrap();
    }
}



fn part1(recipe_description: &String) {
    let factory = Nanofactory::new(recipe_description).unwrap();
    println!("Part 1: {} ore needed", factory.calculate_fuel());
}

pub fn execute() {
    let content = read_content(&"data/day14.txt".to_string());
    part1(&content);
}


#[cfg(test)]
mod tests {
    use crate::day14::*;

    #[test]
    fn test_calculate_fuel() {
        let factory = Nanofactory::new(&"157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_string()).unwrap();
        assert_eq!(factory.calculate_fuel(), 13312);
    }
}
