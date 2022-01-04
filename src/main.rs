use std::env;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// Hyper-minimal fake CPU
struct CPU {
    program_counter: usize,
    memory: [u8; 0x1000],
    registers: [u8; 16],
    stack: [u16; 16],
    stack_pointer: usize,
}

// What the fake CPU can actually do
impl CPU {

    // this combines two u8's to form a single u16
    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        // bitwise operation shifts op_byte1 left 8 bits
        // to make room for the op_byte2 which is then combined unless
        // op_byte2 contains only zeroes
        // result of the expression below is returned
        op_byte1 << 8 | op_byte2
    }


    fn run(&mut self) {
    loop {
        let opcode = self.read_opcode();
        self.program_counter += 2;


        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;
        
        let nnn = opcode & 0x0FFF;

        match (c, x, y, d) {
            (0, 0, 0, 0)     => { return; },
            (0, 0, 0xE, 0xE) => self.ret(),
            (0x2, _, _, _)   => self.call(nnn),
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _                => todo!("opcode {:04x}", opcode),
            
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }
    
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.program_counter = addr as usize
    }

    // adds two values, handles overflow by setting a boolean
    // if the register size has overflowed
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0
        }
    }
}
fn main() {
    // take the arguments and convert to a string.
    let args: Vec<String> = env::args().collect();
    let args_ = &args[1];
    let args_x: String = args_.to_owned() + " " + &args[2];
   
    println!("string: {:?}", &args_x);
    // convert that string into bytes
    let byte_args = &args_x.into_bytes();
    
    println!("bytes: {:?}", &byte_args);
    print_type_of(&byte_args);
    // need to split this at 0x20 (space)
    // to form 2 args for register values


    // creates our CPU
    let mut cpu = CPU {
        stack: [0; 16],
        stack_pointer: 0,
        program_counter: 0,
        memory: [0; 4096],
        registers: [0; 16],
    };

    cpu.registers[0] = 5;  //TODO: arg1
    cpu.registers[1] = 10; //TODO: arg2
    
    let mem = &mut cpu.memory;

    mem[0x000] = 0x21; mem[0x001] = 0x00;   
    mem[0x002] = 0x21; mem[0x003] = 0x00;   
    mem[0x004] = 0x00; mem[0x005] = 0x00;    
 
    mem[0x100] = 0x80; mem[0x101] = 0x14;  
    mem[0x102] = 0x80; mem[0x103] = 0x14;   
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();
    
    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
