use std::collections::HashMap;

#[derive(Debug)]
struct UniversalOrbitMap {
    relationships: HashMap<String, String>,
}

fn parse_line(line: String) -> (String, String) {
    let parts: Vec<_> = line.split(")").collect();
    return (parts[0].trim().to_string(), parts[1].trim().to_string());
}

fn parse_map(content: String) -> UniversalOrbitMap {
    let mut map = UniversalOrbitMap {
        relationships: HashMap::new(),
    };
    for line in content.lines() {
        let (parent, child) = parse_line(line.to_string());
        map.relationships.insert(child, parent);
    }

    return map;
}

fn traverse_map(map: &UniversalOrbitMap, source: String) -> Vec<String> {
    let mut output = Vec::new();
    let mut current = source;
    while current != "COM" {
        current = map.relationships.get(&current).unwrap().to_string();
        output.push(current.clone());
    }

    output
}

fn calculate_shortest_route(map: &UniversalOrbitMap, a: String, b: String) -> i32 {
    let mut from_a_to_com = traverse_map(map, a);
    let mut from_b_to_com = traverse_map(map, b);
    while from_a_to_com[(from_a_to_com.len() - 1) as usize]
        == from_b_to_com[(from_b_to_com.len() - 1) as usize]
    {
        from_a_to_com.pop();
        from_b_to_com.pop();
    }

    (from_a_to_com.len() + from_b_to_com.len()) as i32
}

fn calculate_orbits(map: &UniversalOrbitMap, object: String) -> i32 {
    let mut current = object;
    let mut orbits = 0;
    while current != "COM" {
        current = map.relationships.get(&current).unwrap().to_string();
        orbits += 1;
    }

    orbits
}

fn calculate_checksum(map: &UniversalOrbitMap) -> i32 {
    let mut checksum = 0;
    for (key, _) in map.relationships.iter() {
        checksum += calculate_orbits(map, key.to_string());
    }

    checksum
}

pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day06.txt"));
    let map = parse_map(content);
    println!("Part 1: Checksum {}", calculate_checksum(&map));
    println!(
        "Part 2: Shortest distance between YOU and SAN {}",
        calculate_shortest_route(&map, "YOU".to_string(), "SAN".to_string())
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_checksum() {
        let map = crate::day06::parse_map(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n".to_string(),
        );
        assert_eq!(crate::day06::calculate_checksum(&map), 42);
    }

    #[test]
    fn test_calculate_shortest_route() {
        let map = crate::day06::parse_map(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN".to_string(),
        );
        assert_eq!(
            crate::day06::calculate_shortest_route(&map, "YOU".to_string(), "SAN".to_string()),
            4
        );
    }
}
