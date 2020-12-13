use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bag {
    color: String,
    contains: Vec<Bag>,
}

impl FromStr for Bag {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BAGS_RE: Regex =
                Regex::new(r"^(?P<bag>(\w\s?)+){1}bags contain (?P<bags>.*).$").unwrap();
            static ref BAG_RE: Regex =
                Regex::new(r"^(?P<qty>\d+)\s(?P<color>(\w\s?)+)bags?$").unwrap();
        }

        let caps = BAGS_RE.captures(s).unwrap();
        let color = caps.name("bag").map_or("", |m| m.as_str());
        let contains = caps.name("bags").map_or("", |m| m.as_str());
        let mut bags: Vec<Bag> = Vec::new();
        if contains != "no other bags" {
            for b in contains.split(", ") {
                let bcaps = BAG_RE.captures(b).unwrap();
                let bcolor = bcaps.name("color").map_or("", |m| m.as_str());
                bags.push(Bag {
                    color: bcolor.trim().to_string(),
                    contains: vec![],
                });
            }
        }

        Ok(Bag {
            color: color.trim().to_string(),
            contains: bags,
        })
    }
}

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let mut contains: HashMap<String, HashSet<String>> = HashMap::new();
    let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();
    if let Ok(lines) = crate::io::read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let bag = Bag::from_str(line.as_str()).unwrap();
                for bbag in bag.contains {
                    contains
                        .entry(bag.color.to_string())
                        .or_insert(HashSet::new())
                        .insert(bbag.color.to_string());
                    contained_by
                        .entry(bbag.color.to_string())
                        .or_insert(HashSet::new())
                        .insert(bag.color.to_string());
                }
            }
        }
        let key = "shiny gold".to_string();
        let mut my_containers = find_containers(key, &contained_by);
        my_containers.sort();
        my_containers.dedup();
        dbg!(&my_containers);
        dbg!(&my_containers.len());
    }

    Ok(())
}

fn find_containers(bag: String, contained_by: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut containers: Vec<String> = Vec::new();
    let immediate_containers = contained_by.get(&bag);
    match immediate_containers {
        Some(x) => {
            for container in x {
                containers.push(container.to_string());
                let mut my_containers = find_containers(container.to_string(), contained_by);
                containers.append(&mut my_containers);
            }
        }
        None => {
            return vec![bag];
        }
    }

    containers
}
