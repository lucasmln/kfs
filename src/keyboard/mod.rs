pub mod en;
pub mod fr;

use crate::io::inb;
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
    _fn_btn: bool,
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
            _fn_btn: false,
            caps_lock: false,
            lang: [
                Kmap {
                    map: &fr::KMAP, map_shift: &fr::KMAP_SHIFT
                },
                Kmap {
                    map: &en::KMAP, map_shift: &en::KMAP_SHIFT
                }
            ],
            lang_id: 1,
            pressed_key: Stack::default(),
            active_key: Stack::default()
        }
    }
}

static mut KEYBOARD_STATE: Option<KeyboardState<'static>> = None;

fn is_switch_kb(knbr: u8, kb_state: &KeyboardState<'_>) -> bool {
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
    let mut state = unsafe { KEYBOARD_STATE.as_mut().unwrap() };

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
            }
            state.active_key.pop();
        }
    }
}

fn key_press(knbr: u8) {
    let mut state = unsafe { KEYBOARD_STATE.as_mut().unwrap() };
    let map;
    
    if state.lshift == true || state.rshift == true || state.caps_lock == true {
        map = state.lang[state.lang_id].map_shift;
    } else {
        map = state.lang[state.lang_id].map;
    }
    match map.get(knbr as usize) {
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
                    state.caps_lock = true;
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

pub fn init() {
    unsafe {
        KEYBOARD_STATE = Some(KeyboardState::default());
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
