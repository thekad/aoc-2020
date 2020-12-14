use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bag {
    color: String,
    contains: Vec<Bag>,
    quantity: i32,
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
                let bqty = bcaps.name("qty").map_or("", |m| m.as_str());
                bags.push(Bag {
                    color: bcolor.trim().to_string(),
                    contains: vec![],
                    quantity: bqty.parse().unwrap(),
                });
            }
        }

        Ok(Bag {
            color: color.trim().to_string(),
            contains: bags,
            quantity: 1,
        })
    }
}

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let mut bags: HashMap<String, HashSet<Bag>> = HashMap::new();
    let mut containers: HashMap<String, HashSet<String>> = HashMap::new();
    if let Ok(lines) = crate::io::read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let bag = Bag::from_str(line.as_str()).unwrap();
                for bbag in bag.contains {
                    bags.entry(bag.color.to_string())
                        .or_insert(HashSet::new())
                        .insert(bbag.clone());
                    containers
                        .entry(bbag.color.to_string())
                        .or_insert(HashSet::new())
                        .insert(bag.color.to_string());
                }
            }
        }
        let key = "shiny gold";
        let mut my_containers = find_my_containers(key.to_string(), &containers);
        my_containers.sort();
        my_containers.dedup();
        dbg!(&my_containers);
        dbg!(&my_containers.len());
        //dbg!(bags.get(key));
        dbg!(sum_my_bags(key.to_string(), &bags));
        println!();
    }

    Ok(())
}

fn find_my_containers(bag: String, contained_by: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut containers: Vec<String> = Vec::new();
    let immediate_containers = contained_by.get(&bag);
    match immediate_containers {
        Some(x) => {
            for container in x {
                containers.push(container.to_string());
                let mut my_containers = find_my_containers(container.to_string(), contained_by);
                containers.append(&mut my_containers);
            }
        }
        None => {
            return vec![bag];
        }
    }

    containers
}

fn sum_my_bags(bag: String, bags: &HashMap<String, HashSet<Bag>>) -> i32 {
    let mut bag_count = 0;

    if let Some(my_bags) = bags.get(&bag) {
        println!("{} contain: ", bag);
        for b in my_bags {
            bag_count += b.quantity;
            bag_count += b.quantity * sum_my_bags(b.color.to_string(), bags);
        }
    } else {
        print!("{} contains no other bags", bag);
    }
    println!();

    bag_count
}
