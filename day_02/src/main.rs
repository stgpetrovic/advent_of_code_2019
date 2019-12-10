use std::fs;
use std::env;
use intmachine;

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let state: Vec<usize> = contents.split(",").map(|s| s.parse().unwrap()).collect();

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&state),
        "b" => solve2(&state),
        _ => println!("error input, only a or b allowed"),
    }
}

fn solve1(state: &Vec<usize>) {
    let mut im = Box::new(intmachine::intmachine::IntMachine {state: state.to_vec()});
    im.init(12, 2);
    let v = im.run();
    println!("{}", v);
}

fn solve2(state : &Vec<usize>) {
    for i in 0..99 {
      for j in 0..99 {
          let mut im = Box::new(intmachine::intmachine::IntMachine {state: state.to_vec()});
          im.init(i, j);
          if im.run() == 19690720 {
              println!("{}", 100*i+j);
              return;
          }
      }
    }
}
