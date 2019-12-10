use std::fs;
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::f64;

#[derive(PartialEq, PartialOrd, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn solve1(p : &Vec<Point>) {
    // Solution is the number of unique angles.
    let mut max = 0;
    let mut p_max = &p[0];
    for p1 in p {
        let mut hs  : HashSet<String> = HashSet::new();
        for p2 in p {
            if p1 != p2 {
                let x1 = p2.x - p1.x;
                let y1 = p2.y - p1.y;
                let angle = y1.atan2(x1).to_string();
                hs.insert(angle);
            }
        }
        if max < hs.len() {
            max = hs.len();
            p_max = p1;
        }
    }
    println!("parsed: {}@{:?}", max, p_max);
}

fn solve2(p : &Vec<Point>) {
    let laser = &Point { x: 25.0, y: 31.0 };
    println!("laser {:?}", laser);

    // Group by angle, invert x/y in arctg because we start from up.
    let mut ast : HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    for p1 in p {
        if p1 != laser {
            let x1 = p1.x - laser.x;
            let y1 = p1.y - laser.y;
            let angle = (x1.atan2(y1)).to_string();
            if let Some(vect) = ast.get_mut(&angle) {
                vect.push((x1, y1));
            } else {
                ast.insert(angle, vec![(x1, y1)]);
            }
        }
    }
    let mut count_vec: Vec<(&String, &mut Vec<(f64, f64)>)> = ast.iter_mut().collect();
    // Sort by angle.
    count_vec.sort_by(|b, a| a.0.parse::<f64>().unwrap().partial_cmp(&b.0.parse::<f64>().unwrap()).unwrap_or(Ordering::Less));
    // Within an angle, sort by distance inverse (we pop from back).
    for i in 0..count_vec.len() {
        count_vec[i].1.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Less));
    }

    let mut killed = 0;
    let mut radix = 0;
    let result;
    loop {
        if count_vec[radix].1.len() > 0 {
            let asteroid = count_vec[radix].1.pop().unwrap();
            killed += 1;
            println!("[{}] Killed asteroid {:?}=={:?} at vec {:?}",killed, asteroid, (asteroid.0+laser.x, asteroid.1+laser.y), count_vec[radix].0);
            if killed == 200 {
                result = (asteroid.0+laser.x)*100.+(asteroid.1+laser.y);
                break;
            }
            radix += 1;
            if radix == count_vec.len() {
                radix = 0;
            }
        }
    }
    println!("{:?}",result);
}

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let mut p = Vec::<Point>::new();
    let mut i = 0; let mut j = 0;
    for c in contents.chars() {
        match c {
            '.' => i += 1,
            '#' => {p.push(Point{x:i as f64, y:j as f64}); i += 1},
            '\n' => {j += 1; i=0},
            _ => println!("explode"),
        }
    }

    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "a" => solve1(&p),
        "b" => solve2(&p),
        _ => println!("error input, only a or b allowed"),
    }
}

