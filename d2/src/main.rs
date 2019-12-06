use std::fs;

fn add(mut dat: *Vec<i32>, src1: usize, src2: usize, dest: usize) {
    dat[dest] = dat[src1] + dat[src2];
}

fn mul(mut dat: *Vec<i32>, src1: usize, src2: usize, dest: usize) {
    dat[dest] = dat[src1] * dat[src2];
}

fn main() {
    let mut prog = fs::read_to_string("./input").expect("Had an error reading file");
    prog.pop();
    let prog_split = prog.split(",");

    let datstr = prog_split.collect::<Vec<&str>>();
    let mut dat = Vec::new();
    for i in 0..datstr.len() {
        let int : i32 = datstr[i].parse::<i32>().unwrap();
        dat.push(int);
    }

    let mut pc : usize = 0;
    while pc < dat.len() {
        match dat[pc] {
            1 => { 
                add(&dat, dat[pc+1] as usize, dat[pc+2] as usize, dat[pc+3] as usize); 
                pc = pc + 4;
            }
            2 => { 
                mul(&dat, dat[pc+1] as usize, dat[pc+2] as usize, dat[pc+3] as usize); 
                pc = pc + 4;
            }
            _ => {
               panic!("Unknown opcode {:#?}", dat[pc]) 
            }
        }
    }
}
