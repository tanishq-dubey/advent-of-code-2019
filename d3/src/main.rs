use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

#[derive(Debug, Copy, Clone)]
struct Segment {
    start: Point,
    end: Point,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn do_intersection(p1: Point, p2: Point, p3: Point, p4: Point) -> (Point, bool) {
    let numer: i32 = ((p3.y - p4.y) * (p1.x - p3.x)) + ((p4.x - p3.x) * (p1.y - p3.y));
    let denom: i32 = ((p4.x - p3.x) * (p1.y - p2.y)) - ((p1.x - p2.x) * (p4.y - p3.y));
    let numer_check: i32 = ((p1.y - p2.y) * (p1.x - p3.x)) + ((p2.x - p1.x) * (p1.y - p3.y));
    let denom_check: i32 = ((p4.x - p3.x) * (p1.y - p2.y)) - ((p1.x - p2.x) * (p4.y - p3.y));
    if denom == 0 || denom_check == 0 {
        return (Point{ x: 0, y: 0}, false);
    }

    let t: f32 = numer as f32/denom as f32;
    let t_check: f32 = numer_check as f32/denom_check as f32;
    if (t > 0.0 && t <= 1.0) && (t_check > 0.0 && t_check <= 1.0) {
       return (Point{ x: p1.x + (t * ((p2.x - p1.x) as f32)) as i32, y: p1.y + (t * (p2.y - p1.y) as f32) as i32}, true);
    }
    return (Point{ x: 0, y: 0}, false);

}

fn skip_string_num(s: &str, skip: usize) -> i32 {
    let num: i32 = match s.chars().skip(skip).collect::<String>().parse::<i32>() {
        Ok(n) => n,
        Err(err) => {
            panic!("Could not parse int at {}: {:?}", s, err);
        }
    };
    return num;
}

fn on_wire_distance(w: Vec<Segment>, target: Point) -> i32 {
    let mut dist: i32 = 0;
    for s in w {
        // check if point is on segment
        let cross: i32 = (target.y - s.start.y) * (s.end.x - s.start.x) - (target.x - s.start.x) * (s.end.y - s.start.y);
        if cross.abs() == 0 && (cmp::min(s.start.x, s.end.x) <= target.x && target.x <= cmp::max(s.start.x, s.end.x)) && (cmp::min(s.start.y, s.end.y) <= target.y && target.y <= cmp::max(s.start.y, s.end.y)) {
            println!("point {:?} is on {:?}", target, s);
            dist = dist + ((target.x  - s.start.x).abs() + (target.y - s.start.y).abs());
            return dist;
        } else {
            dist = dist + ((s.end.x  - s.start.x).abs() + (s.end.y - s.start.y).abs());
        }
    }
    return 1000000000;
}

fn main() {

    let file = File::open("./input");
    let file = match file {
        Ok(f) => f,
        Err(err) => {
            panic!("There was an issue opening the file: {:?}", err);
        }
    };
    let reader = BufReader::new(file);

    let mut wires: Vec<Vec<Segment>> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(err) => {
                panic!("Could not read line at {}: {:?}", index, err);
            }
        };

        let mut wire: Vec<Segment> = Vec::new();
        let mut begin: Point = Point{x: 0, y: 0};
        let path_enum = line.trim().split(",");
        let path = path_enum.collect::<Vec<&str>>();
        for dir in path {
            match &dir[0..1]  {
                "L" => {
                    let num: i32 = skip_string_num(dir, 1);
                    let new_seg: Segment = Segment{start: Point{x: begin.x, y: begin.y}, end: Point{x: begin.x - num, y: begin.y}};
                    wire.push(new_seg);
                    begin = new_seg.end;
                }
                "R" => {
                    let num: i32 = skip_string_num(dir, 1);
                    let new_seg: Segment = Segment{start: Point{x: begin.x, y: begin.y}, end: Point{x: begin.x + num, y: begin.y}};
                    wire.push(new_seg);
                    begin = new_seg.end;
                }
                "U" => {
                    let num: i32 = skip_string_num(dir, 1);
                    let new_seg: Segment = Segment{start: Point{x: begin.x, y: begin.y}, end: Point{x: begin.x , y: begin.y + num}};
                    wire.push(new_seg);
                    begin = new_seg.end;
                }
                "D" => {
                    let num: i32 = skip_string_num(dir, 1);
                    let new_seg: Segment = Segment{start: Point{x: begin.x, y: begin.y}, end: Point{x: begin.x , y: begin.y - num}};
                    wire.push(new_seg);
                    begin = new_seg.end;
                }
                _ => {
                    panic!("Unknown Direction");
                }
            }
        }
        wires.push(wire);
    }

    // Part 1
    let mut dist: i32 = 100000000;
    let wire_1 = &(wires[0]);
    let wire_2 = &(wires[1]);
    for i in wire_1 {
        for j in wire_2 {
            let (p, is_intersect) = do_intersection(i.start, i.end, j.start, j.end);
            if is_intersect {
                let c_dist = p.x.abs() + p.y.abs();
                if c_dist < dist {
                    println!("hit: {:?}", p);
                    dist = c_dist;
                }
            }
        }
    }
    println!("{}", dist);


    println!("\n\npt2");

    // Part 2
    dist = 1000000000;
    for i in wire_1 {
        for j in wire_2 {
            let (p, is_intersect) = do_intersection(i.start, i.end, j.start, j.end);
            if is_intersect {
                println!("intersect @ {:?}", p);
                let c_dist = on_wire_distance(wire_1.to_vec(), p) + on_wire_distance(wire_2.to_vec(), p);
                if c_dist < dist {
                    println!("hit: {:?}", p);
                    dist = c_dist;
                }
            }
        }
    }
    println!("{}", dist);
}
