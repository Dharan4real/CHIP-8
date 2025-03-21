
struct VirtualMachine {
    memory: [u8; 4 * 1024],
    
    registers: [u8; 16],
    reg_i: u16,
    reg_pc: u16,
    
    stack: [u16; 12],
    reg_stack_ptr: u8,

    delay_timer: u8,
    sound_timer: u8,

    graphics: [bool; 64 * 32],

    keypad: [bool; 16],

    fetched: u16,
    addr_main: u16,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        let mut memory = [0; 4 * 1024];
        let fonts: [u8; 80] = [
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
            
        memory[..fonts.len()].copy_from_slice(&fonts);

        VirtualMachine { 
            memory, 
            registers: [0; 16], 
            reg_i: 0,
            reg_pc: 0, 
            stack: [0; 12], 
            reg_stack_ptr: 0, 
            delay_timer: 0, 
            sound_timer: 0, graphics: [false; 64 * 32], 
            keypad: [false; 16], 
            fetched: 0, 
            addr_main: 0 
        }
    }
}

//public
impl VirtualMachine {
    pub fn read(&self, addr: u16) -> u8 {
        self.memory[(addr & 0xFFF) as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[(addr & 0xFFF) as usize] = data;
    }
    
    pub fn fetch(&mut self) {
        let hi = self.read(self.reg_pc) as u16;
        self.reg_pc += 1;
        let lo = self.read(self.reg_pc) as u16;
        self.reg_pc += 1;

        self.fetched = hi << 8 | lo;
    } 

    pub fn operate(&mut self) {
        self.fetch();

        let dis: (u8, u8, u8, u8) = ((self.fetched & 0xF000) as u8, (self.fetched & 0x0F00) as u8, (self.fetched & 0x00F0) as u8, (self.fetched & 0x000F) as u8);
        let registers = &self.registers;

        match dis {
            (0, 0, 0xE, 0xE) => self.inst_00EE(),
            (0, 0, 0xE, 0) => self.inst_00EE(),
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
}
#[allow(non_snake_case)]
impl VirtualMachine {
    //execute machine language instruction at addr NNN
    fn inst_0NNN(&mut self) {
        self.addr_main = self.reg_pc;
        self.reg_pc = self.fetched & 0xFFF;
    }

    //clear screen
    fn inst_00E0(&mut self) {
        self.graphics = [false; 64 * 32];
    }

    //return from subroutine
    fn inst_00EE(&mut self) {
        self.reg_pc = self.addr_main;
    }

    //jump to addr NNN
    fn inst_1NNN(&mut self) {
        self.reg_pc = self.fetched & 0xFFF;
    }

    //exec subroutine at addr NNN
    fn inst_2NNN(&mut self) {
        self.addr_main = self.reg_pc;
        self.reg_pc = self.fetched & 0xFFF;
    }

    fn inst_3XNN(&mut self, reg_vx: u8) {
        if self.registers[reg_vx as usize] == (self.fetched & 0x00FF) as u8 {
            self.reg_pc += 1;
        }
    }

    fn inst_4XNN(&mut self, reg_vx: u8) {
        if self.registers[reg_vx as usize] != (self.fetched & 0x00FF) as u8 {
            self.reg_pc += 1;
        }
    }
    
    fn inst_5XY0(&mut self, reg_vx: u8, reg_vy: u8) {
        if self.registers[reg_vx as usize] == self.registers[reg_vy as usize] {
            self.reg_pc += 1;
        }
    }

    fn inst_6XNN(&mut self, reg_vx: u8) {
        self.registers[reg_vx as usize] = (self.fetched & 0x00FF) as u8;
    }

    fn inst_7XNN(&mut self, reg_vx: u8) {
        self.registers[reg_vx as usize] += (self.fetched & 0x00FF) as u8;
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

        self.set_vf(temp > 0xFF);

        self.registers[reg_vx as usize] = (temp & 0x00FF) as u8;
    }

    pub fn inst_8XY5(&mut self, reg_vx: u8, reg_vy: u8) {
        let temp = self.registers[reg_vx as usize] as u16 + !(self.registers[reg_vy as usize] as u16) + 1;

        self.set_vf(temp > 0xFF);

        self.registers[reg_vx as usize] = (temp & 0x00FF) as u8;
    }

    fn inst_8XY6(&mut self, reg_vx: u8, reg_vy: u8) {
        let temp = self.registers[reg_vy as usize] >> 1;

        self.set_vf((self.registers[reg_vy as usize] & 0x01) != 0);

        self.registers[reg_vx as usize] = temp;
    }
}

fn main() {
    
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