/* file:    garntboy_core.rs
 * author:  garnt
 * date:    10/18/2022
 * desc:    rust_libretro core implementation for garntboy.
 */

use crate::garntboy_state::GarntboyState;
use crate::register_state::RegisterState;
use rust_libretro::retro_core;
use rust_libretro::contexts::{CheatResetContext, CheatSetContext, DeinitContext, GetAvInfoContext, InitContext, LoadGameContext, RunContext};
use rust_libretro::core::{CoreOptions, Core};
use rust_libretro::types::SystemInfo;
use rust_libretro_sys::{retro_game_geometry, retro_game_info, retro_system_av_info, retro_system_timing};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CStr, CString};

// struct definition for our rust_libretro core
struct GarntboyCore {
    option_1: bool,
    option_2: bool,
    pixels: [u8; 160 * 144 * 4],
    timer: i64,
    even: bool,
    state: GarntboyState,
    cheat_memory_cache: HashMap<usize, Box<[u8]>>
}

// invoking this macro does... something.
retro_core!(GarntboyCore {
    option_1: false,
    option_2: true,
    pixels: [0; 160 * 144 * 4],
    timer: 5_000_001,
    even: true,
    state: GarntboyState::new(),
    cheat_memory_cache: HashMap::new()
});

// implementation for CoreOptions
impl CoreOptions for GarntboyCore {}

// implementation for Core
impl Core for GarntboyCore {
    // get_info() feeds retroarch some system information about this core
    fn get_info(&self) -> SystemInfo {
        SystemInfo {
            library_name: CString::new("garntboy").unwrap(),
            library_version: CString::new("0.1.0").unwrap(),
            valid_extensions: CString::new("bin|gb|gbc").unwrap(),
            need_fullpath: false,
            block_extract: false,
        }
    }

    // on_get_av_info() feeds retroarch some system information about the audio and video
    fn on_get_av_info(&mut self, _ctx: &mut GetAvInfoContext) -> retro_system_av_info {
        retro_system_av_info {
            geometry: retro_game_geometry {
                base_width: 160,
                base_height: 144,
                max_width: 160,
                max_height: 144,
                aspect_ratio: 160.0 / 144.0,
            },
            timing: retro_system_timing {
                fps: 60.0,
                sample_rate: 44100.0,
            },
        }
    }

    // roughly equivalent to retro_init()
    fn on_init(&mut self, _ctx: &mut InitContext) {
        self.state.registers = RegisterState::new();
        self.state.memory = [0; u16::MAX as usize];
    }

    // roughly equivalent to retro_deinit()
    fn on_deinit(&mut self, _ctx: &mut DeinitContext) {}

    // on_load_game() is a callback triggered when a new ROM is loaded
    fn on_load_game(&mut self, game: Option<retro_game_info>, _ctx: &mut LoadGameContext<'_>) -> Result<(), Box<dyn Error>> {
        // copy the ROM over into memory
        let game_struct: &retro_game_info = &game.unwrap();
        let rom_slice: &[u8];
        unsafe {
            rom_slice = std::slice::from_raw_parts(game_struct.data as *const u8, game_struct.size as usize);
        }
        self.state.memory.copy_from_slice(rom_slice);

        // set IP to be the start of the cartridge header, which is the entry point
        self.state.registers.set_pc(0x100);

        // if shit hasn't broke at this point, it worked properly.
        Ok(())
    }

    // applying cheats, hysterically, is trivial to implement.
    fn on_cheat_set(&mut self, index: std::os::raw::c_uint, enabled: bool, code: &CStr, _ctx: &mut CheatSetContext) {
        if enabled {
            let code_bytes: &[u8] = code.to_bytes();
            let mem_bytes: &mut [u8] = &mut self.state.memory[(index as usize)..(index as usize + code_bytes.len())];
            self.cheat_memory_cache.insert(index as usize, mem_bytes.to_vec().into_boxed_slice());
            mem_bytes.clone_from_slice(code_bytes);
        }
    }

    // resets all applied cheats
    fn on_cheat_reset(&mut self, _ctc: &mut CheatResetContext) {
        self.cheat_memory_cache
            .drain()
            .for_each(|(key, value)| {
                let mem_bytes: &mut [u8] = &mut self.state.memory[key..(key + value.len())];
                mem_bytes.clone_from_slice(&value);
            });
    }

    // roughly equivalent to retro_run()
    fn on_run(&mut self, _ctx: &mut RunContext, _delta_us: Option<i64>) {
        
    }
}