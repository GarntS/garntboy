// use garntboy_core
mod instruction;
mod register_state;
mod garntboy_state;
mod garntboy_core;

#[no_mangle]
pub extern "C" fn main() {
    println!("Hello, world!");
}
