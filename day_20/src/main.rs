use std::fs;
use std::env;
use std::char;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

const W: usize = 129;
const H: usize = 123;

type Labirint = [[char; W]; H];
type Tacka = (usize, usize);

#[derive(Debug, Hash, PartialEq)]
struct Sracka {
    t:Tacka,
    level :i32,
    dist: i32,
}

struct PituraniLabirint {
    lab: Labirint,
    portal: HashMap<Tacka, String>,
    rportal: HashMap<String, Vec<Tacka>>,
}

impl PartialEq for PituraniLabirint {
    fn eq(&self, other: &PituraniLabirint) -> bool {
        if self.portal != other.portal {
            return false;
        }
        for i in 0..H {
            for j in 0..W {
                if self.lab[i][j] != other.lab[i][j] {
                    return false;
                }
            }
        }
        return true;
    }
}

fn solve1(input : &str) {
    let lab = input_to_ploca(input);
    draw(&lab.lab);
    println!("lab.map {:?}", lab.portal);
    println!("lab.map {:?}", lab.rportal);

    let mut q : VecDeque<(Tacka, i32)> = VecDeque::new();
    let start = lab.rportal.get("AA").unwrap()[0];
    q.push_back((start, 0));
    let mut seen : HashSet<Tacka> = HashSet::new();
    let mut path : HashMap<Tacka, Vec<Tacka>> = HashMap::new();
    path.insert(start, vec![start]);

    loop {
        let mut point = q.pop_front().unwrap();
        println!("Trying {:?}", point);

        if let Some(x) = &lab.portal.get(&point.0) {
            if *x == "ZZ" {
                println!("Found solution, {:?}", point);
                println!("{:?} {}", path.get(&point.0).unwrap(), path.get(&point.0).unwrap().len());
                break;
            }
        }

        if lab.lab[(point.0).0][(point.0).1] != '.' {
            continue;
        }
        if (point.0).0 < 0 || (point.0).0 >= H || (point.0).1 < 0 || (point.0).1 >= W {
            continue;
        }
        if seen.contains(&point.0) {
            continue;
        }
        seen.insert(point.0);

        // neighbour
        for d in vec![(0, 1), (0, -1), (-1, 0), (1, 0)] {
            if ((point.0).0 == 0  && d.0 ==-1)|| ((point.0).1 == 0 && d.1==-1) {
                continue;
            }
            let new_point = (((point.0).0 as isize+d.0) as usize, ((point.0).1 as isize+d.1) as usize);
            let mut p = path.get(&point.0).unwrap().clone();
            p.push(point.0);
            path.insert(new_point, p.to_vec());
            q.push_back((new_point, point.1+1));
        }
        // portal
        if let Some(x) = &lab.portal.get(&point.0) {
            for neighbour in lab.rportal.get(*x) {
                for n in neighbour {
                    if *n != point.0 {
                        println!("Teleport from {:?} to {:?} via {:?}", point.0, *n, *x);
                        let mut p = path.get(&point.0).unwrap().clone();
                        p.push(point.0);
                        path.insert(*n, p.to_vec());
                        q.push_back((*n, point.1+1));
                    }
                }
            }
        }
    }
}

fn solve2(input : &str) {
    let lab = input_to_ploca(input);
    draw(&lab.lab);

    let mut q :VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    let mut seen : HashSet<(usize, usize, usize)> = HashSet::new();
    let mut start = lab.rportal.get("AA").unwrap()[0];
    let mut end = lab.rportal.get("ZZ").unwrap()[0];
    q.push_back((start.0, start.1, 0, 0));

    while !q.is_empty() {
        let (x, y, level, distance) = q.pop_front().unwrap();
        println!("looking: {}, {}, {}, {}", x, y, level, distance);
        if seen.contains(&(x, y, level)) {
            continue;
        }
        seen.insert((x, y, level));

        if lab.lab[x][y] != '.' {
            continue;
        }

        if level == 0 && (x, y) == end {
            println!("found: {}, {}, {}, {}", x, y, level, distance);
            break;
        }

        for d in vec![(0, 1), (0, -1), (-1, 0), (1, 0)] {
            if (x == 0  && d.0 ==-1)|| (y == 0 && d.1==-1) {
                continue;
            }
            let new_point = ((x as isize+d.0) as usize, (y as isize+d.1) as usize);
            q.push_back((new_point.0, new_point.1, level, distance+1));
        }

        if !lab.portal.contains_key(&(x, y)) {
            continue;
        }
        let v = lab.portal.get(&(x, y)).unwrap();
        for new_point in lab.rportal.get(v).unwrap() {
            if new_point == &(x, y) {
                continue;
            }

            let mut next_level = level;
            if is_outer((x, y)) {
                if level == 0 {
                    continue;
                } else {
                    next_level -= 1;
                }
            } else {
                next_level += 1;
            }
            if next_level > lab.portal.len() {
                continue;  // odjebi
            }

            q.push_back((new_point.0, new_point.1, next_level, distance+1));
        }
    }
}

fn draw(ploca : &Labirint) {
    for i in 0..ploca.len() {
        for j in 0..ploca[i].len() {
            print!("{}", ploca[i][j]);
            if ploca[i][j] == '#' {
            }
        }
        println!();
    }
}

fn is_outer(t : Tacka) -> bool {
    return  t.0 == 2 || t.1 == 2 || t.0 == 120 || t.1 == 126;
}

fn input_to_ploca(input : &str) -> PituraniLabirint {
    let mut labirint = [['0'; W]; H];
    let (mut row, mut col) = (0, 0);
    for c in input.chars() {
        if c == '\n' {
            row += 1;
            col = 0;
            continue;
        }
        labirint[row][col] = c;
        col += 1;
    }
    let mut portali : HashMap<Tacka, String> = HashMap::new();
    let mut rportali : HashMap<String, Vec<Tacka>> = HashMap::new();

    for i in 0..H {
        for j in 0..W {
            if labirint[i][j] == '.' {
                for d in &[(0,1), (1,0), (-1,0), (0,-1)] {
                    let new_point = ((i as isize+d.0) as usize, (j as isize+d.1) as usize);
                    if new_point.0 < 0 || new_point.0 >= H || new_point.1 < 0 || new_point.1 >= W {
                        continue;
                    }
                    let new_new_point = ((i as isize+2*d.0) as usize, (j as isize+2*d.1) as usize);
                    if labirint[new_point.0][new_point.1].is_ascii_uppercase() {
                        let mut label = "".to_string();
                        if d.0 + d.1 > 0 {
                            label.push(labirint[new_point.0][new_point.1]);
                            label.push(labirint[new_new_point.0][new_new_point.1]);
                        } else {
                            label.push(labirint[new_new_point.0][new_new_point.1]);
                            label.push(labirint[new_point.0][new_point.1]);
                        }
                        portali.insert((i, j), label.to_string());
                        if let Some(x) = rportali.get_mut(&label) {
                            x.push((i, j));
                        } else {
                            rportali.insert(label, vec![(i, j)]);
                        }
                    }
                }
            }
        }
    }

    return PituraniLabirint { lab: labirint, portal :portali, rportal: rportali};
}

fn parse(input : &str) ->  Labirint {
    let mut ploca : Labirint = [['0'; W]; H];
    let mut row = 0;
    let mut col = 0;
    for c in input.trim().chars() {
        if col == H {
            row +=1;
            col = 0;
        }
        ploca[row][col]=c;
        col +=1;
    }
    return ploca;
}

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&contents),
        "b" => solve2(&contents),
        _ => println!("bezi bre a ili b"),
    };
}
