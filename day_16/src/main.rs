use std::fs;
use std::env;
use std::cmp;
use std::i128;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

const RADIX: u32 = 10;

// prepisat u mod 4*i, div i. mejbi :)
fn pattern(i:usize, j:usize) -> i32 {
    let mut idx = (j+1)%((i+1)*4);
    idx = idx / (i+1);
    if idx % 4 == 1 {
        return 1;
    } else if idx % 4 == 3 {
        return -1;
    } else {
        return 0;
    }
}

fn solve1(input : &str, phases : i32) -> String {
    let mut seq: String = input.to_string();
    for phase in 0..phases {
        seq = seq.chars().enumerate().map(|ic| {
            return (seq.chars().enumerate().map(|jc| jc.1.to_digit(RADIX).unwrap() as i32*pattern(ic.0, jc.0)).sum::<i32>().abs() % 10).to_string();
        }).collect::<String>();
        println!("phase {}: {}", phase, &seq[0..8]);
    }
    return seq;
}

fn solve2(input : &str, phases: i32) -> String {
    let mut seq = input.repeat(10000);
    let message_offset = seq[0..7].parse::<usize>().unwrap();

    for _i in 0..phases {
        let mut res = vec![0; seq.len()];
        let mut sum = 0;
        for i in 1..seq.len() {
            sum += seq[seq.len()-i..seq.len()-i+1].parse::<i32>().unwrap();
            let idx :usize = res.len()-i;
            res[idx] = sum;
            println!("phase {} res {:?}", phase, res);
        }
        seq = res.into_iter().map(|i| (i.abs()%10).to_string()).collect::<String>();
    }

    println!("secret message {:?}", &seq[message_offset..message_offset+8]);
    return seq;
}

fn main() {
    // Parse input.
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let test20 = fs::read_to_string("test20")
        .expect("Something went wrong reading the file");
    let test21 = fs::read_to_string("test21")
        .expect("Something went wrong reading the file");
    let test22 = fs::read_to_string("test22")
        .expect("Something went wrong reading the file");

    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "a" => solve1(&contents, 100),
        "b" => solve2(&contents, 100),
        "test20" => solve2(&test20, 100),
        "test21" => solve2(&test21, 100),
        "test22" => solve2(&test22, 100),
        _ => "".to_string(),
    };
}

