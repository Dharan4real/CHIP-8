
use rand;

const RAM_SIZE: usize = 4 * 1024;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const   ADDR_START: u16 = 0x200;
const FONTS: [u8; 0x50] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug, PartialEq, Eq)]
pub struct VirtualMachine {
    memory: [u8; RAM_SIZE],
    
    registers: [u8; 16],
    reg_i: u16,
    reg_pc: u16,
    
    stack: [u16; 12],
    reg_stack_ptr: u8,

    delay_timer: u8,
    sound_timer: u8,

    graphics: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],

    keypad: [bool; 16],
}

impl Default for VirtualMachine {
    fn default() -> Self {
        let mut memory = [0; RAM_SIZE];
            
        memory[..FONTS.len()].copy_from_slice(&FONTS);

        VirtualMachine { 
            memory: [0; RAM_SIZE], 
            registers: [0; 0x10], 
            reg_i: 0,
            reg_pc: ADDR_START, 
            stack: [0; 12], 
            reg_stack_ptr: 0, 
            delay_timer: 0, 
            sound_timer: 0, graphics: [false; SCREEN_WIDTH * SCREEN_HEIGHT], 
            keypad: [false; 16],
        }
    }
}

//public
impl VirtualMachine {
    pub fn reset(&mut self) {
        self.memory = [0; RAM_SIZE];
        self.registers = [0; 0x10];
        self.reg_i = 0;
        self.reg_pc = ADDR_START;
        self.stack = [0; 12];
        self.reg_stack_ptr = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad = [false; 0x10];
        self.memory[..FONTS.len()].copy_from_slice(&FONTS);

    } 

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        let start = ADDR_START as usize;
        let end = (ADDR_START as usize) + rom_data.len();

        self.memory[start..end].copy_from_slice(rom_data);
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[(addr & 0xFFF) as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[(addr & 0xFFF) as usize] = data;
    }

    pub fn push_to_stack(&mut self, address: u16) {
        self.stack[self.reg_stack_ptr as usize] = address;
        self.reg_stack_ptr += 1;
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        self.reg_stack_ptr -= 1;
        self.stack[self.reg_stack_ptr as usize]
    }
    
    pub fn fetch(&mut self) -> u16 {
        let hi = self.read(self.reg_pc) as u16;
        self.reg_pc += 1;
        let lo = self.read(self.reg_pc) as u16;
        self.reg_pc += 1;

        hi << 8 | lo
    } 

    pub fn execute(&mut self) {
        let opcode = self.fetch();

        let dis: (u8, u8, u8, u8) = (
            (opcode & 0xF000) as u8, 
            (opcode & 0x0F00) as u8, 
            (opcode & 0x00F0) as u8, 
            (opcode & 0x000F) as u8
        );

        match dis {
            (0, 0, 0xE, 0xE) => self.inst_00EE(),
            (0, 0, 0xE, 0) => self.inst_00E0(),
            (0, _, _, _) => self.inst_0NNN(),
            (0x1, _, _, _) => self.inst_1NNN(),
            (0x2, _, _, _) => self.inst_2NNN(),
            (0x3, vx, _, _) => self.inst_3XNN(vx),
            (0x4, vx, _, _) => self.inst_4XNN(vx),
            (0x5, vx, vy, 0x0) => self.inst_5XY0(vx, vy),
            (0x6, vx, _, _) => self.inst_6XNN(vx),
            _ => ()
        };
    }
}

//private
impl VirtualMachine {
    fn set_vf(&mut self, v: bool) {
        if v {
            self.registers[0xF] = 0x1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn key_map(&mut self) {

    }
}
#[allow(non_snake_case)]
impl VirtualMachine {
    //execute machine language instruction at addr NNN
    fn inst_0NNN(&mut self) {
        self.push_to_stack(self.reg_pc);
        self.reg_pc = self.fetch() & 0xFFF;
    }

    //clear screen
    fn inst_00E0(&mut self) {
        self.graphics = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    //return from subroutine
    fn inst_00EE(&mut self) {
        self.reg_pc = self.pop_from_stack();
    }

    //jump to addr NNN
    fn inst_1NNN(&mut self) {
        self.reg_pc = self.fetch() & 0xFFF;
    }

    //exec subroutine at addr NNN
    fn inst_2NNN(&mut self) {
        self.push_to_stack(self.reg_pc);
        self.reg_pc = self.fetch() & 0xFFF;
    }

    fn inst_3XNN(&mut self, reg_vx: u8) {
        if self.registers[reg_vx as usize] == (self.fetch() & 0x00FF) as u8 {
            self.reg_pc += 1;
        }
    }

    fn inst_4XNN(&mut self, reg_vx: u8) {
        if self.registers[reg_vx as usize] != (self.fetch() & 0x00FF) as u8 {
            self.reg_pc += 1;
        }
    }
    
    fn inst_5XY0(&mut self, reg_vx: u8, reg_vy: u8) {
        if self.registers[reg_vx as usize] == self.registers[reg_vy as usize] {
            self.reg_pc += 1;
        }
    }

    fn inst_6XNN(&mut self, reg_vx: u8) {
        self.registers[reg_vx as usize] = (self.fetch() & 0x00FF) as u8;
    }

    fn inst_7XNN(&mut self, reg_vx: u8) {
        self.registers[reg_vx as usize] += (self.fetch() & 0x00FF) as u8;
    }

    fn inst_8XY0(&mut self, reg_vx: u8, reg_vy: u8) {
        self.registers[reg_vx as usize] = self.registers[reg_vy as usize];
    }

    fn inst_8XY1(&mut self, reg_vx: u8, reg_vy: u8) {
        self.registers[reg_vx as usize] = self.registers[reg_vx as usize] | self.registers[reg_vy as usize];
    }

    fn inst_8XY2(&mut self, reg_vx: u8, reg_vy: u8) {
        self.registers[reg_vx as usize] = self.registers[reg_vx as usize] & self.registers[reg_vy as usize];
    }

    fn inst_8XY3(&mut self, reg_vx: u8, reg_vy: u8) {
        self.registers[reg_vx as usize] = self.registers[reg_vx as usize] ^ self.registers[reg_vy as usize];
    }

    fn inst_8XY4(&mut self, reg_vx: u8, reg_vy: u8) {
        let temp = self.registers[reg_vx as usize] as u16 + self.registers[reg_vy as usize] as u16;

        self.set_vf(!(temp > 0xFF));

        self.registers[reg_vx as usize] = (temp & 0x00FF) as u8;
    }

    pub fn inst_8XY5(&mut self, reg_vx: u8, reg_vy: u8) {
        let temp = self.registers[reg_vx as usize] as u16 + !(self.registers[reg_vy as usize] as u16) + 1;

        self.set_vf(temp > 0xFF);

        self.registers[reg_vx as usize] = (temp & 0x00FF) as u8;
    }

    fn inst_8XY6(&mut self, reg_vx: u8, reg_vy: u8) {
        let temp = (self.registers[reg_vy as usize] >> 1) as u16;

        self.set_vf((self.registers[reg_vy as usize] & 0x01) != 0);

        self.registers[reg_vx as usize] = temp as u8;
    }

    fn inst_8XY7(&mut self, reg_vx: u8, reg_vy: u8) {
        let temp = self.registers[reg_vy as usize] as u16 + !(self.registers[reg_vx as usize] as u16) + 1;

        self.set_vf(!(temp > 0xFF));

        self.registers[reg_vx as usize] = (temp & 0xFF) as u8;
    }

    fn inst_7XYE(&mut self, reg_vx: u8, reg_vy: u8) {
        let temp = (self.registers[reg_vy as usize] << 1) as u16;

        self.set_vf((self.registers[reg_vy as usize] & 0x80) != 0);

        self.registers[reg_vx as usize] = (temp & 0xFF) as u8;
    }

    fn inst_9XY0(&mut self, reg_vx: u8, reg_vy: u8) {
        if self.registers[reg_vx as usize] != self.registers[reg_vy as usize] {
            self.reg_pc += 1;
        }
    }

    fn inst_ANNN(&mut self) {
        self.reg_i = self.fetch() & 0xFFF;
    }

    fn inst_BNNN(&mut self) {
        self.reg_pc = self.fetch() & 0xFFF + self.registers[0] as u16;
    }

    fn inst_CXNN(&mut self, reg_vx: u8) {
        self.registers[reg_vx as usize] = rand::random::<u8>() & (self.fetch() & 0xFF) as u8;
    }

    fn inst_DXYN(&mut self, reg_vx: u8, reg_vy: u8) {
        // let mut coordinate_x = self.registers[reg_vx as usize] & 63;
        // let mut coordinate_y = self.registers[reg_vy as usize] & 31;

        // let bytes = self.fetch() & 0x0F;
        // let sprite_data = &self.memory[(self.reg_i as usize)..(bytes as usize)];

        // self.registers[0xF] = 0;

        // for pixel in sprite_data {
        //     for i in 0..=7 {
        //         if pixel << i != 0 {
        //             if self.graphics[coordinate_x as usize][coordinate_y as usize] {
        //                 self.registers[0xF] = 1;
        //             }
        //             self.graphics[coordinate_x as usize][coordinate_y as usize] ^= true;
        //         }
        //         coordinate_x += 1;

        //         if coordinate_x == 63 {
        //             continue;
        //         }
        //     }
        //     coordinate_y += 1;

        //     if coordinate_y == 31 {
        //         continue;
        //     }
        // }
    }

    fn inst_EX9E(&mut self, reg_vx: u8) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inst_8xy4_test() {
        let mut vm = VirtualMachine::default();

        vm.registers[0] = 150;
        vm.registers[1] = 155;

        vm.inst_8XY4(0, 1);

        assert_eq!((vm.registers[0], vm.registers[0xF]), (49, 1));
    }

    #[test]
    fn inst_8xy5_test() {
        let mut vm = VirtualMachine::default();

        vm.registers[0] = 10;
        vm.registers[1] = 15;

        vm.inst_8XY5(0, 1);

        assert_eq!((vm.registers[0], vm.registers[0xF]), (251, 1));
    }

    #[test]
    fn inst_8xy6_test() {
        let mut vm = VirtualMachine::default();

        vm.registers[1] = 15;

        vm.inst_8XY6(0, 1);

        assert_eq!((vm.registers[0], vm.registers[0xF]), (7, 1));
    }
}