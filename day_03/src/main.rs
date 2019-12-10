use std::fs;
use std::collections::HashSet;
use std::env;
use std::hash::{Hash, Hasher};

// Point struct that compares and hashes on only (x, y).
struct Point {
  x: i128,
  y: i128,
  s: i128,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn cable(input : &str) -> Vec<Point> {
  let pieces :Vec<&str> = input.split(",").collect();
  let mut result : Vec<Point> = Vec::new();
  let mut x = 0;
  let mut y = 0;
  let mut s = 0;
  for i in 0..pieces.len()-1 {
      let direction = &pieces[i][0..1];
      let count :i128 = pieces[i][1..].parse().unwrap();

      match direction {
          "R" => {for i in 0..count {x+=1; result.push(Point{x, y, s:s+i+1}); } },
          "D" => {for i in 0..count {y-=1; result.push(Point{x, y, s:s+i+1}); } },
          "L" => {for i in 0..count {x-=1; result.push(Point{x, y, s:s+i+1}); } },
          "U" => {for i in 0..count {y+=1; result.push(Point{x, y, s:s+i+1}); } },
          _ => println!("error unknonwn direction {}", direction),
      }
      s += count;
  }
  return result;
}

fn manhattan(p1 : &Point) -> i128 {
    return i128::abs(p1.x) + i128::abs(p1.y);
}

fn solve1(first: &Vec<Point>, second: &Vec<Point>) -> i128 {
    let first_set : HashSet<&Point> = first.into_iter().collect();

    let mut min = std::i128::MAX;
    for i in second {
        if first_set.contains(i) {
            let d = manhattan(i);
            if d < min {
                min = d;
            }
        }
    }
    return min;
}

fn solve2(first: &Vec<Point>, second: &Vec<Point>) -> i128 {
    let mut first_set : HashSet<&Point> = HashSet::new();
    // Keep only the point with the smallest step count in the hash.
    for i in first {
        if first_set.contains(i) && first_set.get(i).unwrap().s > i.s {
            first_set.remove(i);
        }
        first_set.insert(i);
    }

    let mut min = std::i128::MAX;
    for i in second {
        if first_set.contains(i) {
            if i.s + first_set.get(i).unwrap().s < min {
                min = i.s + first_set.get(i).unwrap().s;
            }
        }
    }
    return min;
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let state: Vec<&str> = contents.split("\n").collect();
    let first : Vec<Point> = cable(&state[0]);
    let second : Vec<Point> = cable(&state[1]);

    let args: Vec<String> = env::args().collect();

    let min = match args[1].as_ref() {
        "a" => solve1(&first, &second),
        "b" => solve2(&first, &second),
        _ => -1,
    };

    println!("min_d {}", min);
}
