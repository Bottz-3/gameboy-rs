use crate::cpu::{Cpu, Register, registers::registers::Register16};

impl Cpu {
    // incrementing can just be handled at call site. no flag setting.
    pub fn add_16(&mut self, reg: Register16) {
        let hl = self.registers.get_hl();
        let rr = self.get16(reg);
        let val = hl.wrapping_add(rr);

        self.set16(Register16::HL, val);
        // set flags
        //
        // set n to 0
        let mut f = self.registers.get(Register::F);

        f &= !(1 << 6);

        if (hl & 0xFFF) + (rr & 0xFFF) > 0xFFF {
            f |= 1 << 5;
        }
        // c flag
        if (hl as u32) + (rr as u32) > 0xFFFF {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f)
    }

    pub fn add_sp(&mut self, e: i8) {
        let sp = self.sp;
        let val = (e as i16) as u16;
        self.sp = sp.wrapping_add(val);
        // set flags
        //
        // set n to 0
        let mut f: u8 = 0;

        // just need to do h and c

        if (sp & 0xF) + (val & 0xF) > 0xFF {
            f |= 1 << 5;
        }
        // c flag
        if sp as u32 + val as u32 > 0xFF {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f)
    }
}
