use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

struct CaveSys {
    edges: HashMap<String, Vec<String>>,
}

impl CaveSys {
    fn from_str(input: &str) -> Self {
        let mut res = Self {
            edges: HashMap::new(),
        };
        input.lines().for_each(|l| {
            let mut fs = l.split('-');
            let a = fs.next().unwrap().to_string();
            let b = fs.next().unwrap().to_string();
            res.edges
                .entry(a.clone())
                .or_insert(Vec::new())
                .push(b.clone());
            res.edges.entry(b).or_insert(Vec::new()).push(a);
        });
        res
    }

    fn get(&self, key: &String) -> Option<&Vec<String>> {
        self.edges.get(key)
    }

    fn get_lowercase_caves(&self) -> Vec<String> {
        self.edges
            .keys()
            .filter(|k| k.chars().next().unwrap().is_lowercase() && *k != "start" && *k != "end")
            .cloned()
            .collect::<Vec<String>>()
    }
}

fn find_paths(
    caves: &CaveSys,
    path: Vec<String>,
    mut blocked_caves: HashSet<String>,
) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let curr_cave = path.last().unwrap();
    if curr_cave == "end" {
        paths.push(path);
        return paths;
    }
    if curr_cave.chars().next().unwrap().is_lowercase() {
        blocked_caves.insert(curr_cave.clone());
    }
    for next in caves.get(curr_cave).unwrap() {
        if blocked_caves.contains(next) {
            continue;
        }
        for ext_path in find_paths(caves, vec![next.clone()], blocked_caves.clone()) {
            paths.push(ext_path);
        }
    }
    paths
}

fn find_paths_double_small(
    caves: &CaveSys,
    path: Vec<String>,
    mut blocked_caves: HashSet<String>,
    special_cave: String,
    mut special_cave_count: usize,
) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let curr_cave = path.last().unwrap();
    if curr_cave == "end" {
        paths.push(path);
        return paths;
    }
    if *curr_cave == special_cave {
        special_cave_count += 1;
    }
    if curr_cave.chars().next().unwrap().is_lowercase()
        && (special_cave_count == 2 || *curr_cave != special_cave)
    {
        blocked_caves.insert(curr_cave.clone());
    }
    for next in caves.get(curr_cave).unwrap() {
        if blocked_caves.contains(next) {
            continue;
        }
        for mut suffix in find_paths_double_small(
            caves,
            vec![next.clone()],
            blocked_caves.clone(),
            special_cave.clone(),
            special_cave_count,
        ) {
            let mut new_path = path.clone();
            new_path.append(&mut suffix);
            paths.push(new_path);
        }
    }
    paths
}

fn solve1(file: &str) -> usize {
    let caves = CaveSys::from_str(file);
    find_paths(
        &caves,
        vec!["start".to_string()],
        vec!["start".to_string()]
            .into_iter()
            .collect::<HashSet<String>>(),
    )
    .len()
}

fn solve2(file: &str) -> usize {
    let caves = CaveSys::from_str(file);
    let mut paths: HashSet<Vec<String>> = HashSet::new();
    for special_cave in caves.get_lowercase_caves() {
        for path in find_paths_double_small(
            &caves,
            vec!["start".to_string()],
            vec!["start".to_string()]
                .into_iter()
                .collect::<HashSet<String>>(),
            special_cave,
            0,
        ) {
            paths.insert(path);
        }
    }
    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 10);
        assert_eq!(solve1(include_str!("../data/test2.txt")), 19);
        assert_eq!(solve1(include_str!("../data/test3.txt")), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 36);
        assert_eq!(solve2(include_str!("../data/test2.txt")), 103);
        assert_eq!(solve2(include_str!("../data/test3.txt")), 3509);
    }
}
