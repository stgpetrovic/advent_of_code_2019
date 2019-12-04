extern crate digits;
use std::env;
use digits::{BaseCustom,Digits};

const MIN :i32 = 246540;
const MAX :i32 = 787419;

fn max_same_adj(n : i32) -> usize {
    let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
    let num = Digits::new(&base10, n.to_string());
    return num.max_adjacent() + 1;
}

fn has_double(n: i32) -> bool {
    let mut streak = 1;
    let mut last = 'a';
    for c in n.to_string().chars() {
        if c == last {
            streak += 1;
        } else {
            last = c;
            if streak == 2 {
                return true;
            }
            streak = 1;
        }
    }
    if streak == 2 {
        return true;
    }
    return false;
}

fn is_increasing(n : i32) -> bool {
    let mut last = n % 10;
    let mut x = n / 10;
    while x > 9 {
        let new_last = x % 10;
        if new_last > last {
            return false;
        }
        last = new_last;
        x = x /10;
    }
    return x <= last;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let count = match args[1].as_ref() {
        "a" => (MIN..MAX).filter(|n| (max_same_adj(*n) > 1) && is_increasing(*n)).count(),
        "b" => (MIN..MAX).filter(|n| has_double(*n) && is_increasing(*n)).count(),
        _ => 0,
    };
    println!("{}", count);
}
