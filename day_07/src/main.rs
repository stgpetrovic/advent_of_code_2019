use std::fs;
use std::env;
use std::isize;

extern crate permutate;
use permutate::Permutator;

struct IntMachine {
  pub state: Vec<isize>,
  pub last_out : isize,
  pub pc : usize,
}

impl IntMachine {
  pub fn cont(&mut self, inputs : &Vec<isize>) ->isize {
      return self.run(self.pc,inputs);
  }

  pub fn run(&mut self, start_i:usize, inputs: &Vec<isize>) -> isize {
    let mut consumed = 0;
    let mut i = start_i;
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
                self.state[idx] = inputs[consumed];
                consumed += 1;
                println!("inputing {} at {}",inputs[consumed-1],idx);
                i+=2;
            },
            4 => {
                let p1 = param(param_mask, i + 1, 0, &self.state);
                self.last_out = p1;
                println!("returning {}", p1);
                self.pc = i+2;
                return p1;
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
            99 => {return isize::MAX;},
            _ => println!("error opcode {}", i),
        }
    }
    println!("ZESCA SU SRANJA NA {}", self.pc);
    return -1;
  }
}

const P : &[&str] = &[ "01234", "01243", "01324", "01342", "01432", "01423", "02134", "02143", "02314", "02341", "02431", "02413", "03214", "03241", "03124", "03142", "03412", "03421", "04231", "04213", "04321", "04312", "04132", "04123", "10234", "10243", "10324", "10342", "10432", "10423", "12034", "12043", "12304", "12340", "12430", "12403", "13204", "13240", "13024", "13042", "13402", "13420", "14230", "14203", "14320", "14302", "14032", "14023", "21034", "21043", "21304", "21340", "21430", "21403", "20134", "20143", "20314", "20341", "20431", "20413", "23014", "23041", "23104", "23140", "23410", "23401", "24031", "24013", "24301", "24310", "24130", "24103", "31204", "31240", "31024", "31042", "31402", "31420", "32104", "32140", "32014", "32041", "32401", "32410", "30214", "30241", "30124", "30142", "30412", "30421", "34201", "34210", "34021", "34012", "34102", "34120", "41230", "41203", "41320", "41302", "41032", "41023", "42130", "42103", "42310", "42301", "42031", "42013", "43210", "43201", "43120", "43102", "43012", "43021", "40231", "40213", "40321", "40312", "40132", "40123"];

const R : &[&str] = &[ "56789", "56798", "56879", "56897", "56987", "56978", "57689", "57698", "57869", "57896", "57986", "57968", "58769", "58796", "58679", "58697", "58967", "58976", "59786", "59768", "59876", "59867", "59687", "59678", "65789", "65798", "65879", "65897", "65987", "65978", "67589", "67598", "67859", "67895", "67985", "67958", "68759", "68795", "68579", "68597", "68957", "68975", "69785", "69758", "69875", "69857", "69587", "69578", "76589", "76598", "76859", "76895", "76985", "76958", "75689", "75698", "75869", "75896", "75986", "75968", "78569", "78596", "78659", "78695", "78965", "78956", "79586", "79568", "79856", "79865", "79685", "79658", "86759", "86795", "86579", "86597", "86957", "86975", "87659", "87695", "87569", "87596", "87956", "87965", "85769", "85796", "85679", "85697", "85967", "85976", "89756", "89765", "89576", "89567", "89657", "89675", "96785", "96758", "96875", "96857", "96587", "96578", "97685", "97658", "97865", "97856", "97586", "97568", "98765", "98756", "98675", "98657", "98567", "98576", "95786", "95768", "95876", "95867", "95687", "95678"];

fn run(program : &Vec<isize>, cfg : &str) {
}

fn solve1(state : &Vec<isize>) {
    let mut ims = Vec::new();
    for _i in 0..5 {
      ims.push(IntMachine {state: state.to_vec(), last_out:0, pc:0});
    }

    let mut max_amp = 0;

    for p in P {
        let mut last_out = 0;
        for i in 0..ims.len() {
            let cfg = p[i..i+1].parse().unwrap();
            println!("----- running ampl({})<-{},{}", i,cfg, last_out);
            let input = vec![cfg, last_out];
            last_out = ims[i].run(0, &input);
            println!("----- ampl({})->{}", i,last_out);
            if last_out > max_amp {
                max_amp = last_out;
            }
        }
    }

    println!("max amp {}", max_amp);
}

fn solve2(state : &Vec<isize>) {
    let mut max_amp = 0;
    for p in R {

        let mut ims = Vec::new();
        for _i in 0..5 {
          ims.push(IntMachine {state: state.to_vec(), last_out:0, pc:0});
        }


        let mut last_out = 0;
        let mut finished = false;
        let mut round = 0;
        while !finished {
            println!("WOOHOO DONE ONE CIRCLE; ROUND {}", round);
            for i in 0..ims.len() {
                let cfg = p[i..i+1].parse().unwrap();
                let mut input = vec![cfg, last_out];
                if round > 0 {
                    input = vec![last_out];
                }
                println!("----- running ampl({})<-{:?}", i,input);
                last_out = ims[i].cont(&input);
                let mut fertig = false;
                if last_out == isize::MAX {
                    fertig = true;
                    last_out = ims[i].last_out;
                }
                println!("----- ampl({})->{}", i,last_out);

                if fertig {
                    println!("----- FERTIG ampl({})->{}", i,last_out);
                    if last_out > max_amp {
                        max_amp = last_out;
                    }
                    finished = true;
                }
            }
            round += 1;
        }
    }

    println!("max amp {}", max_amp);
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

