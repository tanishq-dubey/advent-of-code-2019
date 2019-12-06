use std::fs;

fn add(dat: &mut [i32], pc: &usize) {
    dat[*pc + 3] = dat[*pc + 1] + dat[*pc + 2];
}

fn mul(dat: &mut [i32], pc: &usize) {
    dat[*pc + 3] = dat[*pc + 1] * dat[*pc + 2];
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
                add(dat.as_mut_slice(), &pc); 
                pc = pc + 4;
            }
            2 => { 
                mul(dat.as_mut_slice(), &pc); 
                pc = pc + 4;
            }
            _ => {
               panic!("Unknown opcode {:#?}", dat[pc]) 
            }
        }
    }
}
