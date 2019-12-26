use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let img_width = 25;
    let img_height = 6;
    let file = File::open("./inputmain");

    let file = match file {
        Ok(f) => f,
        Err(err) => {
            panic!("There was an issue opening the file: {:?}", err);
        }
    };

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let res = reader.read_to_string(&mut contents);
    if res.is_err() {
        panic!("Could not read file to string");
    }
    contents = contents.trim().to_string();
    // Setup Image
    let mut image: Vec<Vec<Vec<i32>>> = vec![];

    let char_vec: Vec<i32> = contents.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let mut i = 0;
    while i < char_vec.len() {
        let mut tmp_j: Vec<Vec<i32>> = vec![];
        for _ in 0..img_height {
            let mut tmp_k: Vec<i32> = vec![];
            for _ in 0..img_width {
                tmp_k.push(char_vec[i]);
                i = i + 1;
            }
            tmp_j.push(tmp_k);
        }
        image.push(tmp_j);
    }
    println!("{:?}", image);

    let mut layer_min_0 = 0;
    let mut zero_count = 1000000;
    for i in 0..image.len() {
        let mut zc = 0;
        for x in image[i].clone() {
            for y in x {
                if y == 0 {
                    zc = zc + 1;
                }
            }
        }
        if zc < zero_count {
            zero_count = zc;
            layer_min_0 = i;
        }
    }
    println!("{:?}, {:?}", layer_min_0, zero_count);

    let mut one_c = 0;
    let mut two_c = 0;
    for x in image[layer_min_0].clone() {
        for y in x {
            if y == 1 {
                one_c = one_c + 1;
            }
            if y == 2 {
                two_c = two_c + 1;
            }
        }
    }
    println!("{:?} ({}, {})", one_c * two_c, one_c, two_c);

    // Part 2
    let mut rendered: Vec<Vec<i32>> = vec![];

    for y in 0..img_height {
        let mut tmp_x: Vec<i32> = vec![];
        for x in 0..img_width {
            for l in image.clone() {
                if l[y][x] != 2 {
                    tmp_x.push(l[y][x]);
                    break;
                }
            }
        }
        rendered.push(tmp_x);
    }

    for r in rendered {
        println!("{:?}", r);
    }
}
