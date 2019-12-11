use std::fs;
use std::env;
use std::i128;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

struct IntMachine<'a> {
  pub state: &'a mut HashMap<i128, i128>,
  pub last_out : i128,
  pub pc : i128,
  pub base : i128,
}

impl<'a> IntMachine<'a>  {
  pub fn cont(&mut self, inputs : &Vec<i128>) ->i128 {
      return self.run(self.pc,inputs);
  }

  fn get(&mut self, i : i128) ->i128 {
      return *self.state.get(&i).unwrap_or(&0);
  }

  fn set(&mut self, i : i128, j:i128) {
      self.state.insert(i, j);
  }

  pub fn run(&mut self, start_i:i128, inputs: &Vec<i128>) -> i128 {
    let mut consumed = 0;
    let mut i = start_i;
    while (i as usize) < self.state.len() {
        let opcode :i128 = (self.get(i) % 100) as i128;
        let s = self.get(i).to_string();
        println!(">>>>    [{}] opcode {}, i = {}", self.get(i), opcode, i);
        let mut param_mask = &""[0..0];

        if s.len() > 1 {
            param_mask = &s[0..s.len()-2];
        }
        match opcode {
            1 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                println!("adding {} + {} -> mem {}",p1, p2, idx);
                self.set(idx,  p1 + p2);
                i+=4;
            },
            2 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                println!("mul {} * {} -> mem {} : {}",p1, p2, idx, p1*p2);
                self.set(idx,  p1 * p2);
                i+=4;
            },
            3 => {
                let idx = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                self.set(idx,  inputs[consumed]);
                consumed += 1;
                println!("INPUT {} at {}",inputs[consumed-1],idx);
                i+=2;
            },
            4 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                self.last_out = p1;
                println!("OUTPUT {}", p1);
                self.pc = i+2;
                return p1;
            },
            5 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                if p1 > 0 {
                    println!("{} >0, pc<-{}", p1, p2);
                    i = p2;
                } else {
                    println!("{} <= 0, pc stays", p1);
                  i+=3;
                }
            },
            6 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                if p1 == 0 {
                    println!("{} == 0, pc<-{}", p1, p2);
                    i = p2;
                } else {
                    println!("{} != 0, pc stays", p1);
                  i+=3;
                }
            },
            7 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                if p1 < p2 {
                    println!("{} < {}, {}<-1", p1, p2, idx );
                    self.set(idx,  1);
                } else {
                    println!("{} >= {}, {}<-0", p1, p2, idx );
                    self.set(idx,  0);
                }
                i+=4;
            },
            8 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                if p1 == p2 {
                    println!("{} == {}, {}<-1", p1, p2, idx );
                    self.set(idx,  1);
                } else {
                    println!("{} != {}, {}<-0", p1, p2, idx );
                    self.set(idx,  0);
                }
                i+=4;
            },
            9 => {
                let p1 = param(opcode, self.base, param_mask, i + 1, 0, &self.state);
                println!("changing base by {}, from {} to {}", p1, self.base, self.base + p1);
                self.base += p1;
                i+=2;
            },
            99 => {return i128::MAX;},
            _ => println!("error opcode {}", i),
        }
    }
    println!("ZESCA SU SRANJA NA {}", self.pc);
    return -1;
  }
}

fn get(m: &HashMap<i128, i128>, i : i128) -> i128 {
  return *m.get(&i).unwrap_or(&0);
}

fn param(opcode:i128, base: i128, inmask : &str, i:i128, p:usize, v: &HashMap<i128, i128>) -> i128 {
    let ipos = i + (p as i128);
    let mut mask = "0".repeat(3);
    if mask.len() > 0 {
        mask = inmask.to_string();
    }

    let mut mv = 0;
    if p < mask.len() {
       mv = mask.chars().rev().collect::<String>()[p..p+1].parse::<i128>().unwrap();
    }

    println!("mask {}, base {}, i {}, pos {}, mask value {}, v[i+pos]={}", mask, base, i, p, mv, get(v,ipos));
    // Index ps
    if (p==2 && (opcode == 1 || opcode == 2 || opcode == 7 || opcode == 8)) ||
       (p==0 && opcode == 3) {
           if mv == 0 || mv == 1 {
                println!("IDX param is {}",get(v, ipos));
                return get(v, ipos);
           } else if mv == 2 {  // relative
               println!("REL IDX  param is {}, location there {}",get(v, ipos) + base, get(v, get(v, ipos)+base));
               return get(v, ipos) + base;
           }
    }

    let r = match &mv {
        0 => get(v,get(v,ipos)),
        1 => get(v,ipos),
        2 => if opcode != 3 {get(v,base+get(v,ipos))} else {base+get(v,ipos)},
        _ => -1,
    };
    println!("param is {}",r);
        return r;
}

// 0 is black, 1 is white.
fn get_ploca(ploca : &HashMap<(i128, i128), i128>, l : (i128, i128)) -> i128 {
    match ploca.get(&l) {
        Some(x) => return *x,
        None => return 0,
    }
}

// returns true if map has been updated
fn set_ploca(ploca : &mut HashMap<(i128, i128), i128>, l : (i128, i128), v : i128) ->  bool {
    match ploca.insert(l, v) {
        Some(x) => return x != v,
        None => return false,
    }
}

#[derive(Debug)]
enum Direction {
    UP, RIGHT, DOWN, LEFT,
}

fn solve1(state : &mut HashMap<i128, i128>, initial_color : i128) -> HashMap<(i128, i128), i128> {
    let mut ze_brain = IntMachine {state: state, base:0, last_out:0,pc:0};
    let mut direction = Direction::UP;

    let mut ploca :HashMap<(i128, i128), i128>= HashMap::new();
    let mut l : (i128, i128) = (0, 0);

    let mut new_paint = ze_brain.run(0, &vec![initial_color]);
    let mut new_direction = ze_brain.cont(&vec![]);
    let mut changed_locations : HashSet<(i128, i128)> = HashSet::new();
    loop {
        // pain
        set_ploca(&mut ploca, l, new_paint);
        println!("painting location {:?} color {}", l, if new_paint == 0 {"BLACK"} else {"WHITE"});
        changed_locations.insert(l);

        // turn
        match new_direction {
            0 => {
                match direction {
                    Direction::UP => direction = Direction::LEFT,
                    Direction::RIGHT => direction = Direction::UP,
                    Direction::DOWN => direction = Direction::RIGHT,
                    Direction::LEFT => direction = Direction::DOWN,
                }
            },
            1 => {
                match direction {
                    Direction::UP => direction = Direction::RIGHT,
                    Direction::RIGHT => direction = Direction::DOWN,
                    Direction::DOWN => direction = Direction::LEFT,
                    Direction::LEFT => direction = Direction::UP,
                }
            },
            _ => println!("BOOM"),
        }
        println!("new direction = {:?}", direction);

        // step
        match direction {
            Direction::UP => l.1+=1,
            Direction::RIGHT => l.0+=1,
            Direction::DOWN => l.1-=1,
            Direction::LEFT => l.0-=1,
        }
        println!("stepping, new location = {:?}, current paint {}", l, if get_ploca(&ploca, l) == 0 {"BLACK"}else{"WHITE"});

        new_paint = ze_brain.cont(&vec![get_ploca(&ploca, l)]);
        if new_paint == i128::MAX {
            println!("HALT!");
            break;
        }

        new_direction = ze_brain.cont(&vec![get_ploca(&ploca, l)]);
        if new_direction == i128::MAX {
            println!("HALT!");
            break;
        }
    }
    println!("painted locations count {:?}", changed_locations.len());
    return ploca;
}

fn solve2(state : &mut HashMap<i128, i128>, initial_color:i128) -> HashMap<(i128, i128), i128> {
    let painted = solve1(state, initial_color);
    let mut min_x = 10000;
    let mut min_y = 10000;
    let mut max_x = 0;
    let mut max_y = 0;
    for e in &painted {
        min_x = cmp::min(min_x, (e.0).0);
        max_x = cmp::max(max_x, (e.0).0);

        min_y = cmp::min(min_y, (e.0).1);
        max_y = cmp::max(max_y, (e.0).1);
    }
    println!("({}, {}) -> ({}, {})", min_x, min_y, max_x, max_y);
    println!("{:?}", painted);


    for i in min_x..max_x {
        for j in min_y..max_y {
            match painted.get(&(i, j)) {
                Some(x) => {
                    match x {
                        0 => print!(" "),
                        1 => print!("▓"),
                        _ => print!("?"),
                    };
                }
                None => {},
            };
        }
        println!("");
    }


    return painted;
}


fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    println!("max i128 {}", i128::MAX);
    let state: Vec<i128> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    let mut state_map : HashMap<i128, i128> = HashMap::new();
    for i in 0..state.len() {
        state_map.insert(i as i128, state[i]);
    }

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&mut state_map, 0),
        "b" => solve2(&mut state_map, 1),
        _ => solve1(&mut state_map, 0),
    };
}
