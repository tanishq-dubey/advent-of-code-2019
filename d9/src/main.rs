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

    /*
     * Param mode 0 - Positions mode
     *  example: 000 01 5 6 7
     *  MUL [5] + [6] -> [7]
     * Param mode 1 - Immediate mode
     *  example: 010 01 5 6 7
     *  MUL [5] + 6 -> [7]
     *  write params will never be immediate
     * Param mode 2 - Relative mode
     *  example: 220 01 5 6 7
     *  MUL [5] + (BASER + [6]) -> (BASER + [7])
     */
    fn first_arg_addr(&mut self) -> usize {
        let mut retval: usize = (self.memory[self.pc + 1]) as usize;
        println!("one: {} {}", retval, self.memory[self.pc]);
        if self.memory[self.pc] / 100 % 10 == 0 {
            println!("\tone");
            if retval  > self.memory.len() {
                for _ in 0..(retval - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            return retval;
        } else if self.memory[self.pc] / 100 % 10 == 1 {
            println!("\tone");
            return self.pc + 1
        } else if self.memory[self.pc] / 100 % 10 == 2 {
            let temp_retval: i64 = self.memory[self.pc + 1] + self.base_reg as i64;
            println!("\t\ttwo {} {} {}", temp_retval, self.memory[self.pc + 1], self.base_reg);
            if temp_retval < 0 {
                retval = 0;
            } else {
                retval = temp_retval as usize;
            }
            if retval  > self.memory.len() {
                for _ in 0..(retval - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            return retval;
        }
        println!("SOME HOWE HERE??");
        return retval;
    }

    fn second_arg_addr(&mut self) -> usize {
        let mut retval: usize = (self.memory[self.pc + 2]) as usize;
        if self.memory[self.pc] / 1000 % 10 == 0 {
            if retval  > self.memory.len() {
                for _ in 0..(retval - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            return retval;
        } else if self.memory[self.pc] / 1000 % 10 == 1 {
            return self.pc + 2
        } else if self.memory[self.pc] / 1000 % 10 == 2 {
            let temp_retval: i64 = self.memory[self.pc + 2] + self.base_reg as i64;
            if temp_retval < 0 {
                retval = 0;
            } else {
                retval = temp_retval as usize;
            }
            if retval  > self.memory.len() {
                for _ in 0..(retval - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            return retval;
        }
        return retval;
    }

    fn third_arg_addr(&mut self) -> usize {
        let mut retval: usize = (self.memory[self.pc + 3]) as usize;
        if self.memory[self.pc] / 10000 % 10 == 0 {
            if retval + 1 > self.memory.len() {
                for _ in 0..(retval - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            return retval;
        } else if self.memory[self.pc] / 10000 % 10 == 1 {
            panic!("Value for {} has a immediate write val", self.pc);
        } else if self.memory[self.pc] / 10000 % 10 == 2 {
            let temp_retval: i64 = self.memory[self.pc + 3] + self.base_reg as i64;
            if temp_retval < 0 {
                retval = 0;
            } else {
                retval = temp_retval as usize;
            }
            if retval + 1 > self.memory.len() {
                for _ in 0..(retval - self.memory.len()) + 1 {
                    self.memory.push(0);
                }
            }
            return retval;
        }
        return retval;
    }

    fn add(&mut self) {
        if self.debug {
            println!("{:?}: ADD", self.pc);
        }
        if self.pc + 3 > self.memory.len() {
            panic!("On ADD opcode, but not enough data left to add");
        }

        let addr1 = self.first_arg_addr();
        let addr2 = self.second_arg_addr();
        let addr3 = self.third_arg_addr();

        self.memory[addr3] = self.memory[addr1] + self.memory[addr2];

        self.pc = self.pc + 4;
    }

    fn mul(&mut self) {
        if self.debug {
            println!("{:?}: MUL ({})", self.pc, self.memory[self.pc]);
        }
        if self.pc + 3 > self.memory.len() {
            panic!("On MUL opcode, but not enough data left to add");
        }

        let addr1 = self.first_arg_addr();
        let addr2 = self.second_arg_addr();
        let addr3 = self.third_arg_addr();
        println!("{} {} {}", addr1, addr2, addr3);

        self.memory[addr3] = self.memory[addr1] * self.memory[addr2];

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
                let addr1 = self.first_arg_addr();
                self.memory[addr1] = v;

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
        if self.debug {
            println!("{:?}: PUTS {:?}", self.pc, self.pc + 1);
        }
        if self.pc + 1 > self.memory.len() {
            panic!("On PUTS opcode, but not enough data left to get put data");
        }

        let addr1 = self.first_arg_addr();

        if self.debug {
            println!("The value at {:?} -> {:?}", self.pc + 1, self.memory[addr1]);
        }
        self.out_reg = Some(self.memory[addr1]);
        self.pc = self.pc + 2;
    }

    fn jnz(&mut self) {
        if self.debug {
            println!("{:?}: JNZ ({})", self.pc, self.memory[self.pc]);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On JNZ opcode, but not enough data left to get put data");
        }

        let addr1 = self.first_arg_addr();
        let addr2 = self.second_arg_addr();

        println!("\t{} {}", addr1, addr2);

        if self.memory[addr1] != 0 {
            self.pc = self.memory[addr2] as usize;
            println!("\t{:?} != 0, {:?}, PC: {:?}", self.memory[addr1], self.memory[addr2], self.pc);
            return;
        }

        self.pc = self.pc + 3;
    }

    fn jz(&mut self) {
        if self.debug {
            println!("{:?}: JZ", self.pc);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On JZ opcode, but not enough data left to get put data");
        }

        let addr1 = self.first_arg_addr();
        let addr2 = self.second_arg_addr();
        println!("{} {}", addr1, addr2);
        println!("{} {} -- {} {}", addr1, addr2, self.memory[addr1], self.memory[addr2]);

        if self.memory[addr1] == 0 {
            self.pc = self.memory[addr2] as usize;
            println!("\t{:?} == 0, {:?}, PC: {:?}", self.memory[addr1], self.memory[addr2], self.pc);
            return;
        }

        self.pc = self.pc + 3;
    }

    fn lt(&mut self) {
        if self.debug {
            println!("{:?}: LT", self.pc);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On LT opcode, but not enough data left to get put data");
        }

        let addr1 = self.first_arg_addr();
        let addr2 = self.second_arg_addr();
        let addr3 = self.third_arg_addr();

        if self.memory[addr1] < self.memory[addr2] {
            self.memory[addr3] = 1;
        } else {
            self.memory[addr3] = 0;
        }

        self.pc = self.pc + 4;
    }

    fn eq(&mut self) {
        if self.debug {
            println!("{:?}: EQ", self.pc);
        }
        if self.pc + 2 > self.memory.len() {
            panic!("On EQ opcode, but not enough data left to get put data");
        }

        let addr1 = self.first_arg_addr();
        let addr2 = self.second_arg_addr();
        let addr3 = self.third_arg_addr();

        if self.memory[addr1] == self.memory[addr2] {
            self.memory[addr3] = 1;
        } else {
            self.memory[addr3] = 0;
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

        let addr1 = self.first_arg_addr();
        let bval = self.base_reg as i64 + self.memory[addr1];

        if bval < 0 {
            self.base_reg = 0;
        } else {
            self.base_reg = bval as usize;
        }

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
    comp.ext_write(Some(2));

    while !comp.eom {
        comp.tick();
    }
}
