pub mod en;
pub mod apple_fr;

use crate::{print, println};
use crate::io::{outb, inb};

pub struct KeyboardState {
    lshift: bool,
    rshift: bool,
    lctrl: bool,
    loption: bool,
    roption: bool,
    lcmd: bool,
    rcmd: bool,
    fn_btn: bool,
    caps_lock: bool,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            lshift: false,
            rshift: false,
            lctrl: false,
            loption: false,
            roption: false,
            lcmd: false,
            rcmd: false,
            fn_btn: false,
            caps_lock: false
        }
    }
}

use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    static ref KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState::default());
}

fn key_unpress(knbr: u8) {
    let unpressed_knbr = knbr - 128;
    match crate::keyboard::apple_fr::KMAP.get(unpressed_knbr as usize) {
        Some(key) => {
            let mut state = KEYBOARD_STATE.lock();
            if knbr == apple_fr::Kvalue::Ctrl as u8 {
                state.lctrl = false;
            } else if knbr == apple_fr::Kvalue::LShift as u8 {
                state.lshift = false;
            } else if knbr == apple_fr::Kvalue::Rshift as u8  {
                state.rshift = false;
            } else if knbr == apple_fr::Kvalue::Option as u8  {
                // loption and roption have the same value in macos keyboard
                state.loption = false;
            }
        }
        None => {
            let mut state = KEYBOARD_STATE.lock();
            if unpressed_knbr == 91 {
                state.lcmd = false;
            } else if unpressed_knbr == 92 {
                state.rcmd = false;
            } else {
                println!("None : {}", unpressed_knbr);
            }
        }
    }
}

fn key_press(knbr: u8) {
    let mut map = crate::keyboard::apple_fr::KMAP;
    let mut state = KEYBOARD_STATE.lock();
    
    if state.lshift == true || state.rshift == true || state.caps_lock == true {
        map = crate::keyboard::apple_fr::KMAP_SHIFT;
    }
    match map.get(knbr as usize) {
        Some(key) => {
            if knbr == apple_fr::Kvalue::Ctrl as u8 {
                state.lctrl = true;
            } else if knbr == apple_fr::Kvalue::LShift as u8 {
                state.lshift = true;
            } else if knbr == apple_fr::Kvalue::Rshift as u8  {
                state.rshift = true;
            } else if knbr == apple_fr::Kvalue::Option as u8  {
                // loption and roption have the same value in macos keyboard
                state.loption = true;
            } else if knbr == apple_fr::Kvalue::ShiftLock as u8 {
                state.caps_lock = !state.caps_lock;
            } else {
                println!("{} {}", key, knbr);
            }
        }
        None => {
            let mut state = KEYBOARD_STATE.lock();
            if knbr == 91 {
                state.lcmd = true;
            } else if knbr == 92 {
                state.rcmd = true;
            } else if knbr < 128 {
                println!("nbr: {}", knbr);
            }
        }
    }
}

pub fn handle_keypress() {
    let knbr = inb(0x60);
    if knbr > 127 {
        key_unpress(knbr);
    } else {
        key_press(knbr);
    }
}