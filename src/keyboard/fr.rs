pub const KMAP: [&str; 69] = [
    "NULL",
    "escape",
    "&",
    "é",
    "\"",
    "'",
    "(",
    "§",
    "è",
    "!",
    "ç",
    "à",
    ")",
    "-",
    "DEL",
    "\t",
    "a",
    "z",
    "e",
    "r",
    "t",
    "y",
    "u",
    "i",
    "o",
    "p",
    "^",
    "$",
    "\n",
    "CTRL",
    "q",
    "s",
    "d",
    "f",
    "g",
    "h",
    "j",
    "k",
    "l",
    "m",
    "ù",
    "<",
    "SHIFT",
    "<",
    "w",
    "x",
    "c",
    "v",
    "b",
    "n",
    ",",
    ";",
    ":",
    "=",
    "RSHIFT",
    "",
    "OPTION",
    " ",
    "SHIFT_LOCK",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
];

pub const KMAP_SHIFT: [&str; 69] = [
    "NULL",
    "escape",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "°",
    "_",
    "DEL",
    "TAB",
    "A",
    "Z",
    "E",
    "R",
    "T",
    "Y",
    "U",
    "I",
    "O",
    "P",
    "¨",
    "*",
    "\n",
    "CTRL",
    "Q",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    "M",
    "%",
    "£",
    "SHIFT",
    ">",
    "W",
    "X",
    "C",
    "V",
    "B",
    "N",
    "?",
    ".",
    "/",
    "+",
    "RSHIFT",
    "",
    "",
    " ",
    "SHIFT_LOCK",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
];

#[repr(u8)]
pub enum Kvalue {
    Null,
    Esc,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _0,
    Symbole0,
    Symbole1,
    Del,
    Tab,
    A,
    Z,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    Symbole2,
    Symbole3,
    Enter,
    Ctrl,
    Q,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    Symbole4,
    Symbole5,
    LShift,
    Symbole6,
    W,
    X,
    C,
    V,
    B,
    N,
    Symbole7,
    Symbole8,
    Symbole9,
    Symbole10,
    Rshift,
    Symbole11,
    Option,
    Space,
    ShiftLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
}

pub fn get_kvalue(knbr: u8) -> Kvalue {
    match knbr {
        1 => { Kvalue::Esc }
        2 => { Kvalue::_1 }
        3 => { Kvalue::_2 }
        4 => { Kvalue::_3 }
        5 => { Kvalue::_4 }
        6 => { Kvalue::_5 }
        7 => { Kvalue::_6 }
        8 => { Kvalue::_7 }
        9 => { Kvalue::_8 }
        10 => { Kvalue::_9 }
        11 => { Kvalue::_0 }
        12 => { Kvalue::Symbole0 }
        13 => { Kvalue::Symbole1 }
        14 => { Kvalue::Del }
        15 => { Kvalue::Tab }
        16 => { Kvalue::A }
        17 => { Kvalue::Z }
        18 => { Kvalue::E }
        19 => { Kvalue::R }
        20 => { Kvalue::T }
        21 => { Kvalue::Y }
        22 => { Kvalue::U }
        23 => { Kvalue::I }
        24 => { Kvalue::O }
        25 => { Kvalue::P }
        26 => { Kvalue::Symbole2 }
        27 => { Kvalue::Symbole3 }
        28 => { Kvalue::Enter }
        29 => { Kvalue::Ctrl }
        30 => { Kvalue::Q }
        31 => { Kvalue::S }
        32 => { Kvalue::D }
        33 => { Kvalue::F }
        34 => { Kvalue::G }
        35 => { Kvalue::H }
        36 => { Kvalue::J }
        37 => { Kvalue::K }
        38 => { Kvalue::L }
        39 => { Kvalue::M }
        40 => { Kvalue::Symbole4 }
        41 => { Kvalue::Symbole5 }
        42 => { Kvalue::LShift }
        43 => { Kvalue::Symbole6 }
        44 => { Kvalue::W }
        45 => { Kvalue::X }
        46 => { Kvalue::C }
        47 => { Kvalue::V }
        48 => { Kvalue::B }
        49 => { Kvalue::N }
        50 => { Kvalue::Symbole7 }
        51 => { Kvalue::Symbole8 }
        52 => { Kvalue::Symbole9 }
        53 => { Kvalue::Symbole10 }
        54 => { Kvalue::Rshift }
        55 => { Kvalue::Symbole11 }
        56 => { Kvalue::Option }
        57 => { Kvalue::Space }
        58 => { Kvalue::ShiftLock }
        59 => { Kvalue::F1 }
        60 => { Kvalue::F2 }
        61 => { Kvalue::F3 }
        62 => { Kvalue::F4 }
        63 => { Kvalue::F5 }
        64 => { Kvalue::F6 }
        65 => { Kvalue::F7 }
        66 => { Kvalue::F8 }
        67 => { Kvalue::F9 }
        68 => { Kvalue::F10 }
        _ => { Kvalue::Null}
    }
}