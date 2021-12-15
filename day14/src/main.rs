use std::collections::{HashMap, VecDeque};

struct PolymerBuilder {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
    mem10: HashMap<(char, char), (HashMap<char, usize>, VecDeque<char>)>,
    mem20: HashMap<(char, char), HashMap<char, usize>>,
}

impl PolymerBuilder {
    fn from_str(file: &str) -> Self {
        let mut lines = file.lines();
        let p = lines.next().unwrap().chars().collect::<Vec<char>>();
        lines.next();
        let rules = lines
            .map(|l| {
                let fs = l.split(" -> ").collect::<Vec<&str>>();
                let keys = fs[0].chars().collect::<Vec<char>>();
                ((keys[0], keys[1]), fs[1].chars().next().unwrap())
            })
            .collect::<HashMap<(char, char), char>>();

        Self {
            template: p,
            rules: rules,
            mem10: HashMap::new(),
            mem20: HashMap::new(),
        }
    }

    fn pairs(&self) -> Vec<(char, char)> {
        self.rules.keys().cloned().collect()
    }

    fn memoize10(&mut self) {
        for pair in self.pairs() {
            let new_poly = self.grow_polymer([pair.0, pair.1].iter().cloned().collect(), 10);
            self.mem10
                .insert(pair, (self.count_monomers(&new_poly), new_poly));
        }
    }

    fn memoize20(&mut self) {
        self.memoize10();
        for (pair, (_, poly)) in &self.mem10 {
            self.mem20.insert(
                *pair,
                (0..poly.len() - 1).fold(HashMap::new(), |mut sum, ix| {
                    for (k, v) in &self.mem10.get(&(poly[ix], poly[ix + 1])).unwrap().0 {
                        *sum.entry(*k).or_insert(0) += v;
                    }
                    sum
                }),
            );
        }
    }

    fn counts10(&mut self) -> HashMap<char, usize> {
        self.memoize10();
        let mut tot_counts: HashMap<char, usize> = HashMap::new();
        for ix in 0..self.template.len() - 1 {
            let tmpl_pair = (self.template[ix], self.template[ix + 1]);
            self.add_hm(
                &mut tot_counts,
                self.mem10.get(&tmpl_pair).unwrap().0.clone(),
            );
        }
        *tot_counts
            .entry(*self.template.last().unwrap())
            .or_insert(0) += 1;
        tot_counts
    }

    fn counts40(&mut self) -> HashMap<char, usize> {
        self.memoize20();
        let mut tot_counts: HashMap<char, usize> = HashMap::new();
        for ix in 0..self.template.len() - 1 {
            let tmpl_pair = (self.template[ix], self.template[ix + 1]);
            let (_, poly10) = self.mem10.get(&tmpl_pair).unwrap();
            for ix10 in 0..poly10.len() - 1 {
                let poly10pair = (poly10[ix10], poly10[ix10 + 1]);
                let (_, poly20) = self.mem10.get(&poly10pair).unwrap();
                let new_counts = (0..poly20.len() - 1).fold(HashMap::new(), |mut sum, ix| {
                    for (k, v) in self.mem20.get(&(poly20[ix], poly20[ix + 1])).unwrap() {
                        *sum.entry(*k).or_insert(0) += v;
                    }
                    sum
                });
                self.add_hm(&mut tot_counts, new_counts);
            }
        }
        *tot_counts
            .entry(*self.template.last().unwrap())
            .or_insert(0) += 1;
        tot_counts
    }

    fn add_hm(&self, hm1: &mut HashMap<char, usize>, hm2: HashMap<char, usize>) {
        for (k, v) in hm2 {
            *hm1.entry(k).or_insert(0) += v;
        }
    }

    fn apply_rules(&self, mut poly: VecDeque<char>) -> VecDeque<char> {
        let mut new_poly: VecDeque<char> = VecDeque::with_capacity(poly.len());
        let mut curr_mono = poly.pop_front().unwrap();
        new_poly.push_back(curr_mono);
        while let Some(next_mono) = poly.pop_front() {
            if let Some(new_mono) = self.rules.get(&(curr_mono, next_mono)) {
                new_poly.push_back(*new_mono);
            }
            new_poly.push_back(next_mono);
            curr_mono = next_mono;
        }
        new_poly
    }

    fn grow_polymer(&self, mut poly: VecDeque<char>, iterations: usize) -> VecDeque<char> {
        for _ in 0..iterations {
            poly = self.apply_rules(poly);
        }
        poly
    }

    fn count_monomers(&self, poly: &VecDeque<char>) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for ix in 0..poly.len() - 1 {
            *counts.entry(poly[ix]).or_insert(0) += 1;
        }
        counts
    }
}

fn main() {
    println!("{}", solve1(include_str!("../data/input.txt")));
    println!("{}", solve2(include_str!("../data/input.txt")));
}

fn solve1(file: &str) -> usize {
    let mut pb = PolymerBuilder::from_str(file);
    let counts = pb.counts10();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn solve2(file: &str) -> usize {
    let mut pb = PolymerBuilder::from_str(file);
    let counts = pb.counts40();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve1(include_str!("../data/test1.txt")), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(include_str!("../data/test1.txt")), 2188189693529);
    }
}
