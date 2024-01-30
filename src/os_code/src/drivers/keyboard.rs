use spin::Mutex;
use x86::io::inb;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use lazy_static::lazy_static;

const KEYB_DATA_PORT : u8 = 0x60;
const _KEYB_STATUS_REG : u8 = 0x64; //This is for reading.
const _KEYB_COMMAND_REG : u8 = 0x64; //this is for writing.

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, 
                                 HandleControl::Ignore));
}

pub fn keyboard_read() -> Result<DecodedKey, u8> {
    let mut handle = KEYBOARD.lock();

    let scan_code = unsafe { inb(KEYB_DATA_PORT as u16) };
    
    if let Ok(Some(key)) = handle.add_byte(scan_code) {
        if let Some(key) = handle.process_keyevent(key) {
            return Ok(key);
        }
    }
    Err(scan_code)
}
