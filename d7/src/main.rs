use std::fs;
use std::io::{stdin, stdout, Read, Write};

pub struct Computer {
    memory: Vec<i32>,
    pc: usize,

    in_reg: Option<i32>,
    out_reg: Option<i32>,

    eom: bool,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: vec![99],
            pc: 0,
            in_reg: None,
            out_reg: None,
            eom: false,
        }
    }

    fn reset(&mut self, data: String) {
        self.pc = 0;
        self.in_reg = None;
        self.out_reg = None;
        self.eom = false;

        if data.len() == 0 {
            self.memory = vec![99];
            return;
        }

        self.memory = vec![];
        let program_split = data.split(",");
        let dat_vec = program_split.collect::<Vec<&str>>();
        for i in 0..dat_vec.len() {
            let val: i32 = dat_vec[i].parse::<i32>().unwrap();
            self.memory.push(val);
        }
    }

    // Ehh, the next two functions act as a north/southbridge
    fn ext_read(&mut self) -> Option<i32> {
        match self.out_reg {
            Some(v) => {
                // If there is a value, read and clear
                self.out_reg = None;
                return Some(v);
            }
            None => {
                return None;
            }
        }
    }

    fn ext_write(&mut self, val: Option<i32>) {
        match self.in_reg {
            Some(_) => {
                // There is stil a value, do nothing
            }
            None => {
                // In register has been cleared, store value
                self.in_reg = val;
            }
        }
    }

    fn add(&mut self) {
        println!("{:?}: ADD", self.pc);
        if self.pc + 3 > self.memory.len() {
            panic!("On ADD opcode, but not enough data left to add");
        }
        let mut arg1: i32 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        }
        let mut arg2: i32 = self.memory[self.pc + 2];
        if self.memory[self.pc] / 1000 % 10 == 0 {
            arg2 = self.memory[self.memory[self.pc + 2] as usize];
        }
        let arg3: i32 = self.memory[self.pc + 3];

        self.memory[arg3 as usize] = arg1 + arg2;

        self.pc = self.pc + 4;
    }

    fn mul(&mut self) {
        println!("{:?}: MUL", self.pc);
        if self.pc + 3 > self.memory.len() {
            panic!("On MUL opcode, but not enough data left to add");
        }
        let mut arg1: i32 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        }
        let mut arg2: i32 = self.memory[self.pc + 2];
        if self.memory[self.pc] / 1000 % 10 == 0 {
            arg2 = self.memory[self.memory[self.pc + 2] as usize];
        }
        let arg3: i32 = self.memory[self.pc + 3];

        self.memory[arg3 as usize] = arg1 * arg2;

        self.pc = self.pc + 4;
    }

    fn gets(&mut self) {
        println!("{:?}: GETS", self.pc);
        if self.pc + 1 > self.memory.len() {
            panic!("On GETS opcode, but not enough data left to get get input");
        }

        match self.in_reg {
            Some(v) => {
                // Get Value
                let idx = self.memory[self.pc + 1] as usize;
                self.memory[idx] = v;

                // Clear register
                self.in_reg = None;

                self.pc = self.pc + 2;
            }
            None => {
                // Waiting for input, so do nothing
            }
        }
    }

    fn puts(&mut self) {
        println!("{:?}: PUTS {:?}", self.pc, self.pc + 1);
        if self.pc + 1 > self.memory.len() {
            panic!("On PUTS opcode, but not enough data left to get put data");
        }

        let mut arg1: i32 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        }

        println!("The value at {:?} -> {:?}", self.pc + 1, arg1);
        self.out_reg = Some(arg1);
        self.pc = self.pc + 2;
    }

    fn jnz(&mut self) {
        println!("{:?}: JNZ", self.pc);
        if self.pc + 2 > self.memory.len() {
            panic!("On JNZ opcode, but not enough data left to get put data");
        }
        let mut arg1: i32 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        }
        let mut arg2: i32 = self.memory[self.pc + 2];
        if self.memory[self.pc] / 1000 % 10 == 0 {
            arg2 = self.memory[self.memory[self.pc + 2] as usize];
        }
        if arg1 != 0 {
            self.pc = arg2 as usize;
            return;
        }
        self.pc = self.pc + 3;
    }

    fn jz(&mut self) {
        println!("{:?}: JZ", self.pc);
        if self.pc + 2 > self.memory.len() {
            panic!("On JZ opcode, but not enough data left to get put data");
        }
        let mut arg1: i32 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        }
        let mut arg2: i32 = self.memory[self.pc + 2];
        if self.memory[self.pc] / 1000 % 10 == 0 {
            arg2 = self.memory[self.memory[self.pc + 2] as usize];
        }
        if arg1 == 0 {
            self.pc = arg2 as usize;
            return;
        }
        self.pc = self.pc + 3;
    }

    fn lt(&mut self) {
        println!("{:?}: LT", self.pc);
        if self.pc + 2 > self.memory.len() {
            panic!("On LT opcode, but not enough data left to get put data");
        }
        let mut arg1: i32 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        }
        let mut arg2: i32 = self.memory[self.pc + 2];
        if self.memory[self.pc] / 1000 % 10 == 0 {
            arg2 = self.memory[self.memory[self.pc + 2] as usize];
        }
        let arg3: i32 = self.memory[self.pc + 3];

        if arg1 < arg2 {
            self.memory[arg3 as usize] = 1;
        } else {
            self.memory[arg3 as usize] = 0;
        }
        self.pc = self.pc + 4;
    }

    fn eq(&mut self) {
        println!("{:?}: EQ", self.pc);
        if self.pc + 2 > self.memory.len() {
            panic!("On EQ opcode, but not enough data left to get put data");
        }
        let mut arg1: i32 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        }
        let mut arg2: i32 = self.memory[self.pc + 2];
        if self.memory[self.pc] / 1000 % 10 == 0 {
            arg2 = self.memory[self.memory[self.pc + 2] as usize];
        }
        let arg3: i32 = self.memory[self.pc + 3];

        if arg1 == arg2 {
            self.memory[arg3 as usize] = 1;
        } else {
            self.memory[arg3 as usize] = 0;
        }
        self.pc = self.pc + 4;
    }

    fn tick(&mut self) {
        match self.memory[self.pc] % 100 {
            1 => {
                self.add();
            }
            2 => {
                self.mul();
            }
            3 => {
                self.gets();
            }
            4 => {
                self.puts();
            }
            5 => {
                self.jnz();
            }
            6 => {
                self.jz();
            }
            7 => {
                self.lt();
            }
            8 => {
                self.eq();
            }
            99 => {
                // Just loop here, like a regular pc
                self.pc = self.pc;
                self.eom = true;
            }
            _ => panic!(
                "Unknown opcode {:#?}\n\n{:?}",
                self.memory[self.pc], self.memory
            ),
        }
    }
}

// Taken from https://rosettacode.org/wiki/Permutations#Rust
pub fn permutations(size: usize) -> Permutations {
    Permutations {
        idxs: (0..size).collect(),
        swaps: vec![0; size],
        i: 0,
    }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    let perms = permutations(5).collect::<Vec<_>>();

    let mut prog = fs::read_to_string("./inputmain").expect("Had an error reading file");
    prog = prog.trim().to_string();

    let mut amp_a = Computer::new();
    let mut amp_b = Computer::new();
    let mut amp_c = Computer::new();
    let mut amp_d = Computer::new();
    let mut amp_e = Computer::new();

    let mut max = 0;
    let mut max_p: Vec<usize> = vec![];

    for p in perms {
        // Reset all amplifiers
        amp_a.reset(prog.clone());
        amp_b.reset(prog.clone());
        amp_c.reset(prog.clone());
        amp_d.reset(prog.clone());
        amp_e.reset(prog.clone());

        // Set all amplifier phase setting
        amp_a.ext_write(Some(p[0] as i32 + 5));
        amp_b.ext_write(Some(p[1] as i32 + 5));
        amp_c.ext_write(Some(p[2] as i32 + 5));
        amp_d.ext_write(Some(p[3] as i32 + 5));
        amp_e.ext_write(Some(p[4] as i32 + 5));

        let mut amp_a_val = None;
        let mut amp_b_val = None;
        let mut amp_c_val = None;
        let mut amp_d_val = None;
        let mut amp_e_val = Some(0);

        let mut e_val_out = 0;

        while !amp_e.eom {
            amp_a.tick();
            amp_a.ext_write(amp_e_val);
            amp_a_val = amp_a.ext_read();

            amp_b.tick();
            amp_b.ext_write(amp_a_val);
            amp_b_val = amp_b.ext_read();

            amp_c.tick();
            amp_c.ext_write(amp_b_val);
            amp_c_val = amp_c.ext_read();

            amp_d.tick();
            amp_d.ext_write(amp_c_val);
            amp_d_val = amp_d.ext_read();

            amp_e.tick();
            amp_e.ext_write(amp_d_val);
            amp_e_val = amp_e.ext_read();
            match amp_e_val {
                None => {}
                Some(v) => {
                    e_val_out = v;
                }
            }

            println!("A: {:?}", amp_a_val);
            println!("B: {:?}", amp_b_val);
            println!("C: {:?}", amp_c_val);
            println!("D: {:?}", amp_d_val);
            println!("E: {:?}", amp_e_val);
        }

        println!("{:?}: {:?}", p, e_val_out);
        if e_val_out > max {
            max = e_val_out;
            max_p = p;
        }
    }
    println!("MAX {:?}: {:?}", max_p, max);
}
