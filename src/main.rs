/* Testing
*  this is as test
*/

fn main() {
    let val: u32 = 30;
    let opcode: u32 = val >> 1 & 0xF;
    print!("Val: {} => {:032b}", val, val);
    print!("Opcode: {} => {:032b}", opcode, opcode);
}

pub struct LVDC {
    pub ic: u32,           // Instruction Coutner register
    pub acc: u32,          // Accumulator register
    pub pq: u32,           // Product-Quotient register
    pub mem: [u32; 0xFFF], // Memory module
}

impl LVDC {
    pub fn new() -> Self {
        LVDC {
            acc: 0,
            ic: 0,
            pq: 0,
            mem: [0; 0xFFF],
        }
    }

    fn hop(&mut self) {
        todo!()
    }

    fn mpy(&mut self, operand_address: u8) {
        self.pq = self.acc * self.mem[operand_address as usize];
    }

    fn sub(&mut self, operand_address: u8) {
        self.acc -= self.mem[operand_address as usize];
    }

    fn div(&mut self, operand_address: u8) {
        self.pq = self.acc / self.mem[operand_address as usize];
    }

    fn tnz(&mut self) {
        todo!("")
    }

    fn mph(&mut self, operand_address: u8) {
        self.acc *= self.mem[operand_address as usize];
    }

    fn and(&mut self, operand_address: u8) {
        self.acc = self.acc & self.mem[operand_address as usize];
    }

    fn add(&mut self, operand_address: u8) {
        self.acc += self.mem[operand_address as usize];
    }

    fn tra(&mut self, operand_address: u8) {
        todo!();
    }

    fn xor(&mut self, operand_address: u8) {
        self.acc = self.acc ^ self.mem[operand_address as usize];
    }

    fn pio(&mut self, operand_address: u8) {
        todo!()
    }

    fn sto(&mut self, operand_address: u8) {
        self.mem[operand_address as usize] = self.acc;
    }

    fn tmi(&mut self) {
        todo!()
    }

    fn shf(&mut self, operand_address: u8) {
        todo!()
    }

    fn cla(&mut self, operand_address: u8) {
        self.acc = self.mem[operand_address as usize];
    }

    pub fn interpret(&mut self, program: Vec<u32>) {
        self.ic = 0;

        let instruction = program[self.ic as usize];

        let opcode = (instruction >> 1 & 0xF) as u8;
        self.ic += 1;

        match opcode {
            0x00 => self.hop(),

            0x01 => {
                let operand_address = (instruction >> 6) as u8;
                self.mpy(operand_address);
            }

            0x02 => {
                let operand_address = (instruction >> 6) as u8;
                self.sub(operand_address);
            }

            0x03 => {
                let operand_address = (instruction >> 6) as u8;
                self.div(operand_address);
            }

            0x04 => {
                self.tnz();
            }

            0x05 => {
                let operand_address = (instruction >> 6) as u8;
                self.mph(operand_address);
            }

            0x06 => {
                let operand_address = (instruction >> 6) as u8;
                self.and(operand_address);
            }

            0x07 => {
                let operand_address = (instruction >> 6) as u8;
                self.add(operand_address);
            }

            0x08 => {
                let operand_address = (instruction >> 6) as u8;
                self.tra(operand_address);
            }

            0x09 => {
                let operand_address = (instruction >> 6) as u8;
                self.xor(operand_address);
            }

            0x0A => {
                let operand_address = (instruction >> 6) as u8;

                self.pio(operand_address);
            }

            0x0B => {
                let operand_address = (instruction >> 6) as u8;
                self.sto(operand_address);
            }

            0x0C => {
                self.tmi();
            }

            0x0E => {
                let operand_address = (instruction >> 6) as u8;
                self.shf(operand_address);
            }

            0x0F => {
                let operand_address = (instruction >> 6) as u8;

                self.cla(operand_address);
            }

            _ => todo!(),
        }
    }
}
