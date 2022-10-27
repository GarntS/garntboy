/* file:    cpu.rs
 * author:  garnt
 * date:    10/26/2022
 * desc:    code to handle the threaded execution of and instruction emulation
 *          for garntboy.
 */

use crate::instruction::{Instruction, INSTRUCTIONS};
use crate::garntboy_state::GarntboyState;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

// constant representing the amount of time in nanoseconds that
// should be slept between clock cycles in order to keep
const gb_clock_rate: usize = 4_194_304;
const nanos_between_clocks: usize = 1_000_000_000 / gb_clock_rate;

// cpu_thread_func() is the function that runs on the cpu thread and is
// responsible for executing the emulated cpu instructions
pub fn cpu_thread_func(state_mtx: Mutex<GarntboyState>) {
    loop {
        // keep track of when we started executing the current instruction so that
        // we can start executing the next one at the right time
        let now = Instant::now();

        // lock the state mutex
        let mut state = state_mtx.lock().unwrap();

        // do nothing this cycle if we need to sleep
        if state.cycles_to_sleep > 0 {
            state.cycles_to_sleep -= 1;

            // sleep until the next clock cycle should start
            thread::sleep(Duration::from_nanos(nanos_between_clocks) - now.elapsed())

            // do nothing and loop
            continue;
        }

        // if we're ready to execute a new instruction, and there are state
        // changes queued, make the state changes before executing the next
        // instruction
        if state.has_queued_changes {
            // make register changes
            state.registers = state.register_changes;

            // make memory changes
            state.memory_changes.drain().for_each(|(addr, val)| {
                state.memory[addr] = val;
            });
        }

        // execute the new instruction
        let opcode: u8 = state.memory[state.registers.pc()];
        execute_single_insn(opcode, state);

        // sleep until the next clock cycle should start
        thread::sleep(Duration::from_nanos(nanos_between_clocks) - now.elapsed())
    }
}