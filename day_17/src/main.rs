use std::fs;
use std::env;
use std::char;
use std::i128;
use std::collections::HashMap;
use std::collections::VecDeque;

type Scaffolding = Vec<Vec<char>>;

fn input_to_ploca(state : &mut HashMap<i128, i128>) -> Scaffolding {
    let mut im = IntMachine {state: state, base:0, last_out:0,pc:0};

    let mut s : String = "".to_string();
    loop {
        let camera = im.cont(&mut VecDeque::new());
        if camera == i128::MAX {
            break;
        }
        s.push(char::from_u32(camera as u32).unwrap());
    }
    return parse(&s);
}

fn solve1(state : &mut HashMap<i128, i128>) {
    let ploca = input_to_ploca(state);
    draw(&ploca);
    println!("alignment {}", alignment(&ploca));
}

fn solve2(state : &mut HashMap<i128, i128>) {
    //let ploca = input_to_ploca(state);
    //draw(&ploca);
    //println!("{}", walk(&ploca));

    // jadna ti majka
    // <A><B><A><B><C><C><B><A><B><C>
    // L4R8L6L10L6R8R10L6L6L4R8L6L10L6R8R10L6L6L4L4L10L4L4L10L6R8R10L6L6L4R8L6L10L6R8R10L6L6L4L4L10
    let prog_a = "L,4,R,8,L,6,L,10";
    let prog_b = "L,6,R,8,R,10,L,6,L,6";
    let prog_c = "L,4,L,4,L,10";
    let code = "A,B,A,B,C,C,B,A,B,C";

    let mut code_chars :VecDeque<i128> = code.chars().map(|c| c as i128).collect();
    let mut prog_a_chars :VecDeque<i128> = prog_a.chars().map(|c| c as i128).collect();
    let mut prog_b_chars :VecDeque<i128> = prog_b.chars().map(|c| c as i128).collect();
    let mut prog_c_chars :VecDeque<i128> = prog_c.chars().map(|c| c as i128).collect();
    let mut program :VecDeque<i128> = VecDeque::new();
    code_chars.push_back('\n' as i128);
    prog_a_chars.push_back('\n' as i128);
    prog_b_chars.push_back('\n' as i128);
    prog_c_chars.push_back('\n' as i128);
    program.extend(code_chars);
    program.extend(prog_a_chars);
    program.extend(prog_b_chars);
    program.extend(prog_c_chars);
    program.push_back('n' as i128);
    program.push_back('\n' as i128);

    let mut im = IntMachine {state: state, base:0, last_out:0,pc:0};
    im.set(0, 2);
    loop {
        let out :i128 = im.cont(&mut program);
        println!("out {}", out);
        if out == i128::MAX {
            break;
        }
    }
}

fn draw(ploca : &Scaffolding) {
    let mut ri = 0;
    let mut rj = 0;
    for i in 0..ploca.len() {
        for j in 0..ploca[i].len() {
            print!("{}", ploca[i][j]);
            if ploca[i][j] == '^' {
                ri = i;
                rj = j;
            }
        }
        println!();
    }
    println!("Robot at ({}, {})", ri, rj);
}

fn parse(input : &str) ->  Scaffolding {
    let mut ploca : Scaffolding = vec![vec![]];
    let mut row = 0;
    let mut col = 0;
    for c in input.trim().chars() {
        if c == '\n' {
            row +=1;
            ploca.push(vec![]);
            col = 0;
            continue;
        }
        ploca[row].push(c);
        col +=1;
    }
    return ploca;
}

fn is_intersection(ploca : &Scaffolding, i : usize, j:usize) -> bool {
    // corner
    if i > ploca.len() - 2 || i < 1 || j > ploca[0].len() - 2 || j < 1 {
        return false;
    }
    // field and all neighbours = #
    return ploca[i][j] == '#' && ploca[i-1][j] == '#' && ploca[i][ j-1] == '#' && ploca[i+1][j] == '#' && ploca[i][ j+1] == '#';
}

fn is_scaffolding(ploca : &Scaffolding, i : usize, j:usize) -> bool {
    // out of frame
    if i > ploca.len() - 1 || i < 0 || j > ploca[0].len() - 1 || j < 0 {
        return false;
    }
    // space
    return ploca[i][j] == '#';
}

fn alignment(ploca:&Scaffolding) ->usize {
    let mut sum = 0;
    for i in 0..ploca.len() {
        for j in 0..ploca[i].len() {
            if is_intersection(&ploca, i, j) {
                sum += i*j;
            }
        }
    }
    return sum;
}

fn test() {
    let test_input = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

    let ploca = parse(test_input);
    draw(&ploca);
    println!("alignment {}", alignment(&ploca));
    walk(&ploca);
}

fn walk(ploca: &Scaffolding) -> String {
    let mut weg = String::new();
    let mut robot = (16, 36);
    let mut direction = 'U';
    loop {
        let mut dstep = (0, 0);
        let mut turn_step = (0, 0);
        if direction == 'U' {
            dstep = (0,-1);
            turn_step = (-1,0);
        } else if direction == 'D' {
            dstep = (0,1);
            turn_step = (1,0);
        } else if direction == 'L' {
            dstep = (-1,0);
            turn_step = (0,1);
        } else if direction == 'R' {
            dstep = (1,0);
            turn_step = (0,-1);
        }

        let mut steps = 0;
        //println!(" {:?}  {:?}, {:?}", robot, dstep, turn_step);
        while is_scaffolding(&ploca, (robot.0 as isize+dstep.0) as usize, (robot.1 as isize +dstep.1) as usize) {
            robot.0 = (robot.0 as isize + dstep.0) as usize;
            robot.1 = (robot.1 as isize + dstep.1) as usize;
            steps += 1;
            //println!("stepping ahead");
        }
        weg += &steps.to_string();

        let mut new_direction = 'X';
        if is_scaffolding(&ploca, (robot.0 as isize+turn_step.0) as usize, (robot.1 as isize+turn_step.1)  as usize) {
            //println!("turning left");
            new_direction = 'L';
        }
        if is_scaffolding(&ploca, (robot.0 as isize -turn_step.0) as usize, (robot.1 as isize -turn_step.1) as usize) {
            //println!("turning right");
            new_direction = 'R';
        }

        if new_direction == 'X' {
            break;
        }
        fn turn_new_direction(direction: char, new_direction: char) -> char {
            match direction {
                'U' => if new_direction == 'L' {'L'} else {'R'}
                'D' => if new_direction == 'L' {'R'} else {'L'}
                'L' => if new_direction == 'L' {'D'} else {'U'}
                'R' => if new_direction == 'L' {'U'} else {'D'}
                _ => 'K'
            }
        }
        if direction == 'U' {
            if new_direction == 'L' {
                direction = 'L';
            } else {
                direction = 'R';
            }
        } else if direction == 'D' {
            if new_direction == 'L' {
                direction = 'R';
            } else {
                direction = 'L';
            }
        } else if direction == 'L' {
            if new_direction == 'L' {
                direction = 'D';
            } else {
                direction = 'U';
            }
        } else if direction == 'R' {
            if new_direction == 'L' {
                direction = 'U';
            } else {
                direction = 'D';
            }
        }
        weg.push(if new_direction == 'L' {'R'} else {'L'});
    }
    return weg.to_string()
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
        "test" => test(),
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
                let v = inputs.pop_front().unwrap();
                self.set(idx,  v);
                println!("INPUT {} at {}",v,idx);
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
