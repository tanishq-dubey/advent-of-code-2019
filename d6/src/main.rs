//#[derive(Default)]
//#[derive(Debug)]
//struct Tree {
//    root: String,
//    left: Option<Box<Tree>>,
//    right: Option<Box<Tree>>,
//}
//
//// Maybe don't use this....
//impl Tree {
//    fn new(root: String) -> Tree {
//        Tree {
//            root,
//            ..Default::default()
//        }
//    }
//
//    fn left(mut self, leaf: Tree) -> Self {
//        self.left = Some(Box::new(leaf));
//        self
//    }
//
//    fn right(mut self, leaf: Tree) -> Self {
//        self.right = Some(Box::new(leaf));
//        self
//    }
//}
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::VecDeque;

fn orbit_count(planet: String, orbits: &HashMap<String, Vec<String>>) -> i32 {
    let mut tmp = planet.clone();
    let mut distance = 0;
    while tmp != "COM" {
        for key in orbits.keys() {
            match orbits.get(key) {
                Some(val) => {
                    if val.contains(&tmp) {
                        tmp = key.clone();
                        distance = distance + 1;
                    }
                }
                None => {
                    panic!("How did we end up here")
                }
            };
        }
    }
    return distance;
}

fn distance_between(start: String, end: String, orbits: &HashMap<String, Vec<String>>) -> i32 {
    let mut distance = 0;
    let mut visited: Vec<String> = vec![];
    let mut frontier: VecDeque<String> = VecDeque::new();
    frontier.push_back(start);
    loop {
        if frontier.contains(&end) {
            return distance;
        }
        let mut tmp_frontier: VecDeque<String> = VecDeque::new();
        for f in frontier {
            visited.push(f.clone());
            match orbits.get(&f) {
                Some(value) => {
                    for v in value {
                        if !visited.contains(v) {
                            tmp_frontier.push_back(v.to_string());
                        }
                    }
                }
                None => {
                    panic!("How did we end up here")
                }
            }
            for key in orbits.keys() {
                match orbits.get(key) {
                    Some(k) => {
                        if k.contains(&f) && !visited.contains(key) {
                            tmp_frontier.push_back(key.to_string());
                        }
                    }
                    None => {
                        panic!("How did we end up here")
                    }
                }
            }
        }
        distance = distance + 1;
        frontier = tmp_frontier;
    }
}


fn triangle_number(n: u64) -> u64 {
    (1..=n).sum()
}

fn main() {
    let mut orbits: std::collections::HashMap<String, Vec<String>> = HashMap::new();


    let file = File::open("./input");
    let file = match file {
        Ok(f) => f,
        Err(err) => {
            panic!("There was an issue opening the file: {:?}", err);
        }
    };
    let reader = BufReader::new(file);

    // Build structure
    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(err) => {
                panic!("Could not read line at {}: {:?}", index, err);
            }
        };
        let planets = line.split(")").collect::<Vec<&str>>();
        orbits.entry(planets[1].to_string()).or_insert(vec![]);
        orbits.entry(planets[0].to_string()).or_insert(vec![]).push(planets[1].to_string());
    }

    let mut total_dist = 0;
    for key in orbits.keys() {
        match orbits.get(key) {
            Some(v) => {
                if v.len() == 0{
                    let single_dist = orbit_count(key.to_string(), &orbits);
                    let o_count = triangle_number(single_dist as u64);
                    total_dist = total_dist + o_count;
                    println!("OC: {:?}", o_count);
                }
            }
            None => {
                panic!("How did we end up here")

            }
        }
        //let dist = orbit_count(key.to_string(), &orbits);
        //total_dist = total_dist + dist;
        //println!("{:?}: {:?}", key, dist);
    }
    println!("{:?}", total_dist);
    let dist = distance_between("YOU".to_string(), "SAN".to_string(), &orbits);
    println!("{:?}", dist - 2);


    //println!("{:?}", total_dist);
}
