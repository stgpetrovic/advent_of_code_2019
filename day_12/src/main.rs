use std::env;

#[derive(Clone, Eq,PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Moon {
    l: Point,
    v: Point,
}

impl Moon {
    fn energy(&self) -> i32 {
        return (self.l.x.abs() + self.l.y.abs() + self.l.z.abs()) *
               (self.v.x.abs() + self.v.y.abs() + self.v.z.abs());
    }
}

fn solve1(m : &mut Vec<Moon>) {
    for _step in 0..1000 {
        // gravity
        for i in 0..3 {
            for j in i+1..4 {
                if m[i].l.x > m[j].l.x {
                    m[j].v.x += 1;
                    m[i].v.x -= 1;
                } else if m[i].l.x < m[j].l.x {
                    m[j].v.x -= 1;
                    m[i].v.x += 1;
                }

                if m[i].l.y > m[j].l.y {
                    m[j].v.y += 1;
                    m[i].v.y -= 1;
                } else if m[i].l.y < m[j].l.y {
                    m[j].v.y -= 1;
                    m[i].v.y += 1;
                }

                if m[i].l.z > m[j].l.z {
                    m[j].v.z += 1;
                    m[i].v.z -= 1;
                } else if m[i].l.z < m[j].l.z {
                    m[j].v.z -= 1;
                    m[i].v.z += 1;
                }
            }
        }
        // velocity
        for i in 0..4 {
            m[i].l.x += m[i].v.x;
            m[i].l.y += m[i].v.y;
            m[i].l.z += m[i].v.z;
        }
    }
    println!("energy after {}", m.iter().fold(0i32, |sum, moon| sum + moon.energy()));
}

fn solve2(m : &mut Vec<Moon>) {
    let m0 = m.clone();
    let mut x :i128 = 0; let mut y :i128 = 0; let mut z :i128 = 0; let mut step :i128 = 0;


    loop {
        for i in 0..3 {
            for j in i+1..4 {
                if m[i].l.x > m[j].l.x {
                    m[j].v.x += 1;
                    m[i].v.x -= 1;
                } else if m[i].l.x < m[j].l.x {
                    m[j].v.x -= 1;
                    m[i].v.x += 1;
                }

                if m[i].l.y > m[j].l.y {
                    m[j].v.y += 1;
                    m[i].v.y -= 1;
                } else if m[i].l.y < m[j].l.y {
                    m[j].v.y -= 1;
                    m[i].v.y += 1;
                }

                if m[i].l.z > m[j].l.z {
                    m[j].v.z += 1;
                    m[i].v.z -= 1;
                } else if m[i].l.z < m[j].l.z {
                    m[j].v.z -= 1;
                    m[i].v.z += 1;
                }
            }
        }
        // velocity
        for i in 0..4 {
            m[i].l.x += m[i].v.x;
            m[i].l.y += m[i].v.y;
            m[i].l.z += m[i].v.z;
        }
        step += 1;

        if x == 0 {
            let mut arrived = true;
            for (i, moon) in m.iter().enumerate() {
                if moon.l.x != m0[i].l.x || moon.v.x != m0[i].v.x {
                    arrived = false;
                    break;
                }
            }
            if arrived {
                x = step;
            }
        }

        if y == 0 {
            let mut arrived = true;
            for (i, moon) in m.iter().enumerate() {
                if moon.l.y != m0[i].l.y || moon.v.y != m0[i].v.y {
                    arrived = false;
                    break;
                }
            }
            if arrived {
                y = step;
            }
        }

        if z == 0 {
            let mut arrived = true;
            for (i, moon) in m.iter().enumerate() {
                if moon.l.z != m0[i].l.z || moon.v.z != m0[i].v.z {
                    arrived = false;
                    break;
                }
            }
            if arrived {
                z = step;
            }
        }

        if x != 0 && y != 0 && z != 0  {
            break;
        }
    }
    println!("took {} to repeat", num::integer::lcm(z, num::integer::lcm(x, y)));
}

fn main() {

    let mut m = vec![
        Moon{l: Point{x: -7, y: 17, z: -11}, v: Point{x: 0, y: 0, z: 0}},
        Moon{l: Point{x: 9, y: 12, z: 5}, v: Point{x: 0, y: 0, z: 0}},
        Moon{l: Point{x: -9, y: 0, z: -4}, v: Point{x: 0, y: 0, z: 0}},
        Moon{l: Point{x: 4, y: 6, z: 0}, v: Point{x: 0, y: 0, z: 0}},
    ];

    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "a" => solve1(&mut m),
        "b" => solve2(&mut m),
        _ => println!("BOOOM"),
    };
}
