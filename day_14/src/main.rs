use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

extern crate num_bigint;

use num_bigint::BigInt;
use num_bigint::ToBigInt;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Ingredient<'a> {
    name : &'a str,
    quantity : i128,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Recipe<'a> {
    i : Vec<Box<Ingredient<'a>>>,
    serves : i128,
}

fn solve1(recipes : &HashMap<&str, Recipe>) {
    let mut need = 0;
    let mut storage : HashMap<&str, i128> = HashMap::new();
    let need = solve(recipes, "FUEL", 1, &mut storage);
    // println!("need ORE {}\nde sobre {:?}", need, storage);
}

fn solve<'a>(recipes: &'a HashMap<&'a str, Recipe<'a>>, ingredient : &'a str, quantity: i128, storage: &mut HashMap<&'a str, i128>) -> i128 {
    if ingredient == "ORE" {
        return quantity;
    }
    let mut need = quantity;
    if let Some(x) = storage.get_mut(&ingredient) {
        let mut from_storage = *x;
        if from_storage > need {
            from_storage = need;
        }
        *x -= from_storage;
        need -= from_storage;
        // println!("Served {} x {} from storage", from_storage, ingredient);
    }
    let mut ore_need = 0;
    let recipe = recipes.get(ingredient).unwrap();
    let mul = (1.0*(need  as f64)/ (recipe.serves as f64)).ceil() as i128;
    // println!("need {} {} , using {} x {}; using {} x recipe {:?}", need, ingredient, mul, ingredient, mul, recipe);
    for i in &recipe.i {
        let tot = solve(recipes, i.name, mul*i.quantity, storage);
        ore_need += tot;
    }
    if mul*recipe.serves > need {
        // println!("left with {} x {} de sobre", mul*recipe.serves - need, ingredient);
        if let Some(x) = storage.get_mut(&ingredient) {
            *x += mul*recipe.serves - need;
        } else {
            storage.insert(ingredient, mul*recipe.serves - need);
        }
    }
    // println!("ore needed for {} x {} --> {}", mul, ingredient, ore_need);
    return ore_need;
}

fn solve2(recipes : &HashMap<&str, Recipe>) {

    println!("max fuel producable {}", bs(recipes));
}

fn bs(recipes: &HashMap<&str, Recipe>) ->i128 {
    let need = 1e12 as i128;
    let mut lo = 0; let mut mid = -1; let mut hi = 1e12 as i128;

    while lo <= hi {
        mid = (lo+hi)/2; // nema unsigned shift :S
        let mut storage : HashMap<&str, i128> = HashMap::new();
        let v = solve(recipes, "FUEL", mid, &mut storage);
        if v < need {
            lo = mid+1;
        } else if (v > need) {
            hi = mid-1;
        } else {
            return mid;
        }
    }
    return mid - 1;
}

fn parse(contents : &str) -> HashMap<&str, Recipe> {
    let mut state :HashMap<&str, Recipe> = HashMap::new();
    let lines : Vec<&str> = contents.split("\n").collect();
    for line in lines {
        let parts : Vec<&str>= line.split("=>").collect();
        let ins : Vec<&str>= parts[0].split(", ").collect();
        let right_parts : Vec<&str>= parts[1].split(" ").collect();

        let key = right_parts[2];
        for ing in ins {
            let parts : Vec<&str>= ing.split(" ").collect();
            let val = Box::new(Ingredient{quantity: parts[0].parse::<i128>().unwrap_or(-1), name: parts[1]});
            if let Some(x) = state.get_mut(&key) {
                x.i.push(val);
            } else {
                state.insert(key, Recipe{serves: right_parts[1].parse::<i128>().unwrap_or(-1), i:vec![val]});
            }
        }
    }
    return state;
}

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let test0 = fs::read_to_string("test0")
        .expect("Something went wrong reading the file");
    let test01 = fs::read_to_string("test01")
        .expect("Something went wrong reading the file");
    let test1 = fs::read_to_string("test1")
        .expect("Something went wrong reading the file");
    let test2 = fs::read_to_string("test2")
        .expect("Something went wrong reading the file");
    let test3 = fs::read_to_string("test3")
        .expect("Something went wrong reading the file");

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&parse(&contents)),
        "b" => solve2(&parse(&contents)),
        "test0" => solve1(&parse(&test0)),
        "test01" => solve1(&parse(&test01)),
        "test1" => solve1(&parse(&test1)),
        "test2" => solve1(&parse(&test2)),
        "test3" => solve1(&parse(&test3)),
        _ => println!("bezi bre a ili b"),
    };
}
