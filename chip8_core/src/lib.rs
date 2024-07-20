use rand::random;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
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


/// Emu is a struct representing an emulator.
///
/// It contains the following fields:
/// - `pc`: Represents the program counter
/// - `ram`: Represents the random-access memory
/// - `screen`: Represents the emulator's screen
/// - `v_reg`: Represents the general purpose registers
/// - `i_reg`: Represents the index register
/// - `sp`: Represents the stack pointer
/// - `stack`: Represents the stack
/// - `keys`: Represents the emulator's key input
/// - `dt`: Represents the delay timer
/// - `st`: Represents the sound timer
pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        // Copy FONTSET to RAM from first to 80
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    fn push(&mut self, val: u16)
    {
        self.stack[self.sp as usize] = val;

        self.sp += 1;
    }

    fn pop(&mut self) -> u16
    {
        self.sp -= 1;

        self.stack[self.sp as usize]
    }

    // return the array of display
    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    // handle key press
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        // set pressed key to true
        self.keys[idx] = pressed;
    }

    // load game code from file into our RAM
    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();

        self.ram[start..end].copy_from_slice(data);
    }

    pub fn reset(&mut self)
    {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    /// Fetches the operation code (op) from the memory.
    ///
    /// # Returns
    ///
    /// Returns a 16-bit unsigned integer representing the operation code.
    ///
    /// # Remarks
    ///
    /// This function fetches the higher byte and lower byte from the memory, and combines them to form the complete instruction.
    /// It then increments the program counter (pc) by 2 to point to the next instruction.
    ///
    /// The fetched operation code (op) is returned.
    fn fetch(&mut self) -> u16
    {
        // Fetch operation code(2 bytes)
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;

        // Add 8 zero bytes after the higher byte, and use OR to combine with the lower byte
        // It will get a complete instruction
        let op = (higher_byte << 8) | lower_byte;

        self.pc += 2;

        op
    }

    /// Executes a single instruction in the game.
    pub fn tick(&mut self)
    {
        // Fetch value from game at the memory address stored in PC, and load into RAM
        let op = self.fetch();

        // Decode instruction
        // Execute
        // Move PC to next instruction
        self.execute(op);
    }

    /// Tick the timers to update their values.
    pub fn tick_timers(&mut self)
    {
        if self.dt > 0 {
            self.dt -= 1
        }

        if self.st > 0 {
            if self.st == 1 {
                // 'BEEP' noise
            }
            self.st -= 1;
        }
    }


    /// Executes an operation based on the given opcode.
    ///
    /// The opcode is a 16-bit unsigned integer. It is split into four parts: digit1, digit2, digit3, and digit4.
    /// Each digit represents a specific part of the operation.
    ///
    /// # Arguments
    ///
    /// * `op` - The opcode to execute.
    fn execute(&mut self, op: u16)
    {
        // Split operation code to four parts
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // LOAD V0 to VX
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            },
            // STORE V0 to VX
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            },
            // BCD(Binary convert to Decimal)
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                // Fetch the hundreds digit by dividing by 100 and tossing the decimal
                let hundreds = (vx / 100.0).floor() as u8;
                // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                // Fetch the one digit by tossing the hundreds and the tens
                let ones = (vx % 10.0).floor() as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },
            // I = FONT
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                // character's size is 5 * RAM address
                self.i_reg = c * 5
            },
            // I += VX
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },
            // ST = VX
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v_reg[x];
            },
            // DT = VX
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
            },
            // WAIT KEY
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;

                // check v registers
                // if any key is pressed and that register will be true
                // the program will be continuously
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                // if not pressed, redo operation code again until pressed
                if !pressed {
                    self.pc -= 2;
                }
            },
            // VX = DT
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
            },
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if !key { 
                    self.pc += 2;
                }
            },
            // SKIP KEY PRESS
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            },
            // DRAW
            (0xD, _, _, _) => {
                // Get the (x, y) coords for our sprite
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                // The lst digital determines how many rows high our sprite is
                let num_rows = digit4;

                // Keep track if any pixels were flipped
                let mut flipped = false;

                // Iterate over each row of our sprite
                for y_line in 0..num_rows {

                    // Determine which memory address our row's data is stored
                    let addr = self.i_reg + y_line;
                    let pixels = self.ram[addr as usize];

                    // Iterate over each column in our row
                    for x_line in 0..8
                    {
                        // User a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0
                        {
                            // Sprites should wrap around screen. so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get our pixel's index for our 1D screen array
                            let idx = x + SCREEN_WIDTH * y;

                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                // Populate CF register
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            },
            // VX = rand() & NN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            },
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            },
            // I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },
            // SKIP VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            },
            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;

                // The variable `msb` stands for "most significant bit", which is the highest bit in a series of numbers in binary notation.
                // In this context, it's the highest bit in the actual byte of the value in `self.v_reg[x]`.
                // The operation `(self.v_reg[x] >> 7)` moves the bits of `self.v_reg[x]` seven places to the right.
                // Essentially, the highest bit (the most significant bit) is moved to the lowest bit (the least significant bit) position.
                // The bitwise AND operation `& 1` then retrieves the value of this least significant bit (which is our `msb`).
                let msb = (self.v_reg[x] >> 7) & 1;

                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },
            // VX = VY - VX
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;

                // The variable `lsb` is short for "Least Significant Bit".
                // In the context of binary numbers, the least significant bit is the bit position in a binary integer giving the
                // units value, that is, determining whether the number is even or odd. It is the bit at the far right of the bit string.
                // Here in the code, the least significant bit of the value in v_reg[x] is being determined using bitwise 'AND' operator with 1.
                // '& 1' helps in identifying whether v_reg[x] is even or odd, this operation will result in 1 if the number (in our case v_reg[x]) is odd.
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb
            },
            // VX -= VY
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            // VX += VY
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vy = if carry { 1 } else { 0 };

                self.v_reg[x] = new_vx;

                // Here, 0xF (15 in decimal) is used as an index to access the 16th element of the `v_reg` array.
                // This is because CHIP-8, the architecture that our emulator is mimicking, uses a total of 16 registers (from V0 to VF).
                // The last register, VF (which corresponds to `v_reg[0xF]` in our case), is used as the carry flag in arithmetic operations.
                // This special register (VF) stores the overflow bit resulting from arithmetic operations, acting as a flag for the next instruction if needed.
                // In this specific instruction `(8, _, _, 4) => {}`, if the addition of Vx and Vy results in an overflow, the VF register is set to 1. Otherwise, it is set to 0.
                self.v_reg[0xF] = new_vy;
            },
            // VX ^= VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            },
            // VX &= VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            },
            // VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            },
            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            },
            // VX += NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },
            // VX = NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            },
            // SKIP VX == VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },
            // SKIP VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            },
            // SKIP VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            },
            // CALL NNN
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            },
            // JMP NNN
            (1, _, _, _) => {
                // move PC to given address
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },
            // CLS
            (0, 0, 0xE, 0) => { self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT] },
            // NOP
            (0, 0, 0, 0) => return,
            (_, _, _, _) => {
                unimplemented!("Unimplemented opcode:{}", op)
            }
        }
    }
}
