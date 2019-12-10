use std::env;
use std::fs;
use std::usize;

const RADIX: u32 = 10;
const W :usize = 25;
const H :usize = 6;

type Layer = [[usize; W]; H];

fn layers(input : &Vec<u32>) -> Vec<Layer> {
    let mut layers = Vec::<Layer>::new();

    for l in 0..input.len()/(W * H)-1 {
        let mut layer : Layer = [[0; W]; H] ;
        for i in 0..H {
            for j in 0..W {
                layer[i][j] = input[l*W*H+i*W+j as usize] as usize;
            }
        }
        layers.push(layer);
    }

    return layers;
}

fn count_in_layer(layer : Layer) -> (usize, usize, usize) {
    let mut count_zero = 0;
    let mut count_eins = 0;
    let mut count_zwei = 0;
    for i in 0..H {
      for j in 0..W {
          match layer[i][j] {
              0 => count_zero += 1,
              1 => count_eins += 1,
              2 => count_zwei += 1,
              _ => println!("Exlode"),
          };
      }
    }
    return (count_zero, count_eins, count_zwei);
}

fn solve1(layers : &Vec<Layer>) -> usize {
    let mut min_zeros = usize::MAX;
    let mut mul = 0;

    for l in layers {
        let counts = count_in_layer(*l);
        if min_zeros > counts.0 {
            min_zeros = counts.0;
            mul = counts.1 * counts.2;
        }
    }
    return mul;
}

fn solve2(layers : &Vec<Layer>) -> usize {
    let mut image : Layer = [[2; W]; H] ;  // transparent

    for l in layers {
        for i in 0..H {
            for j in 0..W {
                if image[i][j] == 2 {
                    image[i][j] = l[i][j];
                }
            }
        }
    }

    for i in 0..H {
        for j in 0..W {
            match image[i][j] {
              0 => print!("□"),
              1 => print!("▓"),
              2 => print!(" "),
              _ => print!("?"),
            }
        }
        println!("");
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let input: Vec<u32> = contents.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
    let layers = layers(&input);

    println!("{}", match args[1].as_ref() {
        "a" => solve1(&layers),
        "b" => solve2(&layers),
        _ => std::usize::MAX,
    });
}
