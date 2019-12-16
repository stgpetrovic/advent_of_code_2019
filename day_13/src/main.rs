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

// tiles -- stupid rust cannot convert int to enum
// 0 empty
// 1 wall
// 2 block
// 3 horizontal paddle
// 4 ball

fn solve1(state : &mut HashMap<i128, i128>, initial_color : i128) {
    let mut im = IntMachine {state: state, base:0, last_out:0,pc:0};

    let mut ploca : HashMap<(i128, i128), i128> = HashMap::new();
    let mut block_count = 0;
    loop {
        let top = im.cont(&vec![]);
        if top == i128::MAX {
            break;
        }
        let down = im.cont(&vec![]);
        let tile_id = im.cont(&vec![]);

        ploca.insert((top, down), tile_id);

        if tile_id == 2 /*block*/ {
            block_count +=1;
        }
    }
    println!("block count {}", block_count);
}

fn draw(ploca : &HashMap<(i128, i128), i128>) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (k, _v) in ploca {
      max_x = cmp::max(max_x, k.0);
      max_y = cmp::max(max_y, k.1);
    }
    for j in 0..max_y+1 {
        for i in 0..max_x+1 {
            match ploca.get(&(i, j)) {
                Some(x) => {
                    match x {
                        0 => print!(" "),
                        1 => print!("█"),
                        2 => print!("░"),
                        3 => print!("_"),
                        4 => print!("☮"),
                        _ => {},
                    };
                },
                None => print!(" "),
            };
        }
        println!();
    }
    println!();
    println!();
}

fn direction(p : &Vec<(i128, i128)>) -> (i128, i128) {
    if p.len() < 2 {
        return (0, 0);
    }
    return (p[p.len()-1].0-p[p.len()-2].0, p[p.len()-1].1-p[p.len()-2].1);
}

fn solve(iploca : HashMap<(i128, i128), i128>, p: (i128, i128), d: (i128, i128)) -> i128 {
    let mut ploca = iploca.clone();
    let mut position = p;
    let mut direction = d;

    // find next crash with y = 23 (horizontal paddle level).
    loop {
        // move 1 frame if possible
        if *ploca.get(&(position.0+direction.0,position.1+direction.1)).unwrap_or(&7) == 0 {
            position.0 = position.0+direction.0;
            position.1 = position.1+direction.1;
        }

        println!("POS {:?}", position);
        if position.1 == 22 { // crash point
            println!("crash point found {}", position.0);
            return position.0;
        }

        match ploca.get(&(position.0+direction.0, position.1+direction.1)) {
            Some(f) => {
                match f {
                    0 => {/*empty, ignore */},
                    1 => {
                        if position.1 == 1 {
                            println!("gornji zid, flip y");
                            direction.1 = -direction.1;
                        }
                        if position.0 == 1 || position.0 > 20 {
                            println!("sa strane zid, flip x");
                            direction.0 = -direction.0;
                        }
                    }, // wall
                    2 => {

                        // if something on the same level
                        let next_x_y = *ploca.get(&(position.0+direction.0, position.1+direction.1)).unwrap_or(&7);
                        let next_x = *ploca.get(&(position.0+direction.0, position.1)).unwrap_or(&7);
                        let povise = *ploca.get(&(position.0, position.1+direction.1)).unwrap_or(&7);
                        println!("next_x({}, {}) {} povise({}, {}) {}",  position.0+direction.0, position.1, next_x, position.0, position.1+direction.1,povise);

                        if povise == 2 { 
                            println!("povise normalno, filp y only");
                            ploca.insert((position.0, position.1+direction.1), 0);
                            direction.1 = -direction.1;
                        } else if next_x == 2 {
                            println!("sa strane udre, flip x");
                            ploca.insert((position.0+direction.0, position.1), 0);
                            direction.0 = -direction.0;
                        } else if next_x_y == 2 {
                            println!("block diagonally, filp x, y");
                            ploca.insert((position.0+direction.0, position.1+direction.1), 0);
                            direction.0 = -direction.0;
                            direction.1 = -direction.1;
                        }
                    },
                    3 => {/*paddle, ignore*/},
                    4 => {/*ball, ignore*/},
                    _ => {},
                };
            },
            None => {},
        };
    }
    return 0;
}


// the stick of joy
//  0 - neutral
// -1 - left
// +1 - right

fn solve2(state : &mut HashMap<i128, i128>, initial_color : i128) {
    let mut cheat_codes = vec![21];
    let mut iter = 0;

    while iter < 270 {
        let mut im = IntMachine {state: state, base:0, last_out:0,pc:0};
        im.set(0, 2);  // coin

        let mut ploca : HashMap<(i128, i128), i128> = HashMap::new();
        let mut hiscore = 0;
        let mut next_input = 0;
        let mut p1 = (12, 23);
        let mut ball = (19, 20);

        loop {
            if p1.0 != cheat_codes[iter] {
                next_input = if p1.0 > cheat_codes[iter] { -1 } else { 1 };
            }

            let mut interesting_frame = false;
            let top = im.cont(&vec![next_input]);
            if top == i128::MAX {
                break;
            }
            let down = im.cont(&vec![next_input]);
            let tile_id = im.cont(&vec![next_input]);

            if top == -1 && down == 0 {
                hiscore = tile_id;

            } else {
                ploca.insert((top, down), tile_id);
            }

            if tile_id == 3 {
                p1 = (top, down);
                interesting_frame = true;
            } else if tile_id == 4 {
                ball = (top, down);
                interesting_frame = true;
            }

            if ball.1 == 22 {
                cheat_codes.push(ball.0);
            }

            if interesting_frame {
                draw(&ploca);
                println!("Player 1: {:?}, Ball: {:?}\n\n\n\n\n\n", p1, ball);
            }
        }
        iter += 1;
        println!("HISCORE {} {}", iter, hiscore);
    }
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
