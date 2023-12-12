pub mod en;
pub mod fr;

use crate::{print, println};
use crate::io::{outb, inb};


pub struct Stack {
    data: [u8; 10]
}

impl Default for Stack {
    fn default() -> Self {
        Self { data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
    }
}

impl Stack {
    fn push(&mut self, elem: u8) {
        for i in (1..10).rev() {
            self.data[i] = self.data[i - 1];
        }
        self.data[0] = elem;
    }

    fn pop(&mut self) {
        for i in 0..9 {
            self.data[i] = self.data[i + 1];
        }
        self.data[9] = 0;
    }

    fn has(&mut self, elem: u8) -> bool {
        for i in 0..10 {
            if self.data[i] == elem {
                return true;
            }
        }
        return false;
    }
}

pub struct Kmap<'a> {
    map: &'a[&'a str; 69],
    map_shift: &'a[&'a str; 69]
}

pub struct KeyboardState<'a> {
    lshift: bool,
    rshift: bool,
    lctrl: bool,
    loption: bool,
    lcmd: bool,
    rcmd: bool,
    fn_btn: bool,
    caps_lock: bool,
    lang: [Kmap<'a>; 2],
    lang_id: usize,
    pressed_key: Stack,
    active_key: Stack
}

impl Default for KeyboardState<'_> {
    fn default() -> Self {
        Self {
            lshift: false,
            rshift: false,
            lctrl: false,
            loption: false,
            lcmd: false,
            rcmd: false,
            fn_btn: false,
            caps_lock: false,
            lang: [
                Kmap {
                    map: &fr::KMAP, map_shift: &fr::KMAP_SHIFT
                },
                Kmap {
                    map: &en::KMAP, map_shift: &en::KMAP_SHIFT
                }
            ],
            lang_id: 0,
            pressed_key: Stack::default(),
            active_key: Stack::default()
        }
    }
}

use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

lazy_static! {
    static ref KEYBOARD_STATE: Mutex<KeyboardState<'static>> = Mutex::new(KeyboardState::default());
}

fn is_switch_kb(knbr: u8, kb_state: &MutexGuard<'_, KeyboardState<'_>>) -> bool {
    if knbr == fr::Kvalue::Space as u8 && kb_state.loption && kb_state.active_key.data[1] == 0 {
        return true;
    }
    return false;
}

fn key_unpress(knbr: u8) {
    let unpressed_knbr = knbr - 128;
    let mut state = KEYBOARD_STATE.lock();

    match state.lang[state.lang_id].map.get(unpressed_knbr as usize) {
        Some(_key) => {
            if unpressed_knbr == fr::Kvalue::Ctrl as u8 {
                state.lctrl = false;
            } else if unpressed_knbr == fr::Kvalue::LShift as u8 {
                state.lshift = false;
            } else if unpressed_knbr == fr::Kvalue::Rshift as u8  {
                state.rshift = false;
            } else if unpressed_knbr == fr::Kvalue::Option as u8  {
                // loption and roption have the same value in macos keyboard
                state.loption = false;
            } else {
                state.pressed_key.push(knbr);
            }
            state.active_key.pop();
        }
        None => {
            if unpressed_knbr == 91 {
                state.lcmd = false;
            } else if unpressed_knbr == 92 {
                state.rcmd = false;
            } else {
                println!("None : {}", unpressed_knbr);
            }
            state.active_key.pop();
        }
    }
}

fn key_press(knbr: u8) {
    let mut state = KEYBOARD_STATE.lock();
    let map;
    
    if state.lshift == true || state.rshift == true || state.caps_lock == true {
        map = state.lang[state.lang_id].map_shift;
    } else {
        map = state.lang[state.lang_id].map;
    }
    match map.get(knbr as usize) {
        Some(key) => {
            if knbr == fr::Kvalue::Ctrl as u8 {
                state.lctrl = true;
            } else if knbr == fr::Kvalue::LShift as u8 {
                state.lshift = true;
            } else if knbr == fr::Kvalue::Rshift as u8  {
                state.rshift = true;
            } else if knbr == fr::Kvalue::Option as u8  {
                // loption and roption have the same value in macos keyboard
                state.loption = true;
            } else if knbr == fr::Kvalue::ShiftLock as u8 {
                state.caps_lock = !state.caps_lock;
            } else if is_switch_kb(knbr, &state) {
                state.lang_id = (state.lang_id + 1) % 2;
            }
            else {
                print!("{}", key);
            }
            if state.active_key.has(knbr) == false {
                state.active_key.push(knbr);
            }
        }
        None => {
            if knbr == 91 {
                state.lcmd = true;
            } else if knbr == 92 {
                state.rcmd = true;
            } else if knbr < 128 {
                println!("nbr: {}", knbr);
            }
            state.active_key.push(knbr);
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
