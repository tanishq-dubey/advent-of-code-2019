use std::fs;
use std::io;

//noinspection ALL
fn add(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: ADD", pc);
    if *pc + 3 > dat.len() {
        panic!("On ADD opcode, but not enough data left to add");
    }

    let mut arg1:i32 = dat[*pc + 1];
    if dat[*pc]/100%10 == 0 {
        arg1 = dat[dat[*pc + 1] as usize];
    }
    let mut arg2:i32 = dat[*pc + 2];
    if dat[*pc]/1000%10 == 0 {
        arg2 = dat[dat[*pc + 2] as usize];
    }
    let arg3:i32 = dat[*pc + 3];

    dat[arg3 as usize] = arg1 + arg2;

    *pc = *pc + 4;
}

//noinspection ALL
fn mul(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: MUL", pc);
    if *pc + 3 > dat.len() {
        panic!("On MUL opcode, but not enough data left to multiply");
    }

    let mut arg1:i32 = dat[*pc + 1];
    if dat[*pc]/100%10 == 0 {
        arg1 = dat[dat[*pc + 1] as usize];
    }
    let mut arg2:i32 = dat[*pc + 2];
    if dat[*pc]/1000%10 == 0 {
        arg2 = dat[dat[*pc + 2] as usize];
    }
    let arg3:i32 = dat[*pc + 3];

    dat[arg3 as usize] = arg1 * arg2;

    *pc = *pc + 4;
}

fn gets(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: GETS", pc);
    if *pc + 1 > dat.len() {
        panic!("On GETS opcode, but not enough data left to get get input");
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Could not get user input");

    dat[dat[*pc + 1] as usize] = input.trim().parse::<i32>().unwrap();

    *pc = *pc + 2;
}

fn puts(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: PUTS {:?}", pc, *pc + 1);
    if *pc + 1 > dat.len() {
        panic!("On PUTS opcode, but not enough data left to get put data");
    }

    let mut arg1:i32 = dat[*pc + 1];
    if dat[*pc]/100%10 == 0 {
        arg1 = dat[dat[*pc + 1] as usize];
    }

    println!("The value at {:?} -> {:?}", *pc + 1, arg1);
    *pc = *pc + 2;
}

fn jnz(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: JNZ", pc);
    if *pc + 2 > dat.len() {
        panic!("On JNZ opcode, but not enough data left to get put data");
    }
    let mut arg1:i32 = dat[*pc + 1];
    if dat[*pc]/100%10 == 0 {
        arg1 = dat[dat[*pc + 1] as usize];
    }
    let mut arg2:i32 = dat[*pc + 2];
    if dat[*pc]/1000%10 == 0 {
        arg2 = dat[dat[*pc + 2] as usize];
    }
    if arg1 != 0 {
        *pc = arg2 as usize;
        return;
    }
    *pc = *pc + 3;
}

fn jz(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: JZ", pc);
    if *pc + 2 > dat.len() {
        panic!("On JZ opcode, but not enough data left to get put data");
    }
    let mut arg1:i32 = dat[*pc + 1];
    if dat[*pc]/100%10 == 0 {
        arg1 = dat[dat[*pc + 1] as usize];
    }
    let mut arg2:i32 = dat[*pc + 2];
    if dat[*pc]/1000%10 == 0 {
        arg2 = dat[dat[*pc + 2] as usize];
    }
    if arg1 == 0 {
        *pc = arg2 as usize;
        return;
    }
    *pc = *pc + 3;
}

fn lt(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: LT", pc);
    if *pc + 2 > dat.len() {
        panic!("On LT opcode, but not enough data left to get put data");
    }
    let mut arg1:i32 = dat[*pc + 1];
    if dat[*pc]/100%10 == 0 {
        arg1 = dat[dat[*pc + 1] as usize];
    }
    let mut arg2:i32 = dat[*pc + 2];
    if dat[*pc]/1000%10 == 0 {
        arg2 = dat[dat[*pc + 2] as usize];
    }
    let arg3:i32 = dat[*pc + 3];

    if arg1 < arg2 {
        dat[arg3 as usize] = 1;
    } else {
        dat[arg3 as usize] = 0;
    }
    *pc = *pc + 4;
}

fn eq(dat: &mut [i32], pc:&mut usize) {
    println!("{:?}: EQ", pc);
    if *pc + 2 > dat.len() {
        panic!("On EQ opcode, but not enough data left to get put data");
    }
    let mut arg1:i32 = dat[*pc + 1];
    if dat[*pc]/100%10 == 0 {
        arg1 = dat[dat[*pc + 1] as usize];
    }
    let mut arg2:i32 = dat[*pc + 2];
    if dat[*pc]/1000%10 == 0 {
        arg2 = dat[dat[*pc + 2] as usize];
    }
    let arg3:i32 = dat[*pc + 3];

    if arg1 == arg2 {
        dat[arg3 as usize] = 1;
    } else {
        dat[arg3 as usize] = 0;
    }
    *pc = *pc + 4;
}

fn run(d_str: &str) {
    let program_split = d_str.split(",");
    let dat_str = program_split.collect::<Vec<&str>>();
    let mut dat = Vec::new();
    for i in 0..dat_str.len() {
        let int : i32 = dat_str[i].parse::<i32>().unwrap();
        dat.push(int);
    }

    let mut pc : usize = 0;
    while pc < dat.len() {
        match dat[pc] % 100 {
            1 => {
                add(dat.as_mut_slice(), &mut pc);
            }
            2 => {
                mul(dat.as_mut_slice(), &mut pc);
            }
            3 => {
                gets(dat.as_mut_slice(), &mut pc);
            }
            4 => {
                puts(dat.as_mut_slice(), &mut pc);
            }
            5 => {
                jnz(dat.as_mut_slice(), &mut pc);
            }
            6 => {
                jz(dat.as_mut_slice(), &mut pc);
            }
            7 => {
                lt(dat.as_mut_slice(), &mut pc);
            }
            8 => {
                eq(dat.as_mut_slice(), &mut pc);
            }
            99 => {
                pc = dat.len() + 1;
            }
            _ => {
                panic!("Unknown opcode {:#?}\n\n{:?}", dat[pc], dat)
            }
        }
    }
}

fn main() {
    let mut prog = fs::read_to_string("./input").expect("Had an error reading file");
    prog = prog.trim().to_string();
    run(&prog);
}
