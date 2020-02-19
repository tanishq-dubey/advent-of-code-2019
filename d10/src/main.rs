#![feature(vec_remove_item)]
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn gcd(m: i32, n: i32) -> i32 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}

fn main() {

    let file = File::open("./inputmain").expect("file does not exist");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("could not parse line")).collect();
    let field_max_x: i32 = lines.len() as i32;
    let field_max_y: i32 = lines.len() as i32;
    let mut asteroids: Vec<(i32, i32)> = vec![];
    for i in 0..lines.len() {
        let line: String = lines.get(i).unwrap().to_string();
        let chars: Vec<char> = line.chars().collect();
        for j in 0..chars.len() {
            let char: String = chars.get(j).unwrap().to_string();
            if char == "#" {
                asteroids.push((j as i32, i as i32));
            }
        }
    }


    let mut max_view: (i32, i32) = (0, 0);
    let mut max_view_val: i32 = 0;
    let asteroids_to_see = asteroids.clone();
    for asteroid in asteroids {
        println!("Currently viewing from {:?}", asteroid);
        let mut current_view = asteroids_to_see.clone();
        current_view.remove_item(&asteroid);
        let mut view_list = current_view.clone();
        for target in current_view {
            let rx: i32 = target.0 - asteroid.0;
            let ry: i32 = target.1 - asteroid.1;
            let div: i32 = gcd(rx, ry);
            let dx: i32 = rx/div;
            let dy: i32 = ry/div;
            let mut cx: i32 = target.0 + dx;
            let mut cy: i32 = target.1 + dy;
            println!("\tT: {:?} \t D: {:?}, {:?} \t C: {:?}, {:?}", target, dx, dy, cx, cy);
            while cx < field_max_x && cx >= 0 && cy < field_max_y && cy >= 0 {
                if view_list.contains(&(cx, cy)) {
                    println!("\t\tremoving {:?}", (cx, cy));
                    view_list.remove_item(&(cx, cy));
                }
                cx = cx + dx;
                cy = cy + dy;
            }
        }
        if view_list.len() as i32 > max_view_val {
            max_view_val = view_list.len() as i32;
            max_view = (asteroid.0, asteroid.1);
        }
    }
    println!("{:?} seen at {:?}", max_view_val, max_view);
}
