pub mod en;
pub mod fr;

use crate::asm;
use crate::shell;


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

pub struct Kmap {
    map: [&'static str; 69],
    map_shift: [&'static str; 69]
}

pub struct KeyboardState {
    lshift: bool,
    rshift: bool,
    lctrl: bool,
    loption: bool,
    lcmd: bool,
    rcmd: bool,
    _fn_btn: bool,
    caps_lock: bool,
    lang_id: usize,
    pressed_key: Stack,
    active_key: Stack
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            lshift: false,
            rshift: false,
            lctrl: false,
            loption: false,
            lcmd: false,
            rcmd: false,
            _fn_btn: false,
            caps_lock: false,
            lang_id: 1,
            pressed_key: Stack::default(),
            active_key: Stack::default()
        }
    }
}

use once_cell::unsync::Lazy;
use spin::{Mutex, MutexGuard};

static KEYBOARD_STATE: Mutex<Lazy<KeyboardState>> = Mutex::new(Lazy::new(|| KeyboardState::default()));

const LANG: [Kmap; 2] = [
    Kmap {
        map: fr::KMAP, map_shift: fr::KMAP_SHIFT
    },
    Kmap {
        map: en::KMAP, map_shift: en::KMAP_SHIFT
    }
];

fn is_switch_kb(knbr: u8, kb_state: &MutexGuard<'_, once_cell::unsync::Lazy<KeyboardState>>) -> bool {
    if knbr == fr::Kvalue::Space as u8 && kb_state.loption && kb_state.active_key.data[1] == 0 {
        return true;
    }
    return false;
}

fn is_printable(value: fr::Kvalue) -> bool {
    match value {
        fr::Kvalue::Null => { false }
        fr::Kvalue::Esc => { false }
        fr::Kvalue::Ctrl => { false }
        fr::Kvalue::LShift => { false }
        fr::Kvalue::Rshift => { false }
        fr::Kvalue::Option => { false }
        fr::Kvalue::ShiftLock => { false }
        fr::Kvalue::F1 => { false }
        fr::Kvalue::F2 => { false }
        fr::Kvalue::F3 => { false }
        fr::Kvalue::F4 => { false }
        fr::Kvalue::F5 => { false }
        fr::Kvalue::F6 => { false }
        fr::Kvalue::F7 => { false }
        fr::Kvalue::F8 => { false }
        fr::Kvalue::F9 => { false }
        fr::Kvalue::F10 => { false }
        _ => { true }
    }
}

fn key_unpress(knbr: u8) {
    let unpressed_knbr = knbr - 128;
    let mut state = KEYBOARD_STATE.lock();

    match LANG[state.lang_id].map.get(unpressed_knbr as usize) {
        Some(_key) => {
            let kvalue = fr::get_kvalue(unpressed_knbr);
            match kvalue {
                fr::Kvalue::Ctrl => { state.lctrl = false; }
                fr::Kvalue::LShift => { state.lshift = false; }
                fr::Kvalue::Rshift => { state.rshift = false; }
                fr::Kvalue::Option => { state.loption = false; }
                _ => { state.pressed_key.push(knbr); }
            }
            state.active_key.pop();
        }
        None => {
            if unpressed_knbr == 91 {
                state.lcmd = false;
            } else if unpressed_knbr == 92 {
                state.rcmd = false;
            }
            state.active_key.pop();
        }
    }
}

fn key_press(knbr: u8) {
    let mut state = KEYBOARD_STATE.lock();
    
    let mut map = &LANG[state.lang_id].map;
    if state.lshift == true || state.rshift == true || state.caps_lock == true {
        map = &LANG[state.lang_id].map_shift;
    }
    match (*map).get(knbr as usize) {
        Some(key) => {
            let kvalue = fr::get_kvalue(knbr);
            match kvalue {
                fr::Kvalue::Ctrl => {
                    state.lctrl = true;
                }
                fr::Kvalue::LShift => {
                    state.lshift = true;
                }
                fr::Kvalue::Rshift => {
                    state.rshift = true;
                }
                fr::Kvalue::Option => {
                    // loption and roption have the same value in macos keyboard
                    state.loption = true;
                }
                fr::Kvalue::ShiftLock => {
                    state.caps_lock = !state.caps_lock;
                }
                fr::Kvalue::Del => {
                    shell::read(&[127]);
                }
                _ => {
                    if is_switch_kb(knbr, &state) {
                        state.lang_id = (state.lang_id + 1) % 2;
                    } else {
                        if is_printable(kvalue) {
                            shell::read(key.as_bytes());
                        }
                    }
                }
            }
            if !state.active_key.has(knbr) {
                state.active_key.push(knbr);
            }
        }
        None => {
            if knbr == 91 {
                state.lcmd = true;
            } else if knbr == 92 {
                state.rcmd = true;
            }
            state.active_key.push(knbr);
        }
    }
}

pub fn handle_keypress() {
    let knbr = asm::inb(0x60);
    if knbr > 127 {
        key_unpress(knbr);
    } else {
        key_press(knbr);
    }
}
