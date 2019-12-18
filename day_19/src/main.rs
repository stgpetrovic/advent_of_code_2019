use std::fs;
use std::env;
use std::char;
use std::i128;
use std::collections::HashMap;
use std::collections::VecDeque;

const W: usize = 50;
const H: usize = 50;

type Vasiona = [[char; W]; H];

fn solve1(state : &mut HashMap<i128, i128>) {
    let ploca = input_to_ploca(state);
    draw(&ploca);
}

fn solve2(state : &mut HashMap<i128, i128>) {
    println!("{}", bs(state));
}

fn bs(state: &mut HashMap<i128, i128>) -> i128 {
    let need = 1e12 as i128;
    let mut lo = 0; let mut mid = -1; let mut hi = 100000 as i128;  // just in case konzerva

    while lo <= hi {
        mid = (lo+hi)/2; // nema unsigned shift :S
        let v = max_y_for_x(state, mid);
        println!("trying {}, {}", mid, v);
        if pun_ko_brod(state, mid, v) {
            println!("found pun ko brod {} {}", mid, v);
            hi = mid;
        } else {
            lo = mid;
        }
        if lo == hi - 1 {
            let y = max_y_for_x(state, hi);
            println!("hi-99 {} max_y{}", hi-99, y);
            return (hi - 99) * 10000 + y;
        }
    }
    return -1;
}

fn draw(ploca : &Vasiona) {
    let mut beam = 0;
    for i in 0..ploca.len() {
        for j in 0..ploca[i].len() {
            print!("{}", ploca[i][j]);
            if ploca[i][j] == '#' {
                beam += 1;
            }
        }
        println!();
    }
    println!("beam fields {}", beam);
}

fn pun_ko_brod(state: &mut HashMap<i128, i128>, i: i128, j: i128) -> bool {
    for d in &[(0,0), (0, 99), (-99, 99), (-99,0 )] {
        if !read_single(state, i+d.0, j+d.1) {
            return false;
        }
    }
    println!("naso ga cijelog {} {}", i, j);
    return true;
}

fn max_y_for_x(state: &mut HashMap<i128, i128>, x:i128) -> i128 {
    let mut lo = 0; let mut mid = -1; let mut hi = 3*x;  // 3x = conservative odoka

    println!("max y fo x {}", x);
    while lo <= hi {
        mid = (lo+hi)/2; // nema unsigned shift :S
        if read_single(state, x, mid) {
            hi = mid;
        } else {
            lo = mid;
        }
        if lo == hi - 1 {
            println!("max y for {} is {}",x, hi);
            return hi;
        }
    }
    return -1;
}

fn read_single(state :&mut  HashMap<i128, i128>, i:i128, j:i128) -> bool {
    let mut im = IntMachine {state: &mut state.clone(), base:0, last_out:0,pc:0};
    let camera = im.cont(&mut VecDeque::from(vec![i , j ]));
    println!("reading at {}, {} = {}",i,j,camera==1);
    return camera == 1;
}

fn input_to_ploca(state : &mut HashMap<i128, i128>) -> Vasiona {
    let mut s : String = "".to_string();
    for i in 0..H {
        for j in 0..W{
            if !read_single(state, i as i128, j as i128) {
                s.push('.');
            } else {
                s.push('#');
            }
        }
    }
    return parse(&s);
}

fn parse(input : &str) ->  Vasiona {
    let mut ploca : Vasiona = [['0'; W]; H];
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
    let state: Vec<i128> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    let mut state_map : HashMap<i128, i128> = HashMap::new();
    for i in 0..state.len() {
        state_map.insert(i as i128, state[i]);
    }

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&mut state_map),
        "b" => solve2(&mut state_map),
        _ => println!("bezi bre a ili b"),
    };
}

struct IntMachine<'a> {
  pub state: &'a mut HashMap<i128, i128>,
  pub last_out : i128,
  pub pc : i128,
  pub base : i128,
}

impl<'a> IntMachine<'a>  {
  pub fn cont(&mut self, inputs : &mut VecDeque<i128>) ->i128 {
      return self.run(self.pc,inputs);
  }

  fn get(&mut self, i : i128) ->i128 {
      return *self.state.get(&i).unwrap_or(&0);
  }

  pub fn set(&mut self, i : i128, j:i128) {
      self.state.insert(i, j);
  }

  pub fn run(&mut self, start_i:i128, inputs: &mut VecDeque<i128>) -> i128 {
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
                let v = inputs.pop_front().unwrap();
                self.set(idx,  v);
                //println!("INPUT {} at {}",v,idx);
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
            _ => {},//println!("error opcode {}", i),
        }
    }
    //println!("ZESCA SU SRANJA NA {}", self.pc);
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
