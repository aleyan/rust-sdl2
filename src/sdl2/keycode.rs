use std::hash::{mod, Hash};

#[derive(PartialEq, Eq, FromPrimitive, Show, Copy)]
pub enum KeyCode {
    Unknown            = 0,
    Backspace          = 8,
    Tab                = 9,
    Return             = 13,
    Escape             = 27,
    Space              = 32,
    Exclaim            = 33,
    Quotedbl           = 34,
    Hash               = 35,
    Dollar             = 36,
    Percent            = 37,
    Ampersand          = 38,
    Quote              = 39,
    LeftParen          = 40,
    RightParen         = 41,
    Asterisk           = 42,
    Plus               = 43,
    Comma              = 44,
    Minus              = 45,
    Period             = 46,
    Slash              = 47,
    Num0               = 48,
    Num1               = 49,
    Num2               = 50,
    Num3               = 51,
    Num4               = 52,
    Num5               = 53,
    Num6               = 54,
    Num7               = 55,
    Num8               = 56,
    Num9               = 57,
    Colon              = 58,
    Semicolon          = 59,
    Less               = 60,
    Equals             = 61,
    Greater            = 62,
    Question           = 63,
    At                 = 64,
    LeftBracket        = 91,
    Backslash          = 92,
    RightBracket       = 93,
    Caret              = 94,
    Underscore         = 95,
    Backquote          = 96,
    A                  = 97,
    B                  = 98,
    C                  = 99,
    D                  = 100,
    E                  = 101,
    F                  = 102,
    G                  = 103,
    H                  = 104,
    I                  = 105,
    J                  = 106,
    K                  = 107,
    L                  = 108,
    M                  = 109,
    N                  = 110,
    O                  = 111,
    P                  = 112,
    Q                  = 113,
    R                  = 114,
    S                  = 115,
    T                  = 116,
    U                  = 117,
    V                  = 118,
    W                  = 119,
    X                  = 120,
    Y                  = 121,
    Z                  = 122,
    Delete             = 127,
    CapsLock           = 1073741881,
    F1                 = 1073741882,
    F2                 = 1073741883,
    F3                 = 1073741884,
    F4                 = 1073741885,
    F5                 = 1073741886,
    F6                 = 1073741887,
    F7                 = 1073741888,
    F8                 = 1073741889,
    F9                 = 1073741890,
    F10                = 1073741891,
    F11                = 1073741892,
    F12                = 1073741893,
    PrintScreen        = 1073741894,
    ScrollLock         = 1073741895,
    Pause              = 1073741896,
    Insert             = 1073741897,
    Home               = 1073741898,
    PageUp             = 1073741899,
    End                = 1073741901,
    PageDown           = 1073741902,
    Right              = 1073741903,
    Left               = 1073741904,
    Down               = 1073741905,
    Up                 = 1073741906,
    NumLockClear       = 1073741907,
    KpDivide           = 1073741908,
    KpMultiply         = 1073741909,
    KpMinus            = 1073741910,
    KpPlus             = 1073741911,
    KpEnter            = 1073741912,
    Kp1                = 1073741913,
    Kp2                = 1073741914,
    Kp3                = 1073741915,
    Kp4                = 1073741916,
    Kp5                = 1073741917,
    Kp6                = 1073741918,
    Kp7                = 1073741919,
    Kp8                = 1073741920,
    Kp9                = 1073741921,
    Kp0                = 1073741922,
    KpPeriod           = 1073741923,
    Application        = 1073741925,
    Power              = 1073741926,
    KpEquals           = 1073741927,
    F13                = 1073741928,
    F14                = 1073741929,
    F15                = 1073741930,
    F16                = 1073741931,
    F17                = 1073741932,
    F18                = 1073741933,
    F19                = 1073741934,
    F20                = 1073741935,
    F21                = 1073741936,
    F22                = 1073741937,
    F23                = 1073741938,
    F24                = 1073741939,
    Execute            = 1073741940,
    Help               = 1073741941,
    Menu               = 1073741942,
    Select             = 1073741943,
    Stop               = 1073741944,
    Again              = 1073741945,
    Undo               = 1073741946,
    Cut                = 1073741947,
    Copy               = 1073741948,
    Paste              = 1073741949,
    Find               = 1073741950,
    Mute               = 1073741951,
    VolumeUp           = 1073741952,
    VolumeDown         = 1073741953,
    KpComma            = 1073741957,
    KpEqualsAS400      = 1073741958,
    AltErase           = 1073741977,
    Sysreq             = 1073741978,
    Cancel             = 1073741979,
    Clear              = 1073741980,
    Prior              = 1073741981,
    Return2            = 1073741982,
    Separator          = 1073741983,
    Out                = 1073741984,
    Oper               = 1073741985,
    ClearAgain         = 1073741986,
    CrSel              = 1073741987,
    ExSel              = 1073741988,
    Kp00               = 1073742000,
    Kp000              = 1073742001,
    ThousandsSeparator = 1073742002,
    DecimalSeparator   = 1073742003,
    CurrencyUnit       = 1073742004,
    CurrencySubUnit    = 1073742005,
    KpLeftParen        = 1073742006,
    KpRightParen       = 1073742007,
    KpLeftBrace        = 1073742008,
    KpRightBrace       = 1073742009,
    KpTab              = 1073742010,
    KpBackspace        = 1073742011,
    KpA                = 1073742012,
    KpB                = 1073742013,
    KpC                = 1073742014,
    KpD                = 1073742015,
    KpE                = 1073742016,
    KpF                = 1073742017,
    KpXor              = 1073742018,
    KpPower            = 1073742019,
    KpPercent          = 1073742020,
    KpLess             = 1073742021,
    KpGreater          = 1073742022,
    KpAmpersand        = 1073742023,
    KpDblAmpersand     = 1073742024,
    KpVerticalBar      = 1073742025,
    KpDblVerticalBar   = 1073742026,
    KpColon            = 1073742027,
    KpHash             = 1073742028,
    KpSpace            = 1073742029,
    KpAt               = 1073742030,
    KpExclam           = 1073742031,
    KpMemStore         = 1073742032,
    KpMemRecall        = 1073742033,
    KpMemClear         = 1073742034,
    KpMemAdd           = 1073742035,
    KpMemSubtract      = 1073742036,
    KpMemMultiply      = 1073742037,
    KpMemDivide        = 1073742038,
    KpPlusMinus        = 1073742039,
    KpCear             = 1073742040,
    KpClearEntry       = 1073742041,
    KpBinary           = 1073742042,
    KpOctal            = 1073742043,
    KpDecimal          = 1073742044,
    KpHexadecimal      = 1073742045,
    LCtrl              = 1073742048,
    LShift             = 1073742049,
    LAlt               = 1073742050,
    LGui               = 1073742051,
    RCtrl              = 1073742052,
    RShift             = 1073742053,
    RAlt               = 1073742054,
    RGui               = 1073742055,
    Mode               = 1073742081,
    AudioNext          = 1073742082,
    AudioPrev          = 1073742083,
    AudioStop          = 1073742084,
    AudioPlay          = 1073742085,
    AudioMute          = 1073742086,
    MediaSelect        = 1073742087,
    Www                = 1073742088,
    Mail               = 1073742089,
    Calculator         = 1073742090,
    Computer           = 1073742091,
    AcSearch           = 1073742092,
    AcHome             = 1073742093,
    AcBack             = 1073742094,
    AcForward          = 1073742095,
    AcStop             = 1073742096,
    AcRefresh          = 1073742097,
    AcBookmarks        = 1073742098,
    BrightnessDown     = 1073742099,
    BrightnessUp       = 1073742100,
    DisplaySwitch      = 1073742101,
    KbdIllumToggle     = 1073742102,
    KbdIllumDown       = 1073742103,
    KbdIllumUp         = 1073742104,
    Eject              = 1073742105,
    Sleep              = 1073742106,
}

impl<S: hash::Writer> Hash<S> for KeyCode {
    #[inline]
    fn hash(&self, state: &mut S) {
        (*self as i32).hash(state);
    }
}

impl ToPrimitive for KeyCode {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        Some(*self as i64)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }

    #[inline]
    fn to_int(&self) -> Option<int> {
        Some(*self as int)
    }
}
