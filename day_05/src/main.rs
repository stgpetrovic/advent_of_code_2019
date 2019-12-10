use std::fs;
use std::env;

struct IntMachine {
  pub state: Vec<isize>,
}

fn param(mask : &str, i:usize, position:usize, v: &Vec<isize>) -> isize {
    if mask.len() == 0 {
        println!("positional param @ {}, {}", i+position, v[v[i+position] as usize]);
        return v[v[i+position] as usize];
    }

    let mut mv = 0;
    if (position < mask.len()) {
       mv = mask.chars().rev().collect::<String>()[position..position+1].parse::<isize>().unwrap();
    }

    println!("mask {}, i {}, pos {}, mask value {}", mask, i, position, mv);
    let r= match &mv {
        0 => v[v[i+position] as usize],
        1 => v[i+position],
        _ => -1,
    };
    println!("returning {}",r);
        return r;
}

impl IntMachine {
  pub fn init(&mut self, a: isize, b: isize) {
      self.state[1]  = a;
      self.state[2]  = b;
  }

  pub fn run(&mut self, input : isize) -> isize {
    let mut i = 0;
    while i < self.state.len() {
        let opcode :usize = (self.state[i] % 100) as usize;
        let s = self.state[i].to_string();
        println!(">>>>    [{}] opcode {}, i = {}", self.state[i], opcode, i);
        let mut param_mask = &""[0..0];

        if s.len() > 1 {
            param_mask = &s[0..s.len()-2];
        }
        match opcode {
            1 => {
                let idx = self.state[i+3] as usize;
                let p1 = param(param_mask, i + 1, 0, &self.state);
                let p2 = param(param_mask, i + 1, 1, &self.state);
                println!("adding {} + {} -> mem {}",p1, p2, idx);
                self.state[idx] = p1 + p2;
                i+=4;
            },
            2 => {
                let idx = self.state[i+3] as usize;
                let p1 = param(param_mask, i + 1, 0, &self.state);
                let p2 = param(param_mask, i + 1, 1, &self.state);
                println!("mul {} * {} -> mem {}",p1, p2, idx);
                self.state[idx] = p1 * p2;
                i+=4;
            },
            3 => {
                let idx = self.state[i+1] as usize;
                self.state[idx] = input;
                println!("inputing {} at {}",input,idx);
                i+=2;
            },
            4 => {
                let p1 = param(param_mask, i + 1, 0, &self.state);
                println!("out: {}", p1);
                i+=2;
            },
            5 => {
                let p1 = param(param_mask, i + 1, 0, &self.state);
                let p2 = param(param_mask, i + 1, 1, &self.state);
                if (p1 > 0) {
                    i = p2 as usize;
                } else {
                  i+=3;
                }
            },
            6 => {
                let p1 = param(param_mask, i + 1, 0, &self.state);
                let p2 = param(param_mask, i + 1, 1, &self.state);
                if (p1 == 0) {
                    i = p2 as usize;
                } else {
                  i+=3;
                }
            },
            7 => {
                let idx = self.state[i+3] as usize;
                let p1 = param(param_mask, i + 1, 0, &self.state);
                let p2 = param(param_mask, i + 1, 1, &self.state);
                if (p1 < p2) {
                    self.state[idx] = 1;
                } else {
                    self.state[idx] = 0;
                }
                i+=4;
            },
            8 => {
                let idx = self.state[i+3] as usize;
                let p1 = param(param_mask, i + 1, 0, &self.state);
                let p2 = param(param_mask, i + 1, 1, &self.state);
                if (p1 == p2) {
                    self.state[idx] = 1;
                } else {
                    self.state[idx] = 0;
                }
                i+=4;
            },
            99 => {break;},
            _ => println!("error opcode {}", i),
        }
    }
    return self.state[0]
  }
}

fn solve1(state : &Vec<isize>) {
    let mut im = IntMachine {state: state.to_vec()};
    let v = im.run(1);
    println!("{}", v);
}

fn solve2(state : &Vec<isize>) {
    let mut im = IntMachine {state: state.to_vec()};
    let v = im.run(5);
    println!("{}", v);
}

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let state: Vec<isize> = contents.split(",").map(|s| s.parse().unwrap()).collect();

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&state),
        "b" => solve2(&state),
        _ => println!("error input, only a or b allowed"),
    }
}
