pub mod instructions;
pub mod registers;
pub mod flags;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::u16;

use crate::i8080::instructions::*;
use crate::i8080::flags::*;
use crate::i8080::registers::*;
use crate::utils::*;

const MEMORY_SIZE: usize = 0xFFFFF;

/// The main struct that will hold the state of the processor attributes
#[derive(Clone, Debug)]
pub struct Processor 
{
    clock_frequency   : u32,
    stack_pointer     : u16,
    program_counter   : u16,
    memory            : [u8; MEMORY_SIZE],
    registers         : Registers,
    flags             : StatusFlags,
    current_op        : Instruction,
    out               : u8, 
    interrupts_enabled: bool,
}

impl Processor 
{
    pub fn from_file(p: String, hz: u32) -> Processor 
    {
        let mut proc = Processor 
        {
            stack_pointer     : 0x20,
            clock_frequency   : hz,
            program_counter   : 0,
            memory            : [0; MEMORY_SIZE],
            flags             : StatusFlags::new(),
            current_op        : Instruction::new(),
            registers         : Registers  ::new(),
            out               : 0,
            interrupts_enabled: false,
        };

        let mut file = File::open(p).expect("No such file");
        file.read(&mut proc.memory).expect("opsie");
        proc
    }

    pub fn reset(&mut self) 
    {
        self.stack_pointer      = 0x20;
        self.program_counter    = 0;
        self.flags              = StatusFlags::new();
        self.current_op         = Instruction::new();
        self.registers          = Registers::new();
        self.out                = 0;
        self.interrupts_enabled = false;
    }

    pub fn from_bytes(bytes: Vec<u8>, hz: u32) -> Processor 
    {
        let mut processor = Processor 
        {
            stack_pointer     : 0x20,
            clock_frequency   : hz,
            program_counter   : 0,
            memory            : [0; MEMORY_SIZE],
            flags             : StatusFlags::new(),
            current_op        : Instruction::new(),
            registers         : Registers  ::new(),
            out               : 0,
            interrupts_enabled: false,
        };

        for (i, byte) in bytes.iter().enumerate()
        {
            processor.memory[i] = *byte;
        }
        return processor;
    }

    pub fn clock(&mut self) 
    {
        self.fetch_instruction();
        self.execute_instruction();
        self.update_program_counter();
        if self.clock_frequency > 0
        {
            std::thread::sleep(std::time::Duration::from_secs(1) / self.clock_frequency);
        }
    }

    pub fn get_instruction_length(&self) -> u8
    {
        return self.current_op.get_length();
    }

    pub fn reset_pc(&mut self)
    {
        self.program_counter = 0;
    }

    pub fn fetch_instruction(&mut self)  
    {
        let op = self.memory[self.program_counter as usize];
        let immediate_lsb = self.memory[(self.program_counter + 1) as usize];
        let immediate_msb = self.memory[(self.program_counter + 2) as usize];
        self.current_op.byte_to_op(op, immediate_lsb, immediate_msb);
    }

    fn execute_instruction(&mut self)
    {
        match self.current_op.instruction_type
        {
            InstructionTypes::MOV  => self.mov_op(),
            InstructionTypes::ADD  => self.add_op(false),
            InstructionTypes::ADC  => self.add_op(true),
            InstructionTypes::SUB  => self.sub_op(false),
            InstructionTypes::SBB  => self.sub_op(true),
            InstructionTypes::MVI  => self.mvi_op(),
            InstructionTypes::ANA  => self.ana_op(),
            InstructionTypes::ORA  => self.ora_op(),
            InstructionTypes::XRA  => self.xra_op(),
            InstructionTypes::CMP  => self.cmp_op(),
            InstructionTypes::ADI  => self.add_op(false),
            InstructionTypes::ACI  => self.add_op(true),
            InstructionTypes::SUI  => self.sub_op(false),
            InstructionTypes::SBI  => self.sub_op(true),
            InstructionTypes::ANI  => self.ana_op(),
            InstructionTypes::XRI  => self.xra_op(),
            InstructionTypes::ORI  => self.ora_op(),
            InstructionTypes::CPI  => self.cmp_op(),
            InstructionTypes::JMP  => self.jmp_op(),
            InstructionTypes::JC   => self.jc_op(),
            InstructionTypes::JNC  => self.jnc_op(),
            InstructionTypes::JZ   => self.jz_op(),
            InstructionTypes::JNZ  => self.jnz_op(),
            InstructionTypes::JPE  => self.jpe_op(),
            InstructionTypes::JPO  => self.jpo_op(),
            InstructionTypes::JP   => self.jp_op(),
            InstructionTypes::JM   => self.jm_op(),
            InstructionTypes::LXI  => self.lxi_op(),
            InstructionTypes::DCR  => self.dcr_op(),
            InstructionTypes::DAD  => self.dad_op(),
            InstructionTypes::RRC  => self.rrc_op(),
            InstructionTypes::RLC  => self.rlc_op(),
            InstructionTypes::RAL  => self.ral_op(),
            InstructionTypes::RAR  => self.rar_op(),
            InstructionTypes::INX  => self.inx_op(),
            InstructionTypes::LDA  => self.lda_op(),
            InstructionTypes::LDAX => self.ldax_op(),
            InstructionTypes::STA  => self.sta_op(),
            InstructionTypes::PUSH => self.push_op(),
            InstructionTypes::POP  => self.pop_op(),
            InstructionTypes::CALL => self.call_op(),
            InstructionTypes::RET  => self.ret_op(),
            InstructionTypes::XCHG => self.xchg_op(),
            InstructionTypes::OUT  => self.out_op(),
            InstructionTypes::EI   => self.ei_op(),
            InstructionTypes::DI   => self.di_op(),
            InstructionTypes::INR  => self.inr_op(),
            InstructionTypes::CP   => self.cp_op(),
            InstructionTypes::CNZ  => self.cnz_op(),
            InstructionTypes::CC   => self.cc_op(),
            InstructionTypes::CNC  => self.cnc_op(),
            InstructionTypes::CPO  => self.cpo_op(),
            InstructionTypes::CPE  => self.cpe_op(),
            InstructionTypes::CM   => self.cm_op(),
            InstructionTypes::CZ   => self.cz_op(),
            InstructionTypes::RC   => self.rc_op(),
            InstructionTypes::RNC  => self.rnc_op(),
            InstructionTypes::RZ   => self.rz_op(),
            InstructionTypes::RNZ  => self.rnz_op(),
            InstructionTypes::RM   => self.rm_op(),
            InstructionTypes::RP   => self.rp_op(),
            InstructionTypes::RPE  => self.rpe_op(),
            InstructionTypes::RPO  => self.rpo_op(),
            InstructionTypes::DCX  => self.dcx_op(),
            InstructionTypes::LHLD => self.lhld_op(),
            InstructionTypes::SHLD => self.shld_op(),
            InstructionTypes::STAX => self.stax_op(),
            InstructionTypes::STC  => self.stc_op(),
            InstructionTypes::CMC  => self.cmc_op(),
            InstructionTypes::CMA  => self.cma_op(),
            InstructionTypes::DAA  => self.daa_op(),
            InstructionTypes::SPHL => self.sphl_op(),
            InstructionTypes::XTHL => self.xthl_op(),
            InstructionTypes::PCHL => self.pchl_op(),
            InstructionTypes::RST  => self.rst_op(),
            InstructionTypes::NOP  => (),
            InstructionTypes::Unknown => panic!("Uknown Instruction {:?}", self),
        }
    }

    fn update_program_counter(&mut self)
    {
        if self.program_counter == u16::MAX
        {
            println!("Reached end of program memory, shutting down.");
            exit(0);
        }
        self.program_counter += 1;
    }

    pub fn get_flags(&self) -> StatusFlags
    {
        return self.flags
    }

    pub fn get_registers(&self) -> Registers
    {
        return self.registers
    }

    pub fn get_pc(&self) -> u16 
    {
        return self.program_counter;
    }

    pub fn get_current_op(&self) -> Instruction
    {
        return self.current_op.clone();
    }

    pub fn get_stack_pointer(&self) -> u16
    {
        return self.stack_pointer;
    }

    pub fn get_instructions(&mut self) -> Vec<String>
    {
        let mut instructions: Vec<String> = Vec::new();
        instructions.push("".to_string());
        for x in 0..48
        {
            let instruction = Instruction::from_byte(self.memory[self.program_counter as usize + x]);
            let (bin, stri) = instruction.get_name_byte();
            instructions.push(String::from(format!("{a:>6}:     0x{b:02X} {c:}", 
                                                   a=(self.program_counter as usize + x), b=bin, c=stri)));
        }
        return instructions
    }

    pub fn set_all_registers(&mut self, reg: Registers)
    {
        self.registers = reg;
    }

    fn set_reg(&mut self, register: u8, value: u8)
    {
        match register & 0b111
        {
            B_REG   => self.registers.b = value,
            C_REG   => self.registers.c = value,
            D_REG   => self.registers.d = value,
            E_REG   => self.registers.e = value,
            H_REG   => self.registers.h = value,
            L_REG   => self.registers.l = value,
            MEM_REF => 
            {
                let addr = (self.registers.h as usize) << 8 | (self.registers.l as usize);
                self.memory[addr] = value;
            },
            A_REG   => self.registers.accumulator = value,
            _ => panic!("No register {}", register)
        }
    }

    fn get_reg(&self, reg: u8) -> u8
    {
        match reg & 0b111{
            B_REG   => self.registers.b,
            C_REG   => self.registers.c,
            D_REG   => self.registers.d,
            E_REG   => self.registers.e,
            H_REG   => self.registers.h,
            L_REG   => self.registers.l,
            MEM_REF => 
            {
                let addr = (self.registers.h as usize) << 8 | (self.registers.l as usize);
                return self.memory[addr] 
            },
            A_REG   => self.registers.accumulator,
            _ => panic!("No register {}", reg)
        }
    }

    fn get_reg_pair(&mut self, reg: u8) -> (u8, u8)
    {
        match reg
        {
            BC_PAIR_REG => (self.registers.b, self.registers.c),
            DE_PAIR_REG => (self.registers.d, self.registers.e),
            HL_PAIR_REG => (self.registers.h, self.registers.l),
            SP_REG => 
            {
                // In the case of POP/PUSH, the matched REG_PAIR for 0b11 is PSW (flags and accumulator)
                if self.current_op.instruction_type == InstructionTypes::POP  
                    || self.current_op.instruction_type == InstructionTypes::PUSH
                {
                    (self.flags.get_flags_u8(), self.registers.accumulator)
                }
                else
                {
                    ((self.stack_pointer >> 8) as u8, (self.stack_pointer & 0xFF) as u8)
                }
            }
            _ => panic!("No register pair {}, PC at {}", reg, self.program_counter)
        }
    }

    fn set_reg_pair(&mut self, reg: u8, msb: u8, lsb: u8)
    {
        match reg
        {
            BC_PAIR_REG => 
            {
                self.registers.b = msb;
                self.registers.c = lsb;
            }
            DE_PAIR_REG => 
            {
                self.registers.d = msb;
                self.registers.e = lsb;
            }
            HL_PAIR_REG => 
            {
                self.registers.h = msb;
                self.registers.l = lsb;
            }
            SP_REG =>
            {
                if self.current_op.instruction_type == InstructionTypes::POP  
                    || self.current_op.instruction_type == InstructionTypes::PUSH
                {
                    self.flags.set_flags_u8(msb);
                    self.registers.accumulator = lsb;
                }
                else
                {
                    self.stack_pointer = bytes_to_word(msb, lsb);
                }
            }
            _ => panic!("No register pair {}, PC at {}", reg, self.program_counter)
        }
    }

    pub fn get_memory(&self) -> [u8; MEMORY_SIZE]
    {
        return self.memory;
    }

    pub fn get_memory_at(&self, addr: u16) -> u8
    {
        return self.memory[addr as usize];
    }

    pub fn set_memory_at(&mut self, addr: u16, val: u8)
    {
        self.memory[addr as usize] = val;
    }

    /// The move (MOV) instruction copies the value of the second register into the first register.
    fn mov_op(&mut self)
    {
        let to = (self.current_op.machine_code & MOVE_TO_MASK) >> MOVE_TO_BIT_POS;
        let from = (self.current_op.machine_code & MOVE_FROM_MASK) >> MOVE_FROM_BIT_POS;
        let val = self.get_reg(from);
        self.set_reg(to, val);
    }

    /// The addition (ADD) instruction adds the value of the second register to the first register.
    fn add_op(&mut self, with_carry: bool)
    {
        let accumulator = self.get_reg(A_REG);

        let register = match self.current_op.instruction_type 
        {
            InstructionTypes::ADD | InstructionTypes::ADC => 
            {
                self.get_reg(self.current_op.low_nibble.unwrap())
            }
            InstructionTypes::ADI | InstructionTypes::ACI => 
            {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            },
            _ => {panic!("Add type is wrong, {:?}", self);}
        };

        let (res, carry ) = 
            if with_carry 
            {
                let c = if self.flags.carry_flag {1} else {0};
                let (r, ca) = accumulator.overflowing_add(c);
                let (r, c) = register.overflowing_add(r);
                (r, c | ca)
            }
            else
            {
                register.overflowing_add(accumulator)
            };

        let aux_flag = (accumulator & 0x0F) + (register & 0x0F) > 0x0F;
        self.set_flags_cszp(carry, aux_flag, res);
        self.set_reg(A_REG, res);
    }

    /// The subtraction (SUB) instruction subtracts the value of the second register from the 
    /// first register.
    fn sub_op(&mut self, with_carry: bool)
    {
        let accumulator = self.get_reg(A_REG);

        let register = match self.current_op.instruction_type 
        {
            InstructionTypes::SUB | InstructionTypes::SBB => 
            {
                self.get_reg(self.current_op.low_nibble.unwrap())
            }
            InstructionTypes::SUI | InstructionTypes::SBI => 
            {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            },
            _ => {panic!("Add type is wrong, this panic should be impossible");}
        };

        let (res, carry ) = if with_carry 
        {
            let c = if self.flags.carry_flag {1} else {0};
            accumulator.overflowing_sub(register + c)
        }
        else
        {
            accumulator.overflowing_sub(register)
        };

        let aux_flag = (accumulator & 0x0F) < (register & 0x0F);
        self.set_flags_cszp(carry, aux_flag, res);
        self.set_reg(A_REG, res);
    }

    /// The logical AND (ANA) instruction performs a bitwise AND operation between the accumulator 
    /// and the second register.
    fn ana_op(&mut self)
    {
        let accumulator = self.get_reg(A_REG);
        let register = match self.current_op.instruction_type 
        {
            InstructionTypes::ANA => 
            {
                self.get_reg(self.current_op.low_nibble.unwrap())
            }
            InstructionTypes::ANI => 
            {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            }
            _ => panic!("Should be an impossible match"),
        };
        let res = accumulator & register;
        self.set_flags_cszp(false, false, res);
        self.set_reg(A_REG, res);
    }

    /// The logical OR (ORA) instruction performs a bitwise OR operation between the accumulator and
    /// the second register.
    fn ora_op(&mut self)
    {
        let accumulator = self.get_reg(A_REG);
        let register = match self.current_op.instruction_type 
        {
            InstructionTypes::ORA => 
            {
                self.get_reg(self.current_op.low_nibble.unwrap())
            }
            InstructionTypes::ORI => 
            {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            }
            _ => panic!("Should be an impossible match"),
        };

        let res = accumulator | register;
        self.set_flags_cszp(false, false, res);
        self.set_reg(A_REG, res);

    }

    /// The logical exclusive OR (XRA) instruction performs a bitwise XOR operation between the 
    /// accumulator and the second register.
    fn xra_op(&mut self)
    {
        let accumulator = self.get_reg(A_REG);
        let register = match self.current_op.instruction_type 
        {
            InstructionTypes::XRA => 
            {
                self.get_reg(self.current_op.low_nibble.unwrap())
            }
            InstructionTypes::XRI => 
            {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            }
            _ => panic!("Should be an impossible match"),
        };

        let res = accumulator ^ register;
        self.set_flags_cszp(false, false, res);
        self.set_reg(A_REG, res);
    }

    /// The compare (CMP) instruction subtracts the value of the second register from the first 
    /// register and sets respective flag based on the result, but the result is not stored.
    fn cmp_op(&mut self)
    {
        let accumulator = self.get_reg(A_REG);
        let register = match self.current_op.instruction_type 
        {
            InstructionTypes::CMP => 
            {
                self.get_reg(self.current_op.low_nibble.unwrap())
            }
            InstructionTypes::CPI => 
            {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            }
            _ => panic!("Should be an impossible match"),
        };
        let (res, carry) = accumulator.overflowing_sub(register);
        let aux_flag = (accumulator & 0x0F) < (register & 0x0F);
        self.set_flags_cszp(carry, aux_flag, res);
    }

    /// Move immediate (MVI) instruction copies the immediate next value in the memory into the 
    /// specified register.
    fn mvi_op(&mut self)
    {
        self.program_counter += 1;
        let result = self.memory[self.program_counter as usize];
        self.set_reg(self.current_op.low_nibble.unwrap(), result);
    }

    /// Jump (JMP) instruction sets the program counter to the address specified in the next two
    /// bytes in the memory.
    fn jmp_op(&mut self)
    {
        let mut addr = self.get_direct_address();
        if addr > 0
        {
            addr = addr - 1;
        }
        self.program_counter = addr; 
    }
    
    /// Jump not zero (JNZ) instruction sets the program counter to the address specified in the next 
    /// two bytes in the memory if the zero flag is not set.
    fn jnz_op(&mut self)
    {
        if !self.flags.zero_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Jump zero (JZ) instruction sets the program counter to the address specified in the next
    /// two bytes in the memory if the zero flag is set.
    fn jz_op(&mut self)
    {
        if self.flags.zero_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Jump not carry (JNC) instruction sets the program counter to the address specified in the next
    /// two bytes in the memory if the carry flag is not set.
    fn jnc_op(&mut self)
    {
        if !self.flags.carry_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Jump carry (JC) instruction sets the program counter to the address specified in the next
    /// two bytes in the memory if the carry flag is set.
    fn jc_op(&mut self)
    {
        if self.flags.carry_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Jump parity odd (JPO) instruction sets the program counter to the address specified in the next
    /// two bytes in the memory if the parity flag is not set.
    fn jpo_op(&mut self)
    {
        if !self.flags.parity_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Jump parity even (JPE) instruction sets the program counter to the address specified in the next
    /// two bytes in the memory if the parity flag is set.
    fn jpe_op(&mut self)
    {
        if self.flags.parity_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Jump positive (JP) instruction sets the program counter to the address specified in the next
    /// two bytes in the memory if the sign flag is not set.
    fn jp_op(&mut self)
    {
        if !self.flags.sign_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Jump negative (JM) instruction sets the program counter to the address specified in the next
    /// two bytes in the memory if the sign flag is set.
    fn jm_op(&mut self)
    {
        if self.flags.sign_flag
        {
            self.jmp_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// Load immediate (LXI) instruction loads the immediate next two bytes in the memory into the
    /// specified register pair.
    fn lxi_op(&mut self)
    {
        let pc        = self.program_counter as usize;
        let lsb_value = self.memory[pc + 1];
        let msb_value = self.memory[pc + 2];
        let reg_pair  = self.current_op.low_nibble.unwrap();
        self.set_reg_pair(reg_pair, msb_value, lsb_value);
        self.program_counter += 2;
    }

    /// double add (DAD) instruction adds the value of the specified register pair to the HL register pair.
    fn dad_op(&mut self)
    {
        let reg_pair = self.current_op.low_nibble.unwrap();

        let (msb, lsb)   = self.get_reg_pair(reg_pair);
        let num1: u16    = bytes_to_word(msb, lsb);
        let (msb, lsb)   = self.get_reg_pair(HL_PAIR_REG);
        let num2: u16    = bytes_to_word(msb, lsb);
        let (res, carry) = num1.overflowing_add(num2);

        self.set_reg_pair(HL_PAIR_REG, (res >> 8) as u8, res as u8);
        self.flags.carry_flag = carry;
    }

    /// The rotate right through carry (RRC) instruction rotates the accumulator right through the carry flag.
    /// The carry flag is set to the value of the least significant bit of the accumulator.
    fn rrc_op(&mut self)
    {
        let mut accumulator   = self.get_reg(A_REG);
        accumulator           = accumulator.rotate_right(1);
        self.flags.carry_flag = (accumulator & 0x80) == 0x80;
        self.set_reg(A_REG, accumulator);
    }

    /// The rotate left through carry (RLC) instruction rotates the accumulator left through the carry flag.
    /// The carry flag is set to the value of the most significant bit of the accumulator.
    fn rlc_op(&mut self)
    {
        let mut accumulator   = self.get_reg(A_REG);
        accumulator           = accumulator.rotate_left(1);
        self.flags.carry_flag = (accumulator & 0x01) == 0x01;
        self.set_reg(A_REG, accumulator);
    }

    /// The decrement (DCR) instruction decrements the value of the specified register by 1.
    /// The auxiliary flag is set if the lower nibble of the register is 0.
    fn dcr_op(&mut self)
    {
        let reg                   = self.current_op.low_nibble.unwrap();
        let reg_value             = self.get_reg(reg);
        let (res, _)              = reg_value.overflowing_sub(1);
        self.flags.auxiliary_flag = reg_value & 0x0F == 0x00;
        self.flags.parity_flag    = parity(res);
        self.flags.sign_flag      = sign(res);
        self.flags.zero_flag      = zero(res);
        self.set_reg(reg, res);
    }

    /// The rotate left (RAL) instruction rotates the accumulator left through the carry flag.
    /// The carry flag is set to the value of the most significant bit of the accumulator.
    fn ral_op(&mut self)
    {
        let mut accumulator   = self.get_reg(A_REG);
        let carry             = self.flags.carry_flag;
        self.flags.carry_flag = (accumulator & 0x80) == 0x80;
        accumulator           = accumulator << 1;
        accumulator           = accumulator | (carry as u8);
        self.set_reg(A_REG, accumulator);
    }

    /// The rotate right (RAR) instruction rotates the accumulator right through the carry flag.
    /// The carry flag is set to the value of the least significant bit of the accumulator.
    fn rar_op(&mut self)
    {
        let accumulator       = self.get_reg(A_REG);
        let carry             = self.flags.carry_flag;
        self.flags.carry_flag = (accumulator & 0x01) == 0x01;
        let mut res           = accumulator >> 1;
        res                   = res | ((carry as u8) << 7);
        self.set_reg(A_REG, res);
    }

    /// The decrement (DCX) instruction decrements the value of the specified register pair by 1.
    fn dcx_op(&mut self)
    {
        let reg_pair   = self.current_op.low_nibble.unwrap();
        let (msb, lsb) = self.get_reg_pair(reg_pair);
        let num: u16   = ((msb as u16) << 8) + lsb as u16;
        let (res, _)   = num.overflowing_sub(1);
        self.set_reg_pair(reg_pair, (res >> 8) as u8, res as u8);
    }

    /// The increment (INX) instruction increments the value of the specified register pair by 1.
    fn inx_op(&mut self)
    {
        let reg_pair   = self.current_op.low_nibble.unwrap();
        let (msb, lsb) = self.get_reg_pair(reg_pair);
        let num: u16   = ((msb as u16) << 8) + lsb as u16;
        let (res, _)   = num.overflowing_add(1);
        self.set_reg_pair(reg_pair, (res >> 8) as u8, res as u8);
    }

    /// load accumulator (LDA) instruction loads the value at the specified address in the memory into the accumulator.
    /// The address is specified in the next two bytes in the memory.
    fn lda_op(&mut self)
    {
        let addr = self.get_direct_address();
        self.program_counter += 2;
        let value = self.memory[addr as usize]; 
        self.set_reg(A_REG, value);
    }

    /// load accumulator indirect (LDAX) instruction loads the value at the address specified by the BC or DE register pair
    /// into the accumulator.
    fn ldax_op(&mut self)
    {
        let lsb;
        let msb;

        if (self.current_op.machine_code & 0x10) == 0x10
        {
            (msb, lsb) = self.get_reg_pair(DE_PAIR_REG);
        }
        else
        {
            (msb, lsb) = self.get_reg_pair(BC_PAIR_REG);
        }
        let address = (msb as u16) << 8 | lsb as u16;
        self.set_reg(A_REG, self.memory[address as usize]);
    }

    /// store accumulator (STA) instruction stores the value of the accumulator into the address 
    /// specified in the next two bytes in the memory.
    fn sta_op(&mut self)
    {
        let addr = self.get_direct_address();
        self.program_counter += 2;
        let value = self.get_reg(A_REG);
        self.memory[addr as usize] = value;
    }

    /// push (PUSH) instruction pushes the value of the specified register pair onto the stack.
    fn push_op(&mut self)
    {
        let (msb, lsb) = self.get_reg_pair(self.current_op.low_nibble.unwrap());
        self.memory[(self.stack_pointer - 1) as usize] = msb;
        self.memory[(self.stack_pointer - 2) as usize] = lsb;
        self.stack_pointer -= 2;
    }

    /// pop (POP) instruction pops the value of the specified register pair from the stack.
    fn pop_op(&mut self)
    {
        let lsb = self.memory[(self.stack_pointer + 0) as usize];
        let msb = self.memory[(self.stack_pointer + 1) as usize];
        self.stack_pointer += 2;
        self.set_reg_pair(self.current_op.low_nibble.unwrap(), msb, lsb);
    }

    /// call (CALL) instruction pushes the address of the next instruction onto the stack and sets the program counter
    /// to the address specified in the next two bytes in the memory.
    fn call_op(&mut self)
    {
        let addr: u16         = self.get_direct_address();
        let next_addr: u16    = self.program_counter + 3;
        let lsb_next_addr: u8 = (next_addr & 0x00FF) as u8;
        let msb_next_addr: u8 = ((next_addr & 0xFF00) >> 8) as u8;

        self.memory[( self.stack_pointer - 1 ) as usize] = msb_next_addr;
        self.memory[( self.stack_pointer - 2 ) as usize] = lsb_next_addr;

        self.stack_pointer = self.stack_pointer - 2;
        self.program_counter = addr - 1;
    }

    /// call non zero (CNZ) instruction calls the address specified in the next two bytes in the 
    /// memory if the zero flag is not set.
    fn cz_op(&mut self)
    {
        if self.flags.zero_flag
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// call minus (CM) instruction calls the address specified in the next two bytes in the
    /// memory if the sign flag is set.
    fn cm_op(&mut self)
    {
        if self.flags.sign_flag
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// call plus (CP) instruction calls the address specified in the next two bytes in the
    /// memory if the sign flag is not set.
    fn cp_op(&mut self)
    {
        if self.flags.sign_flag == false
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// return (RET) instruction pops the address from the stack and sets the program counter to that address.
    fn ret_op(&mut self)
    {
        let lsb_addr = self.memory[self.stack_pointer as usize];
        let msb_addr = self.memory[(self.stack_pointer + 1) as usize];
        let addr: u16 = bytes_to_word(msb_addr, lsb_addr);

        self.stack_pointer += 2;
        self.program_counter = addr - 1;
    }


    /// The exchange (XCHG) instruction exchanges the values of the DE and HL register pairs.
    fn xchg_op(&mut self)
    {
        let regs = self.get_registers();
        self.set_reg(D_REG, regs.h);
        self.set_reg(H_REG, regs.d);
        self.set_reg(E_REG, regs.l);
        self.set_reg(L_REG, regs.e);
    }

    /// Output (OUT) instruction outputs the value of the accumulator to out.
    fn out_op(&mut self)
    {
        self.out = self.get_reg(A_REG);
    }

    /// The enable interrupts (EI) instruction enables interrupts.
    fn ei_op(&mut self)
    {
        self.interrupts_enabled = true;
    }

    /// The disable interrupts (DI) instruction disables interrupts.
    fn di_op(&mut self)
    {
        self.interrupts_enabled = false;
    }

    /// The increment (INR) instruction increments the value of the specified register by 1.
    fn inr_op(&mut self)
    {
        let reg                   = self.current_op.low_nibble.unwrap();
        let reg_value             = self.get_reg(reg);
        self.flags.auxiliary_flag = reg_value & 0x0F == 0x0F;
        let (res, _)              = reg_value.overflowing_add(1);
        self.flags.parity_flag    = parity(res);
        self.flags.sign_flag      = sign(res);
        self.flags.zero_flag      = zero(res);
        self.set_reg(reg, res);
    }

    /// call non zero (CNZ) instruction calls the address specified in the next two bytes in the
    fn cnz_op(&mut self)
    {
        if !self.flags.zero_flag
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// call carry (CC) instruction calls the address specified in the next two bytes in the
    /// memory if the carry flag is set.
    fn cc_op(&mut self)
    {
        if self.flags.carry_flag
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// call non carry (CNC) instruction calls the address specified in the next two bytes in the
    /// memory if the carry flag is not set.
    fn cnc_op(&mut self)
    {
        if !self.flags.carry_flag
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// call parity odd (CPO) instruction calls the address specified in the next two bytes in the
    /// memory if the parity flag is not set.
    fn cpo_op(&mut self)
    {
        if !self.flags.parity_flag
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// call parity even (CPE) instruction calls the address specified in the next two bytes in the
    /// memory if the parity flag is set.
    fn cpe_op(&mut self)
    {
        if self.flags.parity_flag
        {
            self.call_op();
        }
        else
        {
            self.program_counter += 2;
        }
    }

    /// return carry (RC) instruction pops the address from the stack and sets the program counter 
    /// to that address if the carry flag is set.
    fn rc_op(&mut self)
    {
        if self.flags.carry_flag
        {
            self.ret_op();
        }
    }

    /// return non carry (RNC) instruction pops the address from the stack and sets the program counter
    /// to that address if the carry flag is not set.
    fn rnc_op(&mut self)
    {
        if !self.flags.carry_flag
        {
            self.ret_op();
        }
    }

    /// return zero (RZ) instruction pops the address from the stack and sets the program counter
    /// to that address if the zero flag is set.
    fn rz_op(&mut self)
    {
        if self.flags.zero_flag
        {
            self.ret_op();
        }
    }

    /// return non zero (RNZ) instruction pops the address from the stack and sets the program counter
    /// to that address if the zero flag is not set.
    fn rnz_op(&mut self)
    {
        if !self.flags.zero_flag
        {
            self.ret_op();
        }
    }

    /// return minus (RM) instruction pops the address from the stack and sets the program counter
    /// to that address if the sign flag is set.
    fn rm_op(&mut self)
    {
        if self.flags.sign_flag
        {
            self.ret_op();
        }
    }

    /// return plus (RP) instruction pops the address from the stack and sets the program counter
    /// to that address if the sign flag is not set.
    fn rp_op(&mut self)
    {
        if !self.flags.sign_flag
        {
            self.ret_op();
        }
    }

    /// return parity even (RPE) instruction pops the address from the stack and sets the program counter
    /// to that address if the parity flag is set.
    fn rpe_op(&mut self)
    {
        if self.flags.parity_flag
        {
            self.ret_op();
        }
    }

    /// return parity odd (RPO) instruction pops the address from the stack and sets the program counter
    /// to that address if the parity flag is not set.
    fn rpo_op(&mut self)
    {
        if !self.flags.parity_flag
        {
            self.ret_op();
        }
    }

    /// The exchange stack (XTHL) instruction exchanges the values of the HL register pair and the 
    /// top of the stack.
    fn lhld_op(&mut self)
    {
        let addr = self.get_direct_address();
        let l_reg_value = self.memory[addr as usize];
        let h_reg_value = self.memory[(addr + 1) as usize];
        self.set_reg(L_REG, l_reg_value);
        self.set_reg(H_REG, h_reg_value);
        self.program_counter += 2;
    }

    /// The exchange stack (XTHL) instruction exchanges the values of the HL register pair and the
    /// top of the stack.
    fn shld_op(&mut self)
    {
        let addr = self.get_direct_address();
        let l_reg_value = self.get_reg(L_REG);
        let h_reg_value = self.get_reg(H_REG);
        self.set_memory_at(addr, l_reg_value);
        self.set_memory_at(addr + 1, h_reg_value);
        self.program_counter += 2;
    }

    /// The store accumulator indirect (STAX) instruction stores the value of the accumulator at the
    /// address specified by the BC or DE register pair.
    fn stax_op(&mut self)
    {
        let (msb, lsb) = if self.current_op.machine_code & 0b00010000 == 0x10
        {
            self.get_reg_pair(DE_PAIR_REG)
        }
        else
        {
            self.get_reg_pair(BC_PAIR_REG)
        };

        let addr = ((msb as u16) << 8) + lsb as u16; 
        self.set_memory_at(addr, self.get_reg(A_REG));
    }

    /// set carry (STC) instruction sets the carry flag.
    fn stc_op(&mut self)
    {
        self.flags.carry_flag = true;
    }

    /// complement carry (CMC) instruction complements the carry flag.
    fn cmc_op(&mut self)
    {
        self.flags.carry_flag = !self.flags.carry_flag;
    }

    /// complement accumulator (CMA) instruction complements the accumulator.
    fn cma_op(&mut self)
    {
        self.registers.accumulator = !self.registers.accumulator;
    }

    /// decimal adjust accumulator (DAA) instruction adjusts the accumulator to a binary coded decimal.
    fn daa_op(&mut self)
    {
        let mut accumulator = self.registers.accumulator;

        if ((accumulator & 0x0F) > 9) || (self.flags.auxiliary_flag)
        {
            accumulator += 0x06;
            if (accumulator & 0x0F) < 0x09
            {
                self.flags.auxiliary_flag = true;
            }
        }

        if ((accumulator & 0xF0) > 0x90) || (self.flags.carry_flag)
        {
            (accumulator, self.flags.carry_flag) = accumulator.overflowing_add(0x60);
        }

        self.registers.accumulator = accumulator;
        sign(self.registers.accumulator);
        zero(self.registers.accumulator);
        parity(self.registers.accumulator);
    }

    /// special purpose register and accumulator (SPHL) instruction loads the value of the HL 
    /// register pair into the stack pointer.
    fn sphl_op(&mut self)
    {
        let stackpointer: u16 = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.stack_pointer = stackpointer;
    }

    /// xchange register pair (XCHG) instruction exchanges the values of the HL and DE register pairs.
    fn xthl_op(&mut self)
    {
        let lsb = self.get_memory_at(self.stack_pointer);
        let msb = self.get_memory_at(self.stack_pointer + 1);
        self.set_memory_at(self.stack_pointer, self.registers.l);
        self.set_memory_at(self.stack_pointer + 1, self.registers.h);
        self.registers.l = lsb;
        self.registers.h = msb;
    }

    /// load program counter and HL register pair (PCHL) instruction loads the value of the HL 
    /// register pair into the program counter.
    fn pchl_op(&mut self)
    {
        let program_counter: u16 = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.program_counter = program_counter - 1;
    }

    /// reset (RST) instruction pushes the address of the next instruction onto the stack and sets 
    /// the program counter to the specified address.
    fn rst_op(&mut self)
    {
        let reset_addr = self.current_op.machine_code & 0b00111000;
        self.memory[(self.stack_pointer - 1) as usize] = (self.program_counter & 0x0F) as u8;
        self.memory[(self.stack_pointer - 2) as usize] = (( self.program_counter >> 8 ) & 0x0F) as u8;
        self.stack_pointer -= 2;
        self.program_counter = ( reset_addr - 1 ) as u16;
    }

    pub fn get_immediate(&mut self) -> u8
    {
        return self.memory[(self.program_counter + 1) as usize];
    }

    pub fn get_direct_address(&mut self) -> u16
    {
        let pc        = self.program_counter as usize;
        let lsb_value = self.memory[pc + 1];
        let msb_value = self.memory[pc + 2];
        let addr: u16 = (msb_value as u16) << 8 | lsb_value as u16;
        return addr;
    }

    pub fn set_flags_cszp(&mut self, carry: bool, auxiliary_flag: bool, res: u8)
    {
        self.flags.auxiliary_flag = auxiliary_flag;
        self.flags.carry_flag  = carry;
        self.flags.parity_flag = parity(res);
        self.flags.sign_flag   = sign(res);
        self.flags.zero_flag   = zero(res);
    }

    pub fn set_clock_frequency(&mut self, clock_frequency: u32) {
        self.clock_frequency = clock_frequency;
    }
}
