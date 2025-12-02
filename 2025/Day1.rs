use std::fs::File;
use std::io::{BufRead, BufReader};

fn harder(rotations: Vec<i16>) -> i32 {
    let mut rot: i16 = 50;
    let mut pass: i32 = 0;
    for r in rotations {
        // Full rotations add a tick and leave it at the old counter
        // so count those first before doign additional math with part
        let full = (r / 100).abs() as i32;
        let part = r % 100;
        pass += full;

        // Keep the old rotation and update the current one. 
        // We have edge cases where, if the dial starts at 0,
        // there is the potential for double counting (i.e.
        // if the dial doesn't turn right). In all cases, the
        // partial rotation never does a tick, so set that to 0
        let rot_old = rot;
        rot += part;
        let mut tick = if rot < 0 {
            rot += 100;
            1
        } else if rot > 99 {
            rot -= 100;
            1
        } else if rot == 0 {
            1
        } else {
            0
        };
        if rot_old == 0 || part == 0 {
            tick = 0;
        }
        pass += tick;
    }
    return pass;
}

fn hard(rotations: Vec<i16>) -> i32 {
    let mut rot: i16 = 50;
    let mut pass: i32 = 0;
    for r in rotations {
        rot = (rot + r) % 100;
        if rot < 0 {
            rot += 100;
        }
        if rot == 0 {
            pass += 1;
        }
    }
    return pass;
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <FILE> <HARDER>", args[0]);
        std::process::exit(1);
    }
    let file = File::open(args[1].clone()).unwrap();
    let reader = BufReader::new(file);
    let select: bool = args[2].parse().unwrap();

    let rotations: Vec<i16> = reader.lines().map(|l| {
        let s = l.unwrap();
        let sign = match s.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => panic!("Expected L or R for direction!"),
        };
        return sign * s[1..].parse::<i16>().unwrap();
    }).collect();

    if select {
        println!("Part 2: {}", harder(rotations));
    } else {
        println!("Part 1: {}", hard(rotations));
    }
}
