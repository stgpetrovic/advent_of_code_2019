use std::fs;
use std::env;
use std::cmp;
use std::i128;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

fn solve1(state : HashMap<i128, i128>) {
    let mut masine : HashMap<(i128, i128), Box<IntMachine>> = HashMap::new();
    masine.insert((0, 0), Box::new(IntMachine {state: state, base:0, last_out:0,pc:0}));

    let mut q : VecDeque<(i128, i128)> = VecDeque::new();
    q.push_back((0, 0));
    let mut seen : HashSet<(i128, i128)> = HashSet::new();
    let mut path : HashMap<(i128, i128), i128> = HashMap::new();
    let mut prev : HashMap<(i128, i128), (i128, i128)> = HashMap::new();
    let mut found = false;
    let mut frizider = (0, 0);
    while !q.is_empty() {
        if found {
            break;
        }
        let node = q.pop_front().unwrap();
        println!("exploring {:?}, qsize {}", node, q.len());
        seen.insert(node);

        let im :&mut IntMachine = &mut masine.get(&node).unwrap().clone();

        for (direction, d) in vec![(0, 1), (0, -1), (-1, 0), (1, 0)].iter().enumerate() {
            let mut next_im  = im.clone();
            let code = next_im.cont(&vec![(direction+1) as i128]);
            if code == i128::MAX {
                println!("program ended prematurely");
                found = true;
                break;
            }
            let new_point = (node.0+d.0, node.1+d.1);
            path.insert(new_point, 1 + path.get(&node).unwrap_or(&0));
            prev.insert(new_point, node);
            masine.insert(new_point, Box::new(next_im.clone()));
            match code{
                0=> {},
                1=> {
                    if !seen.contains(&new_point) {
                        q.push_back(new_point);
                    }
                },
                2=> {
                    println!("found fridge @ {:?}", new_point);
                    frizider = new_point;
                    found = true;
                },
                _=> println!("sranja su!"),
            };
        }
    }
    println!("bfs path {}", path.get(&frizider).unwrap());
}

fn solve2(state : HashMap<i128, i128>) {
    let mut masine : HashMap<(i128, i128), Box<IntMachine>> = HashMap::new();
    masine.insert((0, 0), Box::new(IntMachine {state: state, base:0, last_out:0,pc:0}));

    let mut q : VecDeque<(i128, i128)> = VecDeque::new();
    q.push_back((0, 0));
    let mut seen : HashSet<(i128, i128)> = HashSet::new();
    let mut path : HashMap<(i128, i128), i128> = HashMap::new();
    let mut prev : HashMap<(i128, i128), (i128, i128)> = HashMap::new();
    let mut frizider = (0, 0);
    let mut ploca : HashMap<(i128, i128), i128> = HashMap::new();
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        println!("exploring {:?}, qsize {}", node, q.len());
        seen.insert(node);

        let im :&mut IntMachine = &mut masine.get(&node).unwrap().clone();

        for (direction, d) in vec![(0, 1), (0, -1), (-1, 0), (1, 0)].iter().enumerate() {
            let mut next_im  = im.clone();
            let code = next_im.cont(&vec![(direction+1) as i128]);
            if code == i128::MAX {
                println!("program ended prematurely");
                break;
            }
            let new_point = (node.0+d.0, node.1+d.1);
            ploca.insert((new_point.0+21, new_point.1+19), if code == 0 {0} else {1});
            path.insert(new_point, 1 + path.get(&node).unwrap_or(&0));
            prev.insert(new_point, node);
            masine.insert(new_point, Box::new(next_im.clone()));
            match code{
                0=> {},
                1=> {
                    if !seen.contains(&new_point) {
                        q.push_back(new_point);
                    }
                },
                2=> {
                    println!("found fridge @ {:?}", new_point);
                    frizider = new_point;
                },
                _=> println!("sranja su!"),
            };
        }
    }
    println!("{:?}", ploca);
    // novi bfs
    let mut q0 : VecDeque<(i128, i128)> = VecDeque::new();
    q0.push_back((16+21, 12+19));
    let mut seen0 : HashSet<(i128, i128)> = HashSet::new();
    let mut path0 : HashMap<(i128, i128), i128> = HashMap::new();

    while !q0.is_empty() {
        let node = q0.pop_front().unwrap();
        seen0.insert(node);
        println!("exploring {:?}", node);

        for d in vec![(0, 1), (0, -1), (-1, 0), (1, 0)] {
            let new_point = (node.0+d.0, node.1+d.1);
            if new_point.0 > 40 || new_point.0 < 0 || new_point.1 > 40 || new_point.1<0 {
                println!("overboard with {:?}", new_point);
                continue;
            }
            if *ploca.get(&new_point).unwrap() == 1 {
                if !seen0.contains(&(new_point)) {
                    path0.insert(new_point, 1 + path0.get(&node).unwrap_or(&0));
                    q0.push_back(new_point);
                }
            }
        }
    }
    let mut max_distance = 0;
    for (k, v) in path0 {
        println!("{:?} {}",k, v);
        max_distance = cmp::max(max_distance, v);
    }
    println!("{:?}",max_distance);

}

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let state: Vec<i128> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    let mut state_map : HashMap<i128, i128> = HashMap::new();
    for i in 0..state.len() {
        state_map.insert(i as i128, state[i]);
    }

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(state_map),
        "b" => solve2(state_map),
        _ => println!("bezi bre a ili b"),
    };
}

#[derive(Clone)]
struct IntMachine {
  pub state: HashMap<i128, i128>,
  pub last_out : i128,
  pub pc : i128,
  pub base : i128,
}

impl IntMachine  {
  pub fn cont(&mut self, inputs : &Vec<i128>) ->i128 {
      return self.run(self.pc,inputs);
  }

  fn get(&mut self, i : i128) ->i128 {
      return *self.state.get(&i).unwrap_or(&0);
  }

  pub fn set(&mut self, i : i128, j:i128) {
      self.state.insert(i, j);
  }

  pub fn run(&mut self, start_i:i128, inputs: &Vec<i128>) -> i128 {
    let mut consumed = 0;
    let mut i = start_i;
    while (i as usize) < self.state.len() {
        let opcode :i128 = (self.get(i) % 100) as i128;
        let s = self.get(i).to_string();
        //println!(">>>>    [{}] opcode {}, i = {}", self.get(i), opcode, i);
        let mut param_mask = &""[0..0];

        if s.len() > 1 {
            param_mask = &s[0..s.len()-2];
        }
        match opcode {
            1 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                //println!("adding {} + {} -> mem {}",p1, p2, idx);
                self.set(idx,  p1 + p2);
                i+=4;
            },
            2 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                //println!("mul {} * {} -> mem {} : {}",p1, p2, idx, p1*p2);
                self.set(idx,  p1 * p2);
                i+=4;
            },
            3 => {
                let idx = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                self.set(idx,  inputs[consumed]);
                consumed += 1;
                //println!("INPUT {} at {}",inputs[consumed-1],idx);
                i+=2;
            },
            4 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                self.last_out = p1;
                //println!("OUTPUT {}", p1);
                self.pc = i+2;
                return p1;
            },
            5 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                if p1 > 0 {
                    //println!("{} >0, pc<-{}", p1, p2);
                    i = p2;
                } else {
                    //println!("{} <= 0, pc stays", p1);
                  i+=3;
                }
            },
            6 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                if p1 == 0 {
                    //println!("{} == 0, pc<-{}", p1, p2);
                    i = p2;
                } else {
                    //println!("{} != 0, pc stays", p1);
                  i+=3;
                }
            },
            7 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                if p1 < p2 {
                    //println!("{} < {}, {}<-1", p1, p2, idx );
                    self.set(idx,  1);
                } else {
                    //println!("{} >= {}, {}<-0", p1, p2, idx );
                    self.set(idx,  0);
                }
                i+=4;
            },
            8 => {
                let p1 = param(opcode,self.base, param_mask, i + 1, 0, &self.state);
                let p2 = param(opcode,self.base, param_mask, i + 1, 1, &self.state);
                let idx = param(opcode,self.base, param_mask, i + 1, 2, &self.state);
                if p1 == p2 {
                    //println!("{} == {}, {}<-1", p1, p2, idx );
                    self.set(idx,  1);
                } else {
                    //println!("{} != {}, {}<-0", p1, p2, idx );
                    self.set(idx,  0);
                }
                i+=4;
            },
            9 => {
                let p1 = param(opcode, self.base, param_mask, i + 1, 0, &self.state);
                //println!("changing base by {}, from {} to {}", p1, self.base, self.base + p1);
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

    //println!("mask {}, base {}, i {}, pos {}, mask value {}, v[i+pos]={}", mask, base, i, p, mv, get(v,ipos));
    // Index ps
    if (p==2 && (opcode == 1 || opcode == 2 || opcode == 7 || opcode == 8)) ||
       (p==0 && opcode == 3) {
           if mv == 0 || mv == 1 {
                //println!("IDX param is {}",get(v, ipos));
                return get(v, ipos);
           } else if mv == 2 {  // relative
               //println!("REL IDX  param is {}, location there {}",get(v, ipos) + base, get(v, get(v, ipos)+base));
               return get(v, ipos) + base;
           }
    }

    let r = match &mv {
        0 => get(v,get(v,ipos)),
        1 => get(v,ipos),
        2 => if opcode != 3 {get(v,base+get(v,ipos))} else {base+get(v,ipos)},
        _ => -1,
    };
    //println!("param is {}",r);
        return r;
}
