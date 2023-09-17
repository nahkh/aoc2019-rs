use std::collections::HashMap;

#[derive(Debug)]
struct UniversalOrbitMap {
    relationships: HashMap<String, String>
}

fn parse_line(line: String) -> (String, String) {
    let parts: Vec<_> = line.split(")").collect();
    return (parts[0].trim().to_string(), parts[1].trim().to_string())
}

fn parse_map(content: String) -> UniversalOrbitMap {
    let mut map = UniversalOrbitMap { relationships: HashMap::new()};
    for line in content.lines() {
        let (parent, child) = parse_line(line.to_string());
        map.relationships.insert(child, parent);
    }

    return map;
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

fn calculate_checksum(map: UniversalOrbitMap) -> i32 {
    let mut checksum = 0;
    for (key, _) in map.relationships.iter() {
        checksum += calculate_orbits(&map, key.to_string());
    }

    checksum
}

pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day06.txt"));
    let map = parse_map(content);
    println!("Part 1: Checksum {}", calculate_checksum(map));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_checksum() {
        let map = crate::day06::parse_map("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n".to_string());
        assert_eq!(crate::day06::calculate_checksum(map), 42);
    }
}
