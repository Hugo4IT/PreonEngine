namespace Preon.Events;

public enum PreonKeyCode : uint
{
    Key1 = 0,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,

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
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    PrintScreen,
    ScrollLock,
    PauseBreak,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    Backspace,
    Return,
    Space,

    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

public static class PreonKeyCodeMethods
{
    internal static List<char?> CharMapping = new List<char?>(163)
    {
        '1',        // [(int)PreonKeyCode.Key1]
        '2',        // [(int)PreonKeyCode.Key2]
        '3',        // [(int)PreonKeyCode.Key3]
        '4',        // [(int)PreonKeyCode.Key4]
        '5',        // [(int)PreonKeyCode.Key5]
        '6',        // [(int)PreonKeyCode.Key6]
        '7',        // [(int)PreonKeyCode.Key7]
        '8',        // [(int)PreonKeyCode.Key8]
        '9',        // [(int)PreonKeyCode.Key9]
        '0',        // [(int)PreonKeyCode.Key0]
        'a',        // [(int)PreonKeyCode.A]
        'b',        // [(int)PreonKeyCode.B]
        'c',        // [(int)PreonKeyCode.C]
        'd',        // [(int)PreonKeyCode.D]
        'e',        // [(int)PreonKeyCode.E]
        'f',        // [(int)PreonKeyCode.F]
        'g',        // [(int)PreonKeyCode.G]
        'h',        // [(int)PreonKeyCode.H]
        'i',        // [(int)PreonKeyCode.I]
        'j',        // [(int)PreonKeyCode.J]
        'k',        // [(int)PreonKeyCode.K]
        'l',        // [(int)PreonKeyCode.L]
        'm',        // [(int)PreonKeyCode.M]
        'n',        // [(int)PreonKeyCode.N]
        'o',        // [(int)PreonKeyCode.O]
        'p',        // [(int)PreonKeyCode.P]
        'q',        // [(int)PreonKeyCode.Q]
        'r',        // [(int)PreonKeyCode.R]
        's',        // [(int)PreonKeyCode.S]
        't',        // [(int)PreonKeyCode.T]
        'u',        // [(int)PreonKeyCode.U]
        'v',        // [(int)PreonKeyCode.V]
        'w',        // [(int)PreonKeyCode.W]
        'x',        // [(int)PreonKeyCode.X]
        'y',        // [(int)PreonKeyCode.Y]
        'z',        // [(int)PreonKeyCode.Z]
        null,       // [(int)PreonKeyCode.Escape]
        null,       // [(int)PreonKeyCode.F1]
        null,       // [(int)PreonKeyCode.F2]
        null,       // [(int)PreonKeyCode.F3]
        null,       // [(int)PreonKeyCode.F4]
        null,       // [(int)PreonKeyCode.F5]
        null,       // [(int)PreonKeyCode.F6]
        null,       // [(int)PreonKeyCode.F7]
        null,       // [(int)PreonKeyCode.F8]
        null,       // [(int)PreonKeyCode.F9]
        null,       // [(int)PreonKeyCode.F10]
        null,       // [(int)PreonKeyCode.F11]
        null,       // [(int)PreonKeyCode.F12]
        null,       // [(int)PreonKeyCode.F13]
        null,       // [(int)PreonKeyCode.F14]
        null,       // [(int)PreonKeyCode.F15]
        null,       // [(int)PreonKeyCode.F16]
        null,       // [(int)PreonKeyCode.F17]
        null,       // [(int)PreonKeyCode.F18]
        null,       // [(int)PreonKeyCode.F19]
        null,       // [(int)PreonKeyCode.F20]
        null,       // [(int)PreonKeyCode.F21]
        null,       // [(int)PreonKeyCode.F22]
        null,       // [(int)PreonKeyCode.F23]
        null,       // [(int)PreonKeyCode.F24]
        null,       // [(int)PreonKeyCode.PrintScreen]
        null,       // [(int)PreonKeyCode.ScrollLock]
        null,       // [(int)PreonKeyCode.PauseBreak]
        null,       // [(int)PreonKeyCode.Insert]
        null,       // [(int)PreonKeyCode.Home]
        null,       // [(int)PreonKeyCode.Delete]
        null,       // [(int)PreonKeyCode.End]
        null,       // [(int)PreonKeyCode.PageDown]
        null,       // [(int)PreonKeyCode.PageUp]
        null,       // [(int)PreonKeyCode.Left]
        null,       // [(int)PreonKeyCode.Up]
        null,       // [(int)PreonKeyCode.Right]
        null,       // [(int)PreonKeyCode.Down]
        null,       // [(int)PreonKeyCode.Backspace]
        '\n',       // [(int)PreonKeyCode.Return]
        ' ',        // [(int)PreonKeyCode.Space]
        null,       // [(int)PreonKeyCode.Compose]
        null,       // [(int)PreonKeyCode.Caret]
        null,       // [(int)PreonKeyCode.Numlock]
        null,       // [(int)PreonKeyCode.Numpad0]
        null,       // [(int)PreonKeyCode.Numpad1]
        null,       // [(int)PreonKeyCode.Numpad2]
        null,       // [(int)PreonKeyCode.Numpad3]
        null,       // [(int)PreonKeyCode.Numpad4]
        null,       // [(int)PreonKeyCode.Numpad5]
        null,       // [(int)PreonKeyCode.Numpad6]
        null,       // [(int)PreonKeyCode.Numpad7]
        null,       // [(int)PreonKeyCode.Numpad8]
        null,       // [(int)PreonKeyCode.Numpad9]
        null,       // [(int)PreonKeyCode.NumpadAdd]
        null,       // [(int)PreonKeyCode.NumpadDivide]
        null,       // [(int)PreonKeyCode.NumpadDecimal]
        null,       // [(int)PreonKeyCode.NumpadComma]
        null,       // [(int)PreonKeyCode.NumpadEnter]
        null,       // [(int)PreonKeyCode.NumpadEquals]
        null,       // [(int)PreonKeyCode.NumpadMultiply]
        null,       // [(int)PreonKeyCode.NumpadSubtract]
        null,       // [(int)PreonKeyCode.AbntC1]
        null,       // [(int)PreonKeyCode.AbntC2]
        '\'',       // [(int)PreonKeyCode.Apostrophe]
        null,       // [(int)PreonKeyCode.Apps]
        '*',        // [(int)PreonKeyCode.Asterisk]
        null,       // [(int)PreonKeyCode.At]
        null,       // [(int)PreonKeyCode.Ax]
        '\\',       // [(int)PreonKeyCode.Backslash]
        null,       // [(int)PreonKeyCode.Calculator]
        null,       // [(int)PreonKeyCode.Capital]
        ':',        // [(int)PreonKeyCode.Colon]
        ',',        // [(int)PreonKeyCode.Comma]
        null,       // [(int)PreonKeyCode.Convert]
        '=',        // [(int)PreonKeyCode.Equals]
        null,       // [(int)PreonKeyCode.Grave]
        null,       // [(int)PreonKeyCode.Kana]
        null,       // [(int)PreonKeyCode.Kanji]
        null,       // [(int)PreonKeyCode.LAlt]
        '[',        // [(int)PreonKeyCode.LBracket]
        null,       // [(int)PreonKeyCode.LControl]
        null,       // [(int)PreonKeyCode.LShift]
        null,       // [(int)PreonKeyCode.LWin]
        null,       // [(int)PreonKeyCode.Mail]
        null,       // [(int)PreonKeyCode.MediaSelect]
        null,       // [(int)PreonKeyCode.MediaStop]
        '-',        // [(int)PreonKeyCode.Minus]
        null,       // [(int)PreonKeyCode.Mute]
        null,       // [(int)PreonKeyCode.MyComputer]
        null,       // [(int)PreonKeyCode.NavigateForward]
        null,       // [(int)PreonKeyCode.NavigateBackward]
        null,       // [(int)PreonKeyCode.NextTrack]
        null,       // [(int)PreonKeyCode.NoConvert]
        null,       // [(int)PreonKeyCode.OEM102]
        '.',        // [(int)PreonKeyCode.Period]
        null,       // [(int)PreonKeyCode.PlayPause]
        '+',        // [(int)PreonKeyCode.Plus]
        null,       // [(int)PreonKeyCode.Power]
        null,       // [(int)PreonKeyCode.PrevTrack]
        null,       // [(int)PreonKeyCode.RAlt]
        ']',        // [(int)PreonKeyCode.RBracket]
        null,       // [(int)PreonKeyCode.RControl]
        null,       // [(int)PreonKeyCode.RShift]
        null,       // [(int)PreonKeyCode.RWin]
        ';',        // [(int)PreonKeyCode.Semicolon]
        '/',        // [(int)PreonKeyCode.Slash]
        null,       // [(int)PreonKeyCode.Sleep]
        null,       // [(int)PreonKeyCode.Stop]
        null,       // [(int)PreonKeyCode.Sysrq]
        '\t',       // [(int)PreonKeyCode.Tab]
        '_',        // [(int)PreonKeyCode.Underline]
        null,       // [(int)PreonKeyCode.Unlabeled]
        null,       // [(int)PreonKeyCode.VolumeDown]
        null,       // [(int)PreonKeyCode.VolumeUp]
        null,       // [(int)PreonKeyCode.Wake]
        null,       // [(int)PreonKeyCode.WebBack]
        null,       // [(int)PreonKeyCode.WebFavorites]
        null,       // [(int)PreonKeyCode.WebForward]
        null,       // [(int)PreonKeyCode.WebHome]
        null,       // [(int)PreonKeyCode.WebRefresh]
        null,       // [(int)PreonKeyCode.WebSearch]
        null,       // [(int)PreonKeyCode.WebStop]
        null,       // [(int)PreonKeyCode.Yen]
        null,       // [(int)PreonKeyCode.Copy]
        null,       // [(int)PreonKeyCode.Paste]
        null        // [(int)PreonKeyCode.Cut]
    };

    public static bool TryGetChar(this PreonKeyCode key, out char? ch)
    {
        ch = CharMapping[(int)key];

        return ch != null;
    }
}