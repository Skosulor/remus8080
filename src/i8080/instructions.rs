use crate::i8080::registers::*;

#[derive(Clone, Debug)]
pub enum AddressMode 
{
    Direct,
    Pair,
    StackPointer,
    ImmediateOneByte,
    ImmediateTwoBytes,
    Unknown,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum InstructionTypes 
{
    MOV, ADD, ADC, SUB, SBB, ANA, XRA, ORA, CMP, MVI, ADI, ACI, SUI,
    SBI, ANI, ORI, XRI, CPI, JMP, JNZ, JZ, JNC, JC, JPO, JPE, JP, JM,
    LXI, DCR, DAD, RRC, RLC, RAL, RAR, INX, DCX, LDA, LDAX, STA, PUSH,
    POP, CALL, NOP, RET, XCHG, OUT, EI, DI, INR, CP, CNZ, CC, CNC, CPO,
    CPE, CM, CZ, RC, RNC, RZ, RNZ, RM, RP, RPE, RPO, LHLD, SHLD, STAX,
    STC, CMC, CMA, DAA, SPHL, XTHL, PCHL, RST,
    Unknown,
}

const MOVE_ARITHMETIC_LOGICAL_INSTRUCTION_GROUP : u8 = 0x0;
const MOVE_INSTRUCTION_GROUP                    : u8 = 0x40;
const ARITHMETIC_LOGICAL_INSTRUCTION_GROUP      : u8 = 0x80;
const BRANCH_STACK_INSTRUCTION_GROUP            : u8 = 0xC0;

const ARITHMETIC_LOGICAL_GROUP_MASK: u8 = 0b10111000;
const OP_CODE_GROUP_MASK:            u8 = 0xC0;

const ADD_GROUP: u8 = 0b10000000;
const ADC_GROUP: u8 = 0b10001000;
const SUB_GROUP: u8 = 0b10010000;
const SBB_GROUP: u8 = 0b10011000;
const ANA_GROUP: u8 = 0b10100000;
const XRA_GROUP: u8 = 0b10101000;
const ORA_GROUP: u8 = 0b10110000;
const CMP_GROUP: u8 = 0b10111000;

#[derive(Clone, Debug)]
pub struct Instruction 
{
    pub machine_code    : u8,
    immediate_lsb       : u8,
    immediate_msb       : u8,
    name                : String,
    cycles              : u8,
    address_mode         : AddressMode,
    pub instruction_type: InstructionTypes,
    pub low_nibble      : Option<u8>,
    pub high_nibble     : Option<u8>,
}

impl Instruction
{
    pub fn new() -> Instruction
    {
        let ins = Instruction 
        {
            machine_code    : 0,
            immediate_lsb   : 0,
            immediate_msb   : 0,
            name            : "_".to_string(),
            cycles          : 1,
            address_mode     : AddressMode  ::Unknown,
            instruction_type: InstructionTypes::Unknown,
            low_nibble      : None,
            high_nibble     : None,
        };
        ins
    }

    pub fn from_byte(b: u8) -> Instruction 
    {
        let mut instruction = Instruction::new();
        instruction.byte_to_op(b, 0, 0);
        return instruction;
    }

    pub fn get_name_byte(self) -> (u8, String) 
    {
        (self.machine_code, self.name.clone())
    }

    pub fn get_length(&self) -> u8
    {
        return match self.address_mode
        {
            AddressMode::Direct => 1,
            AddressMode::Pair => 1,
            AddressMode::StackPointer => 1,
            AddressMode::ImmediateOneByte => 2,
            AddressMode::ImmediateTwoBytes => 3,
            AddressMode::Unknown => 1,
        };
    }


    pub fn byte_to_op(&mut self, b: u8, immediate_lsb: u8, immediate_msb: u8) 
    {
        *self = Instruction::new();
        self.machine_code = b;
        self.immediate_lsb = immediate_lsb;
        self.immediate_msb = immediate_msb;

        match b & OP_CODE_GROUP_MASK
        {
            // Move Instructions
            // low_nibble holds the destination register
            // high_nibble holds the source register
            MOVE_INSTRUCTION_GROUP => 
            {

                self.address_mode = AddressMode::Direct;
                self.instruction_type = InstructionTypes::MOV;
                self.low_nibble = Some((b >> MOVE_TO_BIT_POS) & REGISTER_MASK);
                self.high_nibble = Some((b >> MOVE_FROM_BIT_POS) & REGISTER_MASK);
                let name = format!("MOV  {},{} ", Registers::translate_to_reg(self.low_nibble.unwrap()), Registers::translate_to_reg(self.high_nibble.unwrap()));
                self.cycles = 1;
                self.name = name;
                return
            },
            // low_nibble holds the source register
            // high_nibble is unused
            ARITHMETIC_LOGICAL_INSTRUCTION_GROUP =>
            {
                self.low_nibble = Some((b >> ARITHMETIC_WITH) & REGISTER_MASK);
                self.address_mode = AddressMode::Direct;
                self.cycles = 1;
                match b & ARITHMETIC_LOGICAL_GROUP_MASK
                {
                    ADD_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ADD;
                        let name = format!("ADD  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    ADC_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ADC;
                        let name = format!("ADC  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    SUB_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::SUB;
                        let name = format!("SUB  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    SBB_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::SBB;
                        let name = format!("SBB  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    },
                    ANA_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ANA;
                        let name = format!("ANA  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    XRA_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::XRA;
                        let name = format!("XRA  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    ORA_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::ORA;
                        let name = format!("ORA  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    CMP_GROUP =>
                    {
                        self.instruction_type = InstructionTypes::CMP;
                        let name = format!("CMP  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
                        self.name = name;
                    }
                    _ => panic!("Arithemtic does not exist!: {:X}", self.machine_code),
                }
            },
            BRANCH_STACK_INSTRUCTION_GROUP => 
            {
                match b 
                {
                    0xC0 => self.set_instruction(InstructionTypes::RNZ, "", 1, AddressMode::Direct),
                    0xC1 | 0xD1 | 0xE1 | 0xF1 => self.decode_pop(),
                    0xC2 | 0xC3 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => self.decode_jump_instructions(),
                    0xC4 | 0xCC | 0xCD | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => self.decode_call_instructions(),
                    0xC5 | 0xD5 | 0xE5 | 0xF5 => self.decode_push(),
                    0xC6 => self.byte_to_immediate_op(),
                    0xC7 => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    0xC8 => self.set_instruction(InstructionTypes::RZ, "", 1, AddressMode::Direct),
                    0xC9 => self.set_instruction(InstructionTypes::RET, "", 1, AddressMode::Direct),
                    0xCB => self.name = "?? NOT IMP".to_string(),
                    0xCE => self.byte_to_immediate_op(),
                    0xCF => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    0xD0 => self.set_instruction(InstructionTypes::RNC, "", 1, AddressMode::Direct),
                    0xD3 => self.set_instruction(InstructionTypes::OUT, "", 2, AddressMode::Unknown),
                    0xD6 => self.byte_to_immediate_op(),
                    0xD7 => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    0xD8 => self.set_instruction(InstructionTypes::RC, "", 1, AddressMode::Direct),
                    0xD9 => self.name = "?? NOT IMP".to_string(),
                    0xDB => self.name = "IN NOT IMP".to_string(),
                    0xDD => self.name = "?? NOT IMP".to_string(),
                    0xDE => self.byte_to_immediate_op(),
                    0xDF => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    0xE0 => self.set_instruction(InstructionTypes::RPO, "", 1, AddressMode::StackPointer),
                    0xE3 => self.set_instruction(InstructionTypes::XTHL, "", 1, AddressMode::Pair),
                    0xE6 => self.byte_to_immediate_op(),
                    0xE7 => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    0xE8 => self.set_instruction(InstructionTypes::RPE, "", 1, AddressMode::Direct),
                    0xE9 => self.set_instruction(InstructionTypes::PCHL, "", 1, AddressMode::Pair),
                    0xEB => self.set_instruction(InstructionTypes::XCHG, "", 1, AddressMode::Pair),
                    0xED => self.name = "?? NOT IMP".to_string(),
                    0xEE => self.byte_to_immediate_op(),
                    0xEF => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    0xF0 => self.set_instruction(InstructionTypes::RP, "", 1, AddressMode::Direct),
                    0xF3 => self.set_instruction(InstructionTypes::DI, "", 1, AddressMode::Unknown),
                    0xF6 => self.byte_to_immediate_op(),
                    0xF7 => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    0xF8 => self.set_instruction(InstructionTypes::RM, "", 1, AddressMode::Direct),
                    0xF9 => self.set_instruction(InstructionTypes::SPHL, "", 1, AddressMode::Pair),
                    0xFB => self.set_instruction(InstructionTypes::EI, "", 1, AddressMode::Unknown),
                    0xFD => self.name = "?? NOT IMP".to_string(),
                    0xFE => self.byte_to_immediate_op(),
                    0xFF => self.set_instruction(InstructionTypes::RST, "", 1, AddressMode::Direct),
                    _ => 
                    {
                        println!("Byte: {:02X}", b);
                        panic!("Should not exist!");
                    }
                }
            },

            MOVE_ARITHMETIC_LOGICAL_INSTRUCTION_GROUP => 
            {
                match b & 0b00111111 
                {
                    0x00                      => self.set_instruction(InstructionTypes::NOP, "", 1, AddressMode::Unknown),
                    0x01 | 0x11 | 0x21 | 0x31 => self.decode_lxi(),
                    0x02 | 0x12               => self.set_instruction(InstructionTypes::STAX, "", 1, AddressMode::Pair),
                    0x03 | 0x13 | 0x23 | 0x33 => self.decode_inx(),
                    0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C  => self.decode_inr(),
                    0x05 | 0x0D |  0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => self.decode_dcr(),
                    0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E  => self.byte_to_immediate_op(),
                    0x07 => self.set_instruction(InstructionTypes::RLC, "", 1, AddressMode::Direct),
                    0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38        => self.name = "__ NOT IMP".to_string(),
                    0x09 | 0x19 | 0x29 | 0x39 => self.decode_dad(),
                    0x0B | 0x1B | 0x2B | 0x3B => self.decode_dcx(),
                    0x0F => self.set_instruction(InstructionTypes::RRC, "", 1, AddressMode::Direct),
                    0x17 => self.set_instruction(InstructionTypes::RAL, "", 1, AddressMode::Direct),
                    0x1A | 0x0A => self.set_instruction(InstructionTypes::LDAX, "", 1, AddressMode::Pair),
                    0x1F => self.set_instruction(InstructionTypes::RAR,  "", 1, AddressMode::Direct),
                    0x22 => self.set_instruction(InstructionTypes::SHLD, "", 3, AddressMode::ImmediateTwoBytes),
                    0x27 => self.set_instruction(InstructionTypes::DAA,  "", 1, AddressMode::Unknown),
                    0x2A => self.set_instruction(InstructionTypes::LHLD, "", 3, AddressMode::ImmediateTwoBytes),
                    0x2F => self.set_instruction(InstructionTypes::CMA,  "", 1, AddressMode::Unknown),
                    0x32 => self.set_instruction(InstructionTypes::STA,  "", 3, AddressMode::ImmediateTwoBytes),
                    0x37 => self.set_instruction(InstructionTypes::STC,  "", 1, AddressMode::Unknown),
                    0x3A => self.set_instruction(InstructionTypes::LDA,  "", 3, AddressMode::ImmediateTwoBytes),
                    0x3F => self.set_instruction(InstructionTypes::CMC,  "", 1, AddressMode::Unknown),
                    _ => panic!("Misc should not exist!"),
                }
            },
            _ => panic!("This match should never happen")

        }

    }

    fn decode_pop(&mut self)
    {
        self.low_nibble = Some(( self.machine_code & 0x30 ) >> 4);
        self.name = format!("POP  {}", Registers::translate_to_reg_pair(self.low_nibble.unwrap()));
        self.address_mode = AddressMode::StackPointer;
        self.cycles = 1;
        self.instruction_type = InstructionTypes::POP;
    }

    fn decode_push(&mut self)
    {
        self.low_nibble = Some(( self.machine_code & 0x30 ) >> 4);
        self.name = format!("PUSH {}", Registers::translate_to_reg_pair(self.low_nibble.unwrap()));
        self.address_mode = AddressMode::StackPointer;
        self.cycles = 1;
        self.instruction_type = InstructionTypes::PUSH;
    }

    fn decode_inx(&mut self)
    {
        self.set_instruction(InstructionTypes::INX, "", 1, AddressMode::Pair);
        self.low_nibble = Some((self.machine_code & 0x30) >> 4);
    }

    fn decode_dcx(&mut self)
    {
        self.set_instruction(InstructionTypes::DCX, "", 1, AddressMode::Direct);
        self.low_nibble = Some((self.machine_code & 0x30) >> 4);
    }

    fn decode_dad(&mut self)
    {
        self.set_instruction(InstructionTypes::DAD, "", 1, AddressMode::Direct);
        self.low_nibble = Some((self.machine_code & 0x30) >> 4);
    }

    fn decode_lxi(&mut self)
    {
        self.address_mode = AddressMode::Direct;
        self.set_instruction(InstructionTypes::LXI, "", 3, AddressMode::ImmediateOneByte);
        let address: u16 = self.immediate_lsb as u16 | ((self.immediate_msb as u16) << 8);
        let reg_pair = Registers::translate_to_reg_pair((self.machine_code & 0x30) >> 4);
        self.name = format!("LXI  {},{}", reg_pair, address);
        self.low_nibble = Some((self.machine_code & 0x30) >> 4);
    }

    fn decode_dcr(&mut self)
    {
        self.address_mode = AddressMode::Direct;
        self.instruction_type = InstructionTypes::DCR;
        self.low_nibble = Some((self.machine_code & 0x38) >> 3);
        self.cycles = 1;
        self.name = format!("DCR  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
    }

    fn decode_inr(&mut self)
    {
        self.instruction_type = InstructionTypes::INR;
        self.low_nibble = Some((self.machine_code & 0x38) >> 3);
        self.cycles = 1;
        self.address_mode = AddressMode::Direct;
        self.name = format!("INR  {}", Registers::translate_to_reg(self.low_nibble.unwrap()));
    }

    fn decode_jump_instructions(&mut self)
    {
        let instruction = match self.machine_code
        {
            0xC2 => InstructionTypes::JNZ,
            0xC3 => InstructionTypes::JMP,
            0xCA => InstructionTypes::JZ,
            0xD2 => InstructionTypes::JNC,
            0xDA => InstructionTypes::JC,
            0xE2 => InstructionTypes::JPO,
            0xEA => InstructionTypes::JPE,
            0xF2 => InstructionTypes::JP,
            0xFA => InstructionTypes::JM,
            _ => panic!("Error, should not be here!"),
        };

        self.address_mode = AddressMode::ImmediateTwoBytes;
        self.cycles = 3;
        self.instruction_type = instruction.clone();
        let address = self.immediate_lsb as u16 | ((self.immediate_msb as u16) << 8);
        self.name = format!("{:<4} {}", Instruction::instruction_to_string(instruction), address);
    }

    fn immediate_op_helper(&mut self, name1: String, op1: InstructionTypes, name2: String, op2: InstructionTypes)
    {
        if self.machine_code & 0x0F == 0x06
        {
            self.instruction_type = op1;
            self.name = name1;
        }
        else if self.machine_code & 0x0F == 0x0E
        {
            self.instruction_type = op2;
            self.name = name2;
        }
        else
        {
            panic!("Error, should either be {} or {}", name1, name2);
        }
        self.name = format!("{:<4} {}", self.name, self.immediate_lsb);
    }

    fn byte_to_immediate_op(&mut self)
    {
        self.address_mode = AddressMode::ImmediateOneByte;
        self.cycles = 2;

        match self.machine_code & 0xF0
        {
            0x00 | 0x10 | 0x20 | 0x30 =>
            {
                self.instruction_type = InstructionTypes::MVI;
                self.low_nibble = Some((self.machine_code & 0x38) >> 3);
                let name = format!("MVI  {},{} ", Registers::translate_to_reg(self.low_nibble.unwrap()), self.immediate_lsb.to_string());
                self.name = name;
                return
            },
            // ADI, ACI
            0xC0 => 
            {
                self.immediate_op_helper("ADI".to_string(), InstructionTypes::ADI, "ACI".to_string(), InstructionTypes::ACI);
            },
            // SUI, SBI
            0xD0 => 
            {
                self.immediate_op_helper("SUI".to_string(), InstructionTypes::SUI, "SBI".to_string(), InstructionTypes::SBI);
            },
            // ANI, XRI
            0xE0 => 
            {
                self.immediate_op_helper("ANI".to_string(), InstructionTypes::ANI, "XRI".to_string(), InstructionTypes::XRI);
            },
            // ORI, CPI
            0xF0 =>
            {
                self.immediate_op_helper("ORI".to_string(), InstructionTypes::ORI, "CPI".to_string(), InstructionTypes::CPI);
            },
            _ => (),
        }
    }

    fn decode_call_instructions(&mut self)
    {
        let instruction = match self.machine_code
        {
            
            0xC4 => InstructionTypes::CNZ,
            0xCC => InstructionTypes::CZ,
            0xCD => InstructionTypes::CALL,
            0xD4 => InstructionTypes::CNC,
            0xDC => InstructionTypes::CC,
            0xE4 => InstructionTypes::CPO,
            0xEC => InstructionTypes::CPE,
            0xF4 => InstructionTypes::CP,
            0xFC => InstructionTypes::CM,
            _    => panic!("Uknown Call instruction"),
        };

        self.address_mode = AddressMode::ImmediateTwoBytes;
        self.cycles = 3;
        self.instruction_type = instruction.clone();
        let address = self.immediate_lsb as u16 | ((self.immediate_msb as u16) << 8);
        self.name = format!("{:<4} {}", Instruction::instruction_to_string(instruction), address);
    }

    fn set_instruction(&mut self, inst: InstructionTypes, suffix: &str, cycles: u8, address_mode: AddressMode)
    {
        self.address_mode = address_mode;
        self.name = format!("{}{}", Instruction::instruction_to_string(inst.clone()), suffix);
        self.cycles = cycles;
        self.instruction_type = inst;
    }

    fn instruction_to_string<T: std::fmt::Debug>(e: T) -> String
    {
        return format!("{:?}", e);
    }
}


