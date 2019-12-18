use std::fs;
use std::env;
use std::char;
use std::u32;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::process;

const H : u8 = 81;
const W : u8 = 81;

type Ploca = [[char; W as usize]; H as usize];

fn solve1(input: &str) {
    generic(input);
}

fn solve2(input : &str) {
    let ploca = parse_input(input);
    let key_locations = key_loc(input);

    let doom_players = [(39, 39), (39, 41), (41, 39), (41, 41)];
    draw(&ploca);

    // posli extractat bfs u neki helper. mejbi (:
    let mut q : VecDeque<Path2> = VecDeque::new();
    let mut min_dist = 1000000;
    q.push_back(Path2{point: doom_players, keys: [false; 26], distance: 0});

    // keyed to path2\distance.
    let mut seen :HashMap<([(u8, u8); 4], [bool; 26]), u32> = HashMap::new();

    while !q.is_empty() {
        let mut path = q.pop_back().unwrap();

        // ako smo vidjeli bolje dane
        if seen.contains_key(&(path.point, path.keys)) && path.distance >= *seen.get(&(path.point, path.keys)).unwrap() {
            continue;
        }
        seen.insert((path.point, path.keys), path.distance);

        let mut all_robots_would_move_to_valid_point = true;
        for p in &path.point {
            let v = ploca[p.0 as usize][p.1 as usize];
            if v == '#' {
                continue;
            }
            if !on_board(&p) {
                all_robots_would_move_to_valid_point = false;
                break;
            }
            if v.is_ascii_uppercase() {  // door
                if !path.keys[v.to_lowercase().next().unwrap() as usize - 97] {
                    all_robots_would_move_to_valid_point = false;
                    break; // kein Schluessel
                }
            }
        }
        if !all_robots_would_move_to_valid_point {
            continue;
        }

        //println!("expand {:?} {:?}", path.point, path.keys);

        let mut doom_queue : VecDeque<Tacka> = VecDeque::new();
        for (i,p) in path.point.iter().enumerate() {
            doom_queue.push_back(Tacka{point: *p, player :i as u8, distance: 0});
        }

        // map all reachable locations from this state.
        let mut distances : HashMap<(u8, u8), PlayerDistance> = HashMap::new();
        while !doom_queue.is_empty() {
            let doom = doom_queue.pop_front().unwrap();

            if !on_board(&((doom.point).0, (doom.point).1)) {
                continue;
            }
            let v = ploca[(doom.point).0 as usize][(doom.point).1 as usize];
            if v == '#' {
                continue;
            }
            if v.is_ascii_uppercase() && !path.keys[v.to_lowercase().next().unwrap() as usize - 97] {
                continue;
            }
            if distances.contains_key(&doom.point) { // been there, done that
                continue;
            }
            distances.insert(doom.point, PlayerDistance{player:doom.player, distance:doom.distance});

            for d in vec![(0, 1), (0, -1), (-1, 0), (1, 0)] {
                doom_queue.push_front(Tacka{point:((((doom.point).0 as isize)+d.0) as u8, (((doom.point).1 as isize)+d.1)as u8), player:doom.player, distance:doom.distance+1});
            }
        }

        // add to stack each reachable location with missing key (move robot there, add distance).
        for (key, loc) in &key_locations {
            // If we don't have the key and could get it by moving from this state.
            if !path.keys[*key as usize - 97] && distances.contains_key(&loc) {
                let player_distance = distances.get(&loc).unwrap();
                let mut next_points = path.point;
                next_points[player_distance.player as usize] = *loc;   // move the robot to this key, collecting it
                let mut next_keys = path.keys;
                next_keys[*key as usize -97] = true;  // add the key
                // If this is the last key, mozda stagod stignen i radit danas
                if next_keys.iter().filter(|&x| *x == true).count() == key_locations.len() {
                    min_dist = cmp::min(min_dist, path.distance+player_distance.distance);
                    println!("found some distance that works {}", min_dist);
                }
                // Idemo dalje (HDZ).
                q.push_back(Path2{point: next_points, keys: next_keys, distance: path.distance+player_distance.distance});
            }
        }
    }
    println!("sol {}", min_dist);
}

fn test() {
    let test_input =
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
    generic(&test_input);
}

fn generic(input: &str) {
    let ploca = parse_input(input);
    draw(&ploca);

    let dist = solve(&ploca, Path{ point: (40, 40), distance: 0, keys: [false; 26]}, /*total_keys=*/26);
    println!("d {}", dist);
}

fn key_loc(input : &str) -> HashMap<char, (u8, u8)>{
  let mut keys : HashMap<char, (u8, u8)> = HashMap::new();
  let mut row = 0; let mut col = 0;
  for c in input.trim().chars() {
    if c == '\n' {
      row += 1;
      col = 0;
      continue;
    }
    if c.is_ascii_lowercase() {
        keys.insert(c, (row as u8, col as u8));
    }
    col += 1;
  }
  return keys;
}

fn parse_input(input : &str) -> Ploca {
  let mut ploca : Ploca = [['0'; W as usize]; H as usize];
  let mut row = 0; let mut col = 0;
  for c in input.trim().chars() {
    if c == '\n' {
      row += 1;
      col = 0;
      continue;
    }
    if c == '@' {
        println!("covik @ {} {}", row, col);
    }
    ploca[row][col] = c;
    col += 1;
  }
  return ploca;
}

fn draw(ploca : &Ploca) {
    let mut ri = 0;
    let mut rj = 0;
    let mut keys : HashMap<(u8, u8), char> = HashMap::new();
    let mut doors : HashMap<(u8, u8), char> = HashMap::new();
    for i in 0..ploca.len() {
        for j in 0..ploca[i].len() {
            print!("{}", ploca[i][j]);
            if ploca[i][j].is_ascii_lowercase() {
                keys.insert((i as u8, j as u8), ploca[i][j]);
            } else if ploca[i][j].is_ascii_uppercase() {
                doors.insert((i as u8, j as u8), ploca[i][j]);
            }
        }
        println!();
    }
    for (k, v) in keys {
        println!("Key {} @ ({},{})", v, k.0, k.1);
    }
    for (k, v) in doors {
        println!("Door {} @ ({},{})", v, k.0, k.1);
    }
}

fn on_board(point : &(u8, u8)) -> bool {
  return point.0 >= 0 && point.0 < H && point.1 >=0 && point.1 < W;
}


#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct PlayerDistance {
    player : u8,
    distance : u32,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Tacka {
    point : (u8, u8),
    player : u8,
    distance : u32,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Path2 {
    point : [(u8, u8); 4],
    keys : [bool; 26],
    distance : u32,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Path {
    point : (u8, u8),
    keys : [bool; 26],
    distance : u32,
}

// bfs sa stateom tacka, keyevi, dist
fn solve(ploca: &Ploca, start: Path, total_keys :usize) -> u32{
    let mut q : VecDeque<Path> = VecDeque::new();
    q.push_back(start);

    let mut seen :HashSet<((u8, u8), [bool; 26])> = HashSet::new();

    loop {
        let mut path = q.pop_back().unwrap();

        if !on_board(&path.point) || seen.contains(&(path.point, path.keys)) {
            continue;
        }
        seen.insert((path.point, path.keys));
        //println!("expand {:?} {:?}", path.point, path.keys);

        let v = ploca[path.point.0 as usize][path.point.1 as usize];

        if v == '#' {
            continue;
        }

        if v.is_ascii_uppercase() {  // door
            if !path.keys[v.to_lowercase().next().unwrap() as usize - 97] {
                println!("no key for door {}, keys={:?}", v,path.keys);
                continue; // kein Schluessel
            }
        }

        let mut next_keys = path.keys;
        if v.is_ascii_lowercase() {  // key
            if !path.keys[v as usize - 97] {
                next_keys[v as usize - 97] = true;
                println!("found key {}", v);
                if next_keys.iter().filter(|&x| *x == true).count() == total_keys {
                    //println!("found all keys, distance {}", path.distance);
                    return path.distance;
                }
            }
        }

        for d in vec![(0, 1), (0, -1), (-1, 0), (1, 0)] {
            q.push_front(Path{point: (((path.point.0 as isize)+d.0) as u8, ((path.point.1 as isize)+d.1)as u8), keys:next_keys, distance:path.distance+1});
        }
    }
    return u32::MAX;
}

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let contents2 = fs::read_to_string("inputb")
        .expect("Something went wrong reading the file");

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&contents),
        "b" => solve2(&contents2),
        "test" => test(),
        _ => println!("bezi bre a ili b"),
    };
}
