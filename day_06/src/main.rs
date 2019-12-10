use std::fs;
use std::env;
use std::usize;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn solve1(arr : &Vec<Vec<usize>>, nodes : &HashMap<&str, usize>) -> usize {
    let mut q = VecDeque::new();
    q.push_back((*nodes.get("COM").unwrap(),0));

    let mut cnt = 0;
    let mut seen = HashSet::new();
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        seen.insert(v.0);
        cnt += v.1;

        for edge in &arr[v.0 as usize] {
            if !seen.contains(&edge) {
                q.push_front((*edge, v.1+1));
            }

        }
    }
    return cnt;
}

fn solve2(arr : &Vec<Vec<usize>>, nodes : &HashMap<&str, usize>) -> usize {
    return dijkstra(&arr, *nodes.get("YOU").unwrap(), *nodes.get("SAN").unwrap()).unwrap()-2;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let state: Vec<&str> = contents.split("\n").collect();

    let mut nodes = HashMap::new();

    let mut i = 0;
    for orbit in &state {
        let pair = orbit.split(")").collect::<Vec<&str>>();

        if !nodes.contains_key(pair[0]) {
            nodes.insert(pair[0], i);
            i+=1;
        }

        if !nodes.contains_key(pair[1]) {
            nodes.insert(pair[1], i);
            i+=1;
        }
    }

    let mut arr : Vec<Vec<usize>> = Vec::new();
    let mut arr1 : Vec<Vec<usize>> = Vec::new();
    for _i in 0..nodes.len() {
        arr.push(Vec::new());
        arr1.push(Vec::new());
    }

    for orbit in &state {
        let pair = orbit.split(")").collect::<Vec<&str>>();
        arr[*nodes.get(pair[0]).unwrap()].push(*nodes.get(pair[1]).unwrap());
        arr1[*nodes.get(pair[0]).unwrap()].push(*nodes.get(pair[1]).unwrap());
        arr1[*nodes.get(pair[1]).unwrap()].push(*nodes.get(pair[0]).unwrap());
    }

    println!("{}", match args[1].as_ref() {
        "a" => solve1(&arr, &nodes),
        "b" => solve2(&arr1, &nodes),
        _ => std::usize::MAX,
    });
}

fn dijkstra(adj_list: &Vec<Vec<usize>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }
        if cost > dist[position] { continue; }
        for edge in &adj_list[position] {
            let next = State { cost: cost + 1, position: *edge };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

