/* file:    register_state.rs
 * author:  garnt
 * date:    10/18/2022
 * desc:    representation of the register state of the gameboy processor
 */

use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian};
use std::fmt;

// bitflags for the F register
bitflags! {
    // Results in default value with bits: 0
    struct CPUFlags: u8 {
        const Z = 0b10000000;
        const N = 0b01000000;
        const H = 0b00100000;
        const C = 0b00010000;
    }
}

// struct definition for the CPU register state
#[derive(Clone, Copy, Debug)]
pub struct RegisterState {
    reg_af: [u8; 2],
    reg_bc: [u8; 2],
    reg_de: [u8; 2],
    reg_hl: [u8; 2],
    reg_sp: u16,
    reg_pc: u16,
}

impl fmt::Display for RegisterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[pc: {:#x}, sp: {:#x}, af: {:#x}, bc: {:#x}, de: {:#x}, hl: {:#x}]",
            self.reg_pc, self.reg_sp, 
            LittleEndian::read_u16(&self.reg_af),
            LittleEndian::read_u16(&self.reg_bc),
            LittleEndian::read_u16(&self.reg_de),
            LittleEndian::read_u16(&self.reg_hl))
    }
}

// methods for register state
impl RegisterState {
    // constructor
    pub fn new() -> RegisterState {
        RegisterState {
            reg_af: [0; 2],
            reg_bc: [0; 2],
            reg_de: [0; 2],
            reg_hl: [0; 2],
            reg_sp: 0,
            reg_pc: 0
        }
    }

    // getters and setters for flags
    pub fn cpu_flags(&mut self) -> CPUFlags {
        CPUFlags::from_bits(self.f()).unwrap()
    }

    pub fn set_cpu_flags(&mut self, val: CPUFlags) {
        self.set_f(val.bits())
    }

    // getters/setters for 16-bit registers
    pub fn af(&mut self) -> u16 {
        LittleEndian::read_u16(&self.reg_af)
    }

    pub fn set_af(&mut self, val: u16) {
        LittleEndian::write_u16(&mut self.reg_af, val)
    }

    pub fn bc(&mut self) -> u16 {
        LittleEndian::read_u16(&self.reg_bc)
    }

    pub fn set_bc(&mut self, val: u16) {
        LittleEndian::write_u16(&mut self.reg_bc, val)
    }

    pub fn de(&mut self) -> u16 {
        LittleEndian::read_u16(&self.reg_de)
    }

    pub fn set_de(&mut self, val: u16) {
        LittleEndian::write_u16(&mut self.reg_de, val)
    }

    pub fn hl(&mut self) -> u16 {
        LittleEndian::read_u16(&self.reg_hl)
    }

    pub fn set_hl(&mut self, val: u16) {
        LittleEndian::write_u16(&mut self.reg_hl, val)
    }

    pub fn sp(&mut self) -> u16 {
        self.reg_sp
    }

    pub fn set_sp(&mut self, val: u16) {
        self.reg_sp = val
    }

    pub fn pc(&mut self) -> u16 {
        self.reg_pc
    }

    pub fn set_pc(&mut self, val: u16) {
        self.reg_pc = val
    }

    // getters/setters for 8-bit registers
    pub fn a(&mut self) -> u8 {
        self.reg_af[1]
    }

    pub fn set_a(&mut self, val: u8) {
        self.reg_af[1] = val
    }

    pub fn f(&mut self) -> u8 {
        self.reg_af[0]
    }

    pub fn set_f(&mut self, val: u8) {
        self.reg_af[0] = val
    }

    pub fn b(&mut self) -> u8 {
        self.reg_af[1]
    }

    pub fn set_b(&mut self, val: u8) {
        self.reg_af[1] = val
    }

    pub fn c(&mut self) -> u8 {
        self.reg_af[0]
    }

    pub fn set_c(&mut self, val: u8) {
        self.reg_af[0] = val
    }

    pub fn d(&mut self) -> u8 {
        self.reg_af[1]
    }

    pub fn set_d(&mut self, val: u8) {
        self.reg_af[1] = val
    }

    pub fn e(&mut self) -> u8 {
        self.reg_af[0]
    }

    pub fn set_e(&mut self, val: u8) {
        self.reg_af[0] = val
    }

    pub fn h(&mut self) -> u8 {
        self.reg_af[1]
    }

    pub fn set_h(&mut self, val: u8) {
        self.reg_af[1] = val
    }

    pub fn l(&mut self) -> u8 {
        self.reg_af[0]
    }

    pub fn set_l(&mut self, val: u8) {
        self.reg_af[0] = val
    }
}