/* file:    garntboy_state.rs
 * author:  garnt
 * date:    10/18/2022
 * desc:    struct containing the internal state of a running garntboy instance
 */

use crate::register_state::RegisterState;
use std::collections::HashMap;

// struct containing the state of a single running garntboy instance
pub struct GarntboyState {
    pub registers: RegisterState,
    pub memory: [u8; u16::MAX as usize],
    pub cycles_to_sleep: u8,
    pub has_queued_changes: bool,
    pub register_changes: RegisterState,
    pub memory_changes: HashMap<usize, u8>,
}

impl GarntboyState {
    pub fn new() -> GarntboyState {
        GarntboyState {
            registers: RegisterState::new(),
            memory: [0; u16::MAX as usize],
            cycles_to_sleep: 0,
            has_queued_changes: false,
            register_changes: RegisterState::new(),
            memory_changes: HashMap::new(),
        }
    }
}