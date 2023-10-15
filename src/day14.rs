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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Nanofactory {
    recipes: HashMap<Material, Reaction>,
    ranks: HashMap<Material, usize>,
    max_requirements: HashMap<Material, usize>,
}

impl Nanofactory {
    fn new(content: &String) -> Option<Nanofactory> {
        let mut recipes = HashMap::new();
        let mut max_requirements = HashMap::new();
        for line in content.lines() {
            let reaction = Reaction::parse(&line.to_string())?;
            let output_material = reaction.output.clone();
            for (material, amount) in reaction.inputs.clone().iter() {
                max_requirements.insert(material.clone(), *amount.max(max_requirements.get(material).unwrap_or(&0)));
            }
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
            max_requirements,
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

    fn calculate_ore_needed_for_1_fuel(&self) -> usize {
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

    fn calculate_maximum_fuel_for_1_trillion_ore(&self) -> usize {
        let search_state = SearchState::new(self, 1000000000000);
        search_state.search()
    }
}

#[derive(Clone)]
struct SearchState<'a> {
    available_materials: HashMap<Material, usize>,
    factory: &'a Nanofactory,
}

impl<'a> SearchState<'a> {
    fn new(factory: &'a Nanofactory, initial_ore: usize) -> SearchState {
        SearchState {
            available_materials: HashMap::from([(Material::Ore, initial_ore)]),
            factory,
        }
    }

    fn can_make(&self, material: &Material) -> bool {
        if *material == Material::Ore {
            return false;
        }
        let recipe = self.factory.recipes.get(material).unwrap();
        for (precursor_material, amount) in recipe.inputs.iter() {
            if self.available_materials.get(precursor_material).unwrap_or(&0) < amount {
                return false;
            }
        }
        
        true
    }

    fn is_needed(&self, material: &Material) -> bool {
        if *material == Material::Fuel {
            return true;
        }

        let current_level = self.available_materials.get(material).unwrap_or(&0);

        current_level < self.factory.max_requirements.get(material).unwrap_or(&0)
    }

    fn make(&self, material: &Material) -> SearchState<'a> {
        let mut new_materials = self.available_materials.clone();
        let recipe = self.factory.recipes.get(material).unwrap();
        for (precursor_material, amount) in recipe.inputs.iter() {
            new_materials.insert(precursor_material.clone(), new_materials.get(&precursor_material).unwrap() - amount);
        }
        new_materials.insert(material.clone(), new_materials.get(material).unwrap_or(&0) + recipe.output_multiplier);

        SearchState {
            available_materials: new_materials,
            factory: self.factory,
        }
    }

    fn search(&self) -> usize {
        let mut best_score = *self.available_materials.get(&Material::Fuel).unwrap_or(&0);
        let mut states = Vec::new();
        states.push(self.clone());
        while states.len() > 0 {
            let current = states.pop().unwrap();
            let mut can_make_something = false;
            for material in self.factory.recipes.keys() {
                if !current.can_make(material) {
                    continue;
                }
                if !current.is_needed(material) {
                    continue;
                }
                can_make_something = true;
                let child = current.make(material);
                states.push(child);
            }
            if !can_make_something {
                best_score = best_score.max(*self.available_materials.get(&Material::Fuel).unwrap_or(&0));
            }
        }
        

        best_score
    }
}

fn part1(recipe_description: &String) {
    let factory = Nanofactory::new(recipe_description).unwrap();
    println!("Part 1: {} ore needed", factory.calculate_ore_needed_for_1_fuel());
}

fn part2(recipe_description: &String) {
    let factory = Nanofactory::new(recipe_description).unwrap();
    //println!("Part 2: {} fuel produced with one trillion ore", factory..calculate_maximum_fuel_for_1_trillion_ore());
    println!("Part 2: Not implemented");
}

pub fn execute() {
    let content = read_content(&"data/day14.txt".to_string());
    part1(&content);
    part2(&content);
}


#[cfg(test)]
mod tests {
    use crate::day14::*;

    fn factory1() -> Nanofactory{
        Nanofactory::new(&"157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_string()).unwrap()
    }

    fn factory2() -> Nanofactory{
        Nanofactory::new(&"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF".to_string()).unwrap()
    }

    fn factory3() -> Nanofactory{
        Nanofactory::new(&"171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX".to_string()).unwrap()
    }

    #[test]
    fn test_calculate_ore_needed_for_1_fuel() {
        assert_eq!(factory1().calculate_ore_needed_for_1_fuel(), 13312);
        assert_eq!(factory2().calculate_ore_needed_for_1_fuel(), 180697);
        assert_eq!(factory3().calculate_ore_needed_for_1_fuel(), 2210736);
    }

    // This test is disabled since it doesn't work yet
    //#[test]
    fn test_calculate_fuel_from_1_trillion_ore() {
        assert_eq!(factory1().calculate_maximum_fuel_for_1_trillion_ore(), 82892753);
        assert_eq!(factory2().calculate_maximum_fuel_for_1_trillion_ore(), 5586022);
        assert_eq!(factory3().calculate_maximum_fuel_for_1_trillion_ore(), 460664);
    }
}
