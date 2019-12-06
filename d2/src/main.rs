use std::fs;

fn add(dat: &mut [i32], pc: &usize) {
    dat[dat[*pc + 3] as usize] = dat[dat[*pc + 1] as usize] + dat[dat[*pc + 2] as usize];
}

fn mul(dat: &mut [i32], pc: &usize) {
    dat[dat[*pc + 3] as usize] = dat[dat[*pc + 1] as usize] * dat[dat[*pc + 2] as usize];
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

    // Pre run fixes
    //dat[1] = 12;
    //dat[2] = 2;

    println!("{:?}", dat);
    let mut pc : usize = 0;
    while pc < dat.len() {
        match dat[pc] {
            1 => {
                println!("{} Doing ADD", pc);
                if pc + 3 > dat.len() {
                    panic!("On ADD opcode, but not enough data left to add");
                }
                add(dat.as_mut_slice(), &pc); 
                pc = pc + 4;
            }
            2 => { 
                println!("{} Doing MUL", pc);
                if pc + 3 > dat.len() {
                    panic!("On MUL opcode, but not enough data left to add");
                }
                mul(dat.as_mut_slice(), &pc); 
                pc = pc + 4;
            }
            99 => {
                pc = dat.len() + 1;
            }
            _ => {
               panic!("Unknown opcode {:#?}", dat[pc]) 
            }
        }
    }

    println!("{:?}", dat);
    println!("Zero Position value is: {}", dat[0])
}
