use std::fs;
use std::io::{stdin, stdout, Read, Write};

pub struct Computer {
    memory: Vec<i64>,
    pc: usize,

    in_reg: Option<i64>,
    out_reg: Option<i64>,

    base_reg: usize,

    eom: bool,

    debug: bool,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: vec![99],
            pc: 0,
            in_reg: None,
            out_reg: None,
            base_reg: 0,
            eom: false,
            debug: false,
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
            let val: i64 = dat_vec[i].parse::<i64>().unwrap();
            self.memory.push(val);
        }
    }

    // Ehh, the next two functions act as a north/southbridge
    fn ext_read(&mut self) -> Option<i64> {
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

    fn ext_write(&mut self, val: Option<i64>) {
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

    fn get_arg_one(&mut self) -> i64 {
        let mut arg1: i64 = self.memory[self.pc + 1];
        if self.memory[self.pc] / 100 % 10 == 0 {
            if self.memory[self.pc + 1] as usize > self.memory.len() {
                println!("GROW: {}", self.memory[self.pc + 1] as usize - self.memory.len());
                for _ in 0..(self.memory[self.pc + 1] as usize - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            arg1 = self.memory[self.memory[self.pc + 1] as usize];
        } else if self.memory[self.pc] / 100 % 10 == 2 {
            let val: i64 = self.base_reg as i64 + arg1;
            if val as usize > self.memory.len() {
                println!("GPOW: {}", val as usize - self.memory.len());
                for _ in 0..(val as usize - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            arg1 = self.memory[val as usize];
        }
        return arg1;
    }

    fn get_arg_two(&mut self) -> i64 {
        let mut arg1: i64 = self.memory[self.pc + 2];
        if self.memory[self.pc] / 1000 % 10 == 0 {
            if self.memory[self.pc + 2] as usize > self.memory.len() {
                println!("GROW2: {}", self.memory[self.pc + 1] as usize - self.memory.len());
                for _ in 0..(self.memory[self.pc + 2] as usize - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            arg1 = self.memory[self.memory[self.pc + 2] as usize];
        } else if self.memory[self.pc] / 1000 % 10 == 2 {
            let val: i64 = self.base_reg as i64 + arg1;
            if val as usize > self.memory.len() {
                println!("GPOW2: {}", val as usize - self.memory.len());
                for _ in 0..(val as usize - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            arg1 = self.memory[val as usize];
        }
        return arg1;
    }

    fn add(&mut self) {
        if self.debug {
            println!("{:?}: ADD", self.pc);
        }
        if self.pc + 3 > self.memory.len() {
            panic!("On ADD opcode, but not enough data left to add");
        }
        let arg1 = self.get_arg_one();
        let arg2 = self.get_arg_two();

        let arg3: i64 = self.memory[self.pc + 3];
        if arg3 as usize > self.memory.len() - 1 {
            for _ in 0..(arg3 as usize - self.memory.len()) + 1 {
                self.memory.push(0);
            }
        }

        self.memory[arg3 as usize] = arg1 + arg2;

        self.pc = self.pc + 4;
    }

    fn mul(&mut self) {
        if self.debug {
            println!("{:?}: MUL", self.pc);
        }
        if self.pc + 3 > self.memory.len() {
            panic!("On MUL opcode, but not enough data left to add");
        }
        let arg1 = self.get_arg_one();
        let arg2 = self.get_arg_two();

        let arg3: i64 = self.memory[self.pc + 3];
        if arg3 as usize > self.memory.len() - 1 {
            for _ in 0..(arg3 as usize - self.memory.len()) + 1 {
                self.memory.push(0);
            }
        }

        self.memory[arg3 as usize] = arg1 * arg2;
        println!("\t{:?} * {:?} = [{:?}],{:?}", arg1, arg2, arg3, self.memory[arg3 as usize]);

        self.pc = self.pc + 4;
    }

    fn gets(&mut self) {
        if self.debug {
            println!("{:?}: GETS", self.pc);
        }
        if self.pc + 1 > self.memory.len() {
            panic!("On GETS opcode, but not enough data left to get get input");
        }

        match self.in_reg {
            Some(v) => {
                // Get Value
                let arg1 = self.get_arg_one();
                let m:usize = self.memory[self.pc + 1] as usize;
                self.memory[self.base_reg + m] = v;

                // Clear register
                self.in_reg = None;
                println!("\t{}, [{:?}], {}", v, self.base_reg + m, self.memory[arg1 as usize]);
                self.pc = self.pc + 2;
            }
            None => {
                // Waiting for input, so do nothing
            }
        }
    }

    fn puts(&mut self) {
        if self.debug {
            println!("{:?}: PUTS {:?}", self.pc, self.pc + 1);
        }
        if self.pc + 1 > self.memory.len() {
            panic!("On PUTS opcode, but not enough data left to get put data");
        }

        let arg1 = self.get_arg_one();

        if self.debug {
            println!("The value at {:?} -> {:?}", self.pc + 1, arg1);
        }
        println!("{}", arg1);
        self.out_reg = Some(arg1);
        self.pc = self.pc + 2;
    }

    fn jnz(&mut self) {
        if self.debug {
            println!("{:?}: JNZ", self.pc);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On JNZ opcode, but not enough data left to get put data");
        }

        let arg1 = self.get_arg_one();
        let arg2 = self.get_arg_two();

        if arg1 != 0 {
            self.pc = arg2 as usize;
            println!("\t{:?} != 0, {:?}, PC: {:?}", arg1, arg2, self.pc);
            return;
        }
        println!("\t{:?} == 0, {:?}, PC: {:?}", arg1, arg2, self.pc);
        self.pc = self.pc + 3;
    }

    fn jz(&mut self) {
        if self.debug {
            println!("{:?}: JZ", self.pc);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On JZ opcode, but not enough data left to get put data");
        }

        let arg1 = self.get_arg_one();
        let arg2 = self.get_arg_two();

        if arg1 == 0 {
            self.pc = arg2 as usize;
            println!("\t{:?} == 0, {:?}, PC: {:?}", arg1, arg2, self.pc);
            return;
        }

        println!("\t{:?} != 0, {:?}, PC: {:?}", arg1, arg2, self.pc);
        self.pc = self.pc + 3;
    }

    fn lt(&mut self) {
        if self.debug {
            println!("{:?}: LT", self.pc);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On LT opcode, but not enough data left to get put data");
        }

        let arg1 = self.get_arg_one();
        let arg2 = self.get_arg_two();

        let arg3: i64 = self.memory[self.pc + 3];
        if arg3 as usize > self.memory.len() - 1 {
            for _ in 0..(arg3 as usize - self.memory.len()) + 1 {
                self.memory.push(0);
            }
        }

        if arg1 < arg2 {
            self.memory[arg3 as usize] = 1;
        } else {
            self.memory[arg3 as usize] = 0;
        }

        println!("\t{:?} < {:?} = [{:?}],{:?}", arg1, arg2, arg3, self.memory[arg3 as usize]);

        self.pc = self.pc + 4;
    }

    fn eq(&mut self) {
        if self.debug {
            println!("{:?}: EQ", self.pc);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On EQ opcode, but not enough data left to get put data");
        }

        let arg1 = self.get_arg_one();
        let arg2 = self.get_arg_two();

        let arg3: i64 = self.memory[self.pc + 3];
        if arg3 as usize > self.memory.len() - 1 {
            for _ in 0..(arg3 as usize - self.memory.len()) + 1 {
                self.memory.push(0);
            }
        }

        if arg1 == arg2 {
            self.memory[arg3 as usize] = 1;
        } else {
            self.memory[arg3 as usize] = 0;
        }
        self.pc = self.pc + 4;
    }

    fn base(&mut self) {
        if self.debug {
            println!("{:?}: BASE ({})", self.pc, self.base_reg);
        }
        if self.pc + 1 > self.memory.len() {
            panic!("On BASE opcode, but not enough data left to get data");
        }

        let arg1 = self.get_arg_one();

        let bval = self.base_reg as i64 + arg1;
        if bval < 0 {
            self.base_reg = 0;
        } else {
            self.base_reg = bval as usize;
        }
        println!("\t{:?}, Base: {:?}", arg1, self.base_reg);
        self.pc = self.pc + 2;
    }

    fn tick(&mut self) {
        if self.pc > self.memory.len() {
            for _ in 0..(self.pc - self.memory.len()) + 1 {
                self.memory.push(0);
            }
        }
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
            9 => {
                self.base();
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

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    let mut prog = fs::read_to_string("./inputmain").expect("Had an error reading file");
    prog = prog.trim().to_string();

    let mut comp = Computer::new();
    comp.reset(prog);
    comp.debug = true;
    comp.ext_write(Some(1));

    while !comp.eom {
        comp.tick();
    }
}
