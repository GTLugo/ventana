use {
  super::{
    ThreadPtr,
    XKBH,
    XKBXH,
    context::XkbContext,
  },
  std::{
    ffi::c_char,
    ops::Deref,
  },
  ventana_hal::{
    input::key::{
      LogicalKey,
      NativeCode,
      NativeKey,
      PhysicalKey,
    },
    keyboard::{
      Code,
      Location,
      NamedKey,
    },
  },
  xkbcommon_dl::{
    self as xkb,
    XKB_MOD_INVALID,
    x11::xcb_connection_t,
    xkb_keycode_t,
    xkb_keymap,
    xkb_keymap_compile_flags,
    xkb_keysym_t,
    xkb_layout_index_t,
    xkb_mod_index_t,
  },
};

/// Map the raw X11-style keycode to the `KeyCode` enum.
///
/// X11-style keycodes are offset by 8 from the keycodes the Linux kernel uses.
pub fn raw_keycode_to_physicalkey(keycode: u32) -> PhysicalKey {
  scancode_to_physicalkey(keycode.saturating_sub(8))
}

/// Map the linux scancode to Keycode.
///
/// Both X11 and Wayland use keys with `+ 8` offset to linux scancode.
pub fn scancode_to_physicalkey(scancode: u32) -> PhysicalKey {
  // The keycode values are taken from linux/include/uapi/linux/input-event-codes.h, as
  // libxkbcommon's documentation seems to suggest that the keycode values we're interested in
  // are defined by the Linux kernel. If Winit programs end up being run on other Unix-likes,
  // I can only hope they agree on what the keycodes mean.
  //
  // The mapping here is heavily influenced by Firefox' and Chromium's sources:
  // - https://searchfox.org/mozilla-central/rev/c597e9c789ad36af84a0370d395be066b7dc94f4/widget/NativeKeyToDOMCodeName.h
  // - https://chromium.googlesource.com/chromium/src.git/+/3e1a26c44c024d97dc9a4c09bbc6a2365398ca2c/ui/events/keycodes/dom/dom_code_data.inc
  //
  // Some of the keycodes are likely superfluous for our purposes, and some are ones which are
  // difficult to test the correctness of, or discover the purpose of. Because of this, they've
  // either been commented out here, or not included at all.
  PhysicalKey::Code(match scancode {
    0 => return PhysicalKey::Unidentified(NativeCode::Code(0)),
    1 => Code::Escape,
    2 => Code::Digit1,
    3 => Code::Digit2,
    4 => Code::Digit3,
    5 => Code::Digit4,
    6 => Code::Digit5,
    7 => Code::Digit6,
    8 => Code::Digit7,
    9 => Code::Digit8,
    10 => Code::Digit9,
    11 => Code::Digit0,
    12 => Code::Minus,
    13 => Code::Equal,
    14 => Code::Backspace,
    15 => Code::Tab,
    16 => Code::KeyQ,
    17 => Code::KeyW,
    18 => Code::KeyE,
    19 => Code::KeyR,
    20 => Code::KeyT,
    21 => Code::KeyY,
    22 => Code::KeyU,
    23 => Code::KeyI,
    24 => Code::KeyO,
    25 => Code::KeyP,
    26 => Code::BracketLeft,
    27 => Code::BracketRight,
    28 => Code::Enter,
    29 => Code::ControlLeft,
    30 => Code::KeyA,
    31 => Code::KeyS,
    32 => Code::KeyD,
    33 => Code::KeyF,
    34 => Code::KeyG,
    35 => Code::KeyH,
    36 => Code::KeyJ,
    37 => Code::KeyK,
    38 => Code::KeyL,
    39 => Code::Semicolon,
    40 => Code::Quote,
    41 => Code::Backquote,
    42 => Code::ShiftLeft,
    43 => Code::Backslash,
    44 => Code::KeyZ,
    45 => Code::KeyX,
    46 => Code::KeyC,
    47 => Code::KeyV,
    48 => Code::KeyB,
    49 => Code::KeyN,
    50 => Code::KeyM,
    51 => Code::Comma,
    52 => Code::Period,
    53 => Code::Slash,
    54 => Code::ShiftRight,
    55 => Code::NumpadMultiply,
    56 => Code::AltLeft,
    57 => Code::Space,
    58 => Code::CapsLock,
    59 => Code::F1,
    60 => Code::F2,
    61 => Code::F3,
    62 => Code::F4,
    63 => Code::F5,
    64 => Code::F6,
    65 => Code::F7,
    66 => Code::F8,
    67 => Code::F9,
    68 => Code::F10,
    69 => Code::NumLock,
    70 => Code::ScrollLock,
    71 => Code::Numpad7,
    72 => Code::Numpad8,
    73 => Code::Numpad9,
    74 => Code::NumpadSubtract,
    75 => Code::Numpad4,
    76 => Code::Numpad5,
    77 => Code::Numpad6,
    78 => Code::NumpadAdd,
    79 => Code::Numpad1,
    80 => Code::Numpad2,
    81 => Code::Numpad3,
    82 => Code::Numpad0,
    83 => Code::NumpadDecimal,
    85 => Code::Lang5,
    86 => Code::IntlBackslash,
    87 => Code::F11,
    88 => Code::F12,
    89 => Code::IntlRo,
    90 => Code::Lang3,
    91 => Code::Lang4,
    92 => Code::Convert,
    93 => Code::KanaMode,
    94 => Code::NonConvert,
    // 95 => Code::KPJPCOMMA,
    96 => Code::NumpadEnter,
    97 => Code::ControlRight,
    98 => Code::NumpadDivide,
    99 => Code::PrintScreen,
    100 => Code::AltRight,
    // 101 => Code::LINEFEED,
    102 => Code::Home,
    103 => Code::ArrowUp,
    104 => Code::PageUp,
    105 => Code::ArrowLeft,
    106 => Code::ArrowRight,
    107 => Code::End,
    108 => Code::ArrowDown,
    109 => Code::PageDown,
    110 => Code::Insert,
    111 => Code::Delete,
    // 112 => Code::MACRO,
    113 => Code::AudioVolumeMute,
    114 => Code::AudioVolumeDown,
    115 => Code::AudioVolumeUp,
    116 => Code::Power,
    117 => Code::NumpadEqual,
    // 118 => Code::KPPLUSMINUS,
    119 => Code::Pause,
    120 => Code::ShowAllWindows,
    121 => Code::NumpadComma,
    122 => Code::Lang1,
    123 => Code::Lang2,
    124 => Code::IntlYen,
    125 => Code::MetaLeft,
    126 => Code::MetaRight,
    127 => Code::ContextMenu,
    128 => Code::BrowserStop,
    129 => Code::Again,
    130 => Code::Props,
    131 => Code::Undo,
    132 => Code::Select, // FRONT
    133 => Code::Copy,
    134 => Code::Open,
    135 => Code::Paste,
    136 => Code::Find,
    137 => Code::Cut,
    138 => Code::Help,
    // 139 => Code::MENU,
    140 => Code::LaunchApp2, // CALC
    // 141 => Code::SETUP,
    142 => Code::Sleep,
    143 => Code::WakeUp,
    144 => Code::LaunchApp1, // FILE
    // 145 => Code::SENDFILE,
    // 146 => Code::DELETEFILE,
    // 147 => Code::XFER,
    // 148 => Code::PROG1,
    // 149 => Code::PROG2,
    // 150 => Code::WWW,
    // 151 => Code::MSDOS,
    // 152 => Code::COFFEE,
    // 153 => Code::ROTATE_DISPLAY,
    // 154 => Code::CYCLEWINDOWS,
    155 => Code::LaunchMail,
    156 => Code::BrowserFavorites, // BOOKMARKS
    // 157 => Code::COMPUTER,
    158 => Code::BrowserBack,
    159 => Code::BrowserForward,
    // 160 => Code::CLOSECD,
    161 => Code::Eject, // EJECTCD
    // 162 => Code::EJECTCLOSECD,
    163 => Code::MediaTrackNext,
    164 => Code::MediaPlayPause,
    165 => Code::MediaTrackPrevious,
    166 => Code::MediaStop,
    167 => Code::MediaRecord,
    168 => Code::MediaRewind,
    // 169 => Code::PHONE,
    // 170 => Code::ISO,
    171 => Code::MediaSelect, // CONFIG
    172 => Code::BrowserHome,
    173 => Code::BrowserRefresh,
    // 174 => Code::EXIT,
    // 175 => Code::MOVE,
    // 176 => Code::EDIT,
    // 177 => Code::SCROLLUP,
    // 178 => Code::SCROLLDOWN,
    179 => Code::NumpadParenLeft,
    180 => Code::NumpadParenRight,
    // 181 => Code::NEW,
    // 182 => Code::REDO,
    183 => Code::F13,
    184 => Code::F14,
    185 => Code::F15,
    186 => Code::F16,
    187 => Code::F17,
    188 => Code::F18,
    189 => Code::F19,
    190 => Code::F20,
    191 => Code::F21,
    192 => Code::F22,
    193 => Code::F23,
    194 => Code::F24,
    // 200 => Code::PLAYCD,
    201 => Code::MediaPause,
    // 202 => Code::PROG3,
    // 203 => Code::PROG4,
    // 204 => Code::DASHBOARD,
    // 205 => Code::SUSPEND,
    // 206 => Code::CLOSE,
    207 => Code::MediaPlay,
    208 => Code::MediaFastForward,
    // 209 => Code::BASSBOOST,
    // 210 => Code::PRINT,
    // 211 => Code::HP,
    // 212 => Code::CAMERA,
    // 213 => Code::SOUND,
    // 214 => Code::QUESTION,
    // 215 => Code::EMAIL,
    // 216 => Code::CHAT,
    217 => Code::BrowserSearch,
    // 218 => Code::CONNECT,
    // 219 => Code::FINANCE,
    // 220 => Code::SPORT,
    // 221 => Code::SHOP,
    // 222 => Code::ALTERASE,
    // 223 => Code::CANCEL,
    224 => Code::BrightnessDown,
    225 => Code::BrightnessUp,
    // 226 => Code::MEDIA,
    227 => Code::DisplayToggleIntExt,
    228 => Code::KeyboardBacklightToggle,
    // 229 => Code::KBDILLUMDOWN,
    // 230 => Code::KBDILLUMUP,
    231 => Code::MailSend,
    232 => Code::MailReply,
    233 => Code::MailForward,
    // 234 => Code::SAVE,
    // 235 => Code::DOCUMENTS,
    // 236 => Code::BATTERY,
    // 237 => Code::BLUETOOTH,
    // 238 => Code::WLAN,
    // 239 => Code::UWB,
    240 => return PhysicalKey::Unidentified(NativeCode::Unidentified),
    // 241 => Code::VIDEO_NEXT,
    // 242 => Code::VIDEO_PREV,
    // 243 => Code::BRIGHTNESS_CYCLE,
    // 244 => Code::BRIGHTNESS_AUTO,
    // 245 => Code::DISPLAY_OFF,
    // 246 => Code::WWAN,
    // 247 => Code::RFKILL,
    248 => Code::MicrophoneMuteToggle,
    372 => Code::ZoomToggle,
    579 => Code::LaunchControlPanel,
    580 => Code::SelectTask,
    581 => Code::LaunchScreenSaver,
    583 => Code::LaunchAssistant,
    584 => Code::KeyboardLayoutSelect,
    633 => Code::PrivacyScreenToggle,
    _ => return PhysicalKey::Unidentified(NativeCode::Code(scancode)),
  })
}

pub fn physicalkey_to_scancode(key: PhysicalKey) -> Option<u32> {
  let code = match key {
    PhysicalKey::Code(code) => code,
    PhysicalKey::Unidentified(code) => {
      return match code {
        NativeCode::Unidentified => Some(240),
        NativeCode::Code(raw) => Some(raw),
        // _ => None,
      };
    },
  };

  match code {
    Code::Escape => Some(1),
    Code::Digit1 => Some(2),
    Code::Digit2 => Some(3),
    Code::Digit3 => Some(4),
    Code::Digit4 => Some(5),
    Code::Digit5 => Some(6),
    Code::Digit6 => Some(7),
    Code::Digit7 => Some(8),
    Code::Digit8 => Some(9),
    Code::Digit9 => Some(10),
    Code::Digit0 => Some(11),
    Code::Minus => Some(12),
    Code::Equal => Some(13),
    Code::Backspace => Some(14),
    Code::Tab => Some(15),
    Code::KeyQ => Some(16),
    Code::KeyW => Some(17),
    Code::KeyE => Some(18),
    Code::KeyR => Some(19),
    Code::KeyT => Some(20),
    Code::KeyY => Some(21),
    Code::KeyU => Some(22),
    Code::KeyI => Some(23),
    Code::KeyO => Some(24),
    Code::KeyP => Some(25),
    Code::BracketLeft => Some(26),
    Code::BracketRight => Some(27),
    Code::Enter => Some(28),
    Code::ControlLeft => Some(29),
    Code::KeyA => Some(30),
    Code::KeyS => Some(31),
    Code::KeyD => Some(32),
    Code::KeyF => Some(33),
    Code::KeyG => Some(34),
    Code::KeyH => Some(35),
    Code::KeyJ => Some(36),
    Code::KeyK => Some(37),
    Code::KeyL => Some(38),
    Code::Semicolon => Some(39),
    Code::Quote => Some(40),
    Code::Backquote => Some(41),
    Code::ShiftLeft => Some(42),
    Code::Backslash => Some(43),
    Code::KeyZ => Some(44),
    Code::KeyX => Some(45),
    Code::KeyC => Some(46),
    Code::KeyV => Some(47),
    Code::KeyB => Some(48),
    Code::KeyN => Some(49),
    Code::KeyM => Some(50),
    Code::Comma => Some(51),
    Code::Period => Some(52),
    Code::Slash => Some(53),
    Code::ShiftRight => Some(54),
    Code::NumpadMultiply => Some(55),
    Code::AltLeft => Some(56),
    Code::Space => Some(57),
    Code::CapsLock => Some(58),
    Code::F1 => Some(59),
    Code::F2 => Some(60),
    Code::F3 => Some(61),
    Code::F4 => Some(62),
    Code::F5 => Some(63),
    Code::F6 => Some(64),
    Code::F7 => Some(65),
    Code::F8 => Some(66),
    Code::F9 => Some(67),
    Code::F10 => Some(68),
    Code::NumLock => Some(69),
    Code::ScrollLock => Some(70),
    Code::Numpad7 => Some(71),
    Code::Numpad8 => Some(72),
    Code::Numpad9 => Some(73),
    Code::NumpadSubtract => Some(74),
    Code::Numpad4 => Some(75),
    Code::Numpad5 => Some(76),
    Code::Numpad6 => Some(77),
    Code::NumpadAdd => Some(78),
    Code::Numpad1 => Some(79),
    Code::Numpad2 => Some(80),
    Code::Numpad3 => Some(81),
    Code::Numpad0 => Some(82),
    Code::NumpadDecimal => Some(83),
    Code::Lang5 => Some(85),
    Code::IntlBackslash => Some(86),
    Code::F11 => Some(87),
    Code::F12 => Some(88),
    Code::IntlRo => Some(89),
    Code::Lang3 => Some(90),
    Code::Lang4 => Some(91),
    Code::Convert => Some(92),
    Code::KanaMode => Some(93),
    Code::NonConvert => Some(94),
    Code::NumpadEnter => Some(96),
    Code::ControlRight => Some(97),
    Code::NumpadDivide => Some(98),
    Code::PrintScreen => Some(99),
    Code::AltRight => Some(100),
    Code::Home => Some(102),
    Code::ArrowUp => Some(103),
    Code::PageUp => Some(104),
    Code::ArrowLeft => Some(105),
    Code::ArrowRight => Some(106),
    Code::End => Some(107),
    Code::ArrowDown => Some(108),
    Code::PageDown => Some(109),
    Code::Insert => Some(110),
    Code::Delete => Some(111),
    Code::AudioVolumeMute => Some(113),
    Code::AudioVolumeDown => Some(114),
    Code::AudioVolumeUp => Some(115),
    Code::Power => Some(116),
    Code::NumpadEqual => Some(117),
    Code::Pause => Some(119),
    Code::ShowAllWindows => Some(120),
    Code::NumpadComma => Some(121),
    Code::Lang1 => Some(122),
    Code::Lang2 => Some(123),
    Code::IntlYen => Some(124),
    Code::MetaLeft => Some(125),
    Code::MetaRight => Some(126),
    Code::ContextMenu => Some(127),
    Code::BrowserStop => Some(128),
    Code::Again => Some(129),
    Code::Props => Some(130),
    Code::Undo => Some(131),
    Code::Select => Some(132),
    Code::Copy => Some(133),
    Code::Open => Some(134),
    Code::Paste => Some(135),
    Code::Find => Some(136),
    Code::Cut => Some(137),
    Code::Help => Some(138),
    Code::LaunchApp2 => Some(140),
    Code::Sleep => Some(142),
    Code::WakeUp => Some(143),
    Code::LaunchApp1 => Some(144),
    Code::LaunchMail => Some(155),
    Code::BrowserFavorites => Some(156),
    Code::BrowserBack => Some(158),
    Code::BrowserForward => Some(159),
    Code::Eject => Some(161),
    Code::MediaTrackNext => Some(163),
    Code::MediaPlayPause => Some(164),
    Code::MediaTrackPrevious => Some(165),
    Code::MediaStop => Some(166),
    Code::MediaRecord => Some(167),
    Code::MediaRewind => Some(168),
    Code::MediaSelect => Some(171),
    Code::BrowserHome => Some(172),
    Code::BrowserRefresh => Some(173),
    Code::NumpadParenLeft => Some(179),
    Code::NumpadParenRight => Some(180),
    Code::F13 => Some(183),
    Code::F14 => Some(184),
    Code::F15 => Some(185),
    Code::F16 => Some(186),
    Code::F17 => Some(187),
    Code::F18 => Some(188),
    Code::F19 => Some(189),
    Code::F20 => Some(190),
    Code::F21 => Some(191),
    Code::F22 => Some(192),
    Code::F23 => Some(193),
    Code::F24 => Some(194),
    Code::MediaPause => Some(201),
    Code::MediaPlay => Some(207),
    Code::MediaFastForward => Some(208),
    Code::BrowserSearch => Some(217),
    Code::BrightnessDown => Some(224),
    Code::BrightnessUp => Some(225),
    Code::DisplayToggleIntExt => Some(227),
    Code::KeyboardBacklightToggle => Some(228),
    Code::MailSend => Some(231),
    Code::MailReply => Some(232),
    Code::MailForward => Some(233),
    // PhysicalKey::Unidentified(NativeCode::Unidentified) => Some(240),
    Code::MicrophoneMuteToggle => Some(248),
    Code::ZoomToggle => Some(372),
    Code::LaunchControlPanel => Some(579),
    Code::SelectTask => Some(580),
    Code::LaunchScreenSaver => Some(581),
    Code::LaunchAssistant => Some(583),
    Code::KeyboardLayoutSelect => Some(584),
    Code::PrivacyScreenToggle => Some(633),
    _ => None,
  }
}

pub fn keysym_to_key(keysym: u32) -> LogicalKey {
  use xkbcommon_dl::keysyms;
  LogicalKey::Named(match keysym {
    // TTY function keys
    keysyms::BackSpace => NamedKey::Backspace,
    keysyms::Tab => NamedKey::Tab,
    // keysyms::Linefeed => NamedKey::Linefeed,
    keysyms::Clear => NamedKey::Clear,
    keysyms::Return => NamedKey::Enter,
    keysyms::Pause => NamedKey::Pause,
    keysyms::Scroll_Lock => NamedKey::ScrollLock,
    keysyms::Sys_Req => NamedKey::PrintScreen,
    keysyms::Escape => NamedKey::Escape,
    keysyms::Delete => NamedKey::Delete,

    // IME keys
    keysyms::Multi_key => NamedKey::Compose,
    keysyms::Codeinput => NamedKey::CodeInput,
    keysyms::SingleCandidate => NamedKey::SingleCandidate,
    keysyms::MultipleCandidate => NamedKey::AllCandidates,
    keysyms::PreviousCandidate => NamedKey::PreviousCandidate,

    // Japanese keys
    keysyms::Kanji => NamedKey::KanjiMode,
    keysyms::Muhenkan => NamedKey::NonConvert,
    keysyms::Henkan_Mode => NamedKey::Convert,
    keysyms::Romaji => NamedKey::Romaji,
    keysyms::Hiragana => NamedKey::Hiragana,
    keysyms::Hiragana_Katakana => NamedKey::HiraganaKatakana,
    keysyms::Zenkaku => NamedKey::Zenkaku,
    keysyms::Hankaku => NamedKey::Hankaku,
    keysyms::Zenkaku_Hankaku => NamedKey::ZenkakuHankaku,
    // keysyms::Touroku => NamedKey::Touroku,
    // keysyms::Massyo => NamedKey::Massyo,
    keysyms::Kana_Lock => NamedKey::KanaMode,
    keysyms::Kana_Shift => NamedKey::KanaMode,
    keysyms::Eisu_Shift => NamedKey::Alphanumeric,
    keysyms::Eisu_toggle => NamedKey::Alphanumeric,
    // NOTE: The next three items are aliases for values we've already mapped.
    // keysyms::Kanji_Bangou => NamedKey::CodeInput,
    // keysyms::Zen_Koho => NamedKey::AllCandidates,
    // keysyms::Mae_Koho => NamedKey::PreviousCandidate,

    // Cursor control & motion
    keysyms::Home => NamedKey::Home,
    keysyms::Left => NamedKey::ArrowLeft,
    keysyms::Up => NamedKey::ArrowUp,
    keysyms::Right => NamedKey::ArrowRight,
    keysyms::Down => NamedKey::ArrowDown,
    // keysyms::Prior => NamedKey::PageUp,
    keysyms::Page_Up => NamedKey::PageUp,
    // keysyms::Next => NamedKey::PageDown,
    keysyms::Page_Down => NamedKey::PageDown,
    keysyms::End => NamedKey::End,
    // keysyms::Begin => NamedKey::Begin,

    // Misc. functions
    keysyms::Select => NamedKey::Select,
    keysyms::Print => NamedKey::PrintScreen,
    keysyms::Execute => NamedKey::Execute,
    keysyms::Insert => NamedKey::Insert,
    keysyms::Undo => NamedKey::Undo,
    keysyms::Redo => NamedKey::Redo,
    keysyms::Menu => NamedKey::ContextMenu,
    keysyms::Find => NamedKey::Find,
    keysyms::Cancel => NamedKey::Cancel,
    keysyms::Help => NamedKey::Help,
    keysyms::Break => NamedKey::Pause,
    keysyms::Mode_switch => NamedKey::ModeChange,
    // keysyms::script_switch => NamedKey::ModeChange,
    keysyms::Num_Lock => NamedKey::NumLock,

    // Keypad keys
    // keysyms::KP_Space => return Key::Character(" "),
    keysyms::KP_Tab => NamedKey::Tab,
    keysyms::KP_Enter => NamedKey::Enter,
    keysyms::KP_F1 => NamedKey::F1,
    keysyms::KP_F2 => NamedKey::F2,
    keysyms::KP_F3 => NamedKey::F3,
    keysyms::KP_F4 => NamedKey::F4,
    keysyms::KP_Home => NamedKey::Home,
    keysyms::KP_Left => NamedKey::ArrowLeft,
    keysyms::KP_Up => NamedKey::ArrowUp,
    keysyms::KP_Right => NamedKey::ArrowRight,
    keysyms::KP_Down => NamedKey::ArrowDown,
    // keysyms::KP_Prior => NamedKey::PageUp,
    keysyms::KP_Page_Up => NamedKey::PageUp,
    // keysyms::KP_Next => NamedKey::PageDown,
    keysyms::KP_Page_Down => NamedKey::PageDown,
    keysyms::KP_End => NamedKey::End,
    // This is the key labeled "5" on the numpad when NumLock is off.
    // keysyms::KP_Begin => NamedKey::Begin,
    keysyms::KP_Insert => NamedKey::Insert,
    keysyms::KP_Delete => NamedKey::Delete,
    // keysyms::KP_Equal => NamedKey::Equal,
    // keysyms::KP_Multiply => NamedKey::Multiply,
    // keysyms::KP_Add => NamedKey::Add,
    // keysyms::KP_Separator => NamedKey::Separator,
    // keysyms::KP_Subtract => NamedKey::Subtract,
    // keysyms::KP_Decimal => NamedKey::Decimal,
    // keysyms::KP_Divide => NamedKey::Divide,

    // keysyms::KP_0 => return Key::Character("0"),
    // keysyms::KP_1 => return Key::Character("1"),
    // keysyms::KP_2 => return Key::Character("2"),
    // keysyms::KP_3 => return Key::Character("3"),
    // keysyms::KP_4 => return Key::Character("4"),
    // keysyms::KP_5 => return Key::Character("5"),
    // keysyms::KP_6 => return Key::Character("6"),
    // keysyms::KP_7 => return Key::Character("7"),
    // keysyms::KP_8 => return Key::Character("8"),
    // keysyms::KP_9 => return Key::Character("9"),

    // Function keys
    keysyms::F1 => NamedKey::F1,
    keysyms::F2 => NamedKey::F2,
    keysyms::F3 => NamedKey::F3,
    keysyms::F4 => NamedKey::F4,
    keysyms::F5 => NamedKey::F5,
    keysyms::F6 => NamedKey::F6,
    keysyms::F7 => NamedKey::F7,
    keysyms::F8 => NamedKey::F8,
    keysyms::F9 => NamedKey::F9,
    keysyms::F10 => NamedKey::F10,
    keysyms::F11 => NamedKey::F11,
    keysyms::F12 => NamedKey::F12,
    keysyms::F13 => NamedKey::F13,
    keysyms::F14 => NamedKey::F14,
    keysyms::F15 => NamedKey::F15,
    keysyms::F16 => NamedKey::F16,
    keysyms::F17 => NamedKey::F17,
    keysyms::F18 => NamedKey::F18,
    keysyms::F19 => NamedKey::F19,
    keysyms::F20 => NamedKey::F20,
    keysyms::F21 => NamedKey::F21,
    keysyms::F22 => NamedKey::F22,
    keysyms::F23 => NamedKey::F23,
    keysyms::F24 => NamedKey::F24,
    keysyms::F25 => NamedKey::F25,
    keysyms::F26 => NamedKey::F26,
    keysyms::F27 => NamedKey::F27,
    keysyms::F28 => NamedKey::F28,
    keysyms::F29 => NamedKey::F29,
    keysyms::F30 => NamedKey::F30,
    keysyms::F31 => NamedKey::F31,
    keysyms::F32 => NamedKey::F32,
    keysyms::F33 => NamedKey::F33,
    keysyms::F34 => NamedKey::F34,
    keysyms::F35 => NamedKey::F35,

    // Modifiers
    keysyms::Shift_L => NamedKey::Shift,
    keysyms::Shift_R => NamedKey::Shift,
    keysyms::Control_L => NamedKey::Control,
    keysyms::Control_R => NamedKey::Control,
    keysyms::Caps_Lock => NamedKey::CapsLock,
    // keysyms::Shift_Lock => NamedKey::ShiftLock,
    keysyms::Alt_L => NamedKey::Alt,
    keysyms::Alt_R => NamedKey::Alt,
    #[allow(deprecated)]
    keysyms::Hyper_L => NamedKey::Hyper,
    #[allow(deprecated)]
    keysyms::Hyper_R => NamedKey::Hyper,

    // Browsers map X11's Super keys to Meta, so we do that as well.
    keysyms::Super_L => NamedKey::Meta,
    keysyms::Super_R => NamedKey::Meta,
    // The actual Meta keys do not seem to be used by browsers, so we don't do that either.
    // keysyms::Meta_L => NamedKey::Super,
    // keysyms::Meta_R => NamedKey::Super,

    // XKB function and modifier keys
    // keysyms::ISO_Lock => NamedKey::IsoLock,
    // keysyms::ISO_Level2_Latch => NamedKey::IsoLevel2Latch,
    keysyms::ISO_Level3_Shift => NamedKey::AltGraph,
    keysyms::ISO_Level3_Latch => NamedKey::AltGraph,
    keysyms::ISO_Level3_Lock => NamedKey::AltGraph,
    // keysyms::ISO_Level5_Shift => NamedKey::IsoLevel5Shift,
    // keysyms::ISO_Level5_Latch => NamedKey::IsoLevel5Latch,
    // keysyms::ISO_Level5_Lock => NamedKey::IsoLevel5Lock,
    // keysyms::ISO_Group_Shift => NamedKey::IsoGroupShift,
    // keysyms::ISO_Group_Latch => NamedKey::IsoGroupLatch,
    // keysyms::ISO_Group_Lock => NamedKey::IsoGroupLock,
    keysyms::ISO_Next_Group => NamedKey::GroupNext,
    // keysyms::ISO_Next_Group_Lock => NamedKey::GroupNextLock,
    keysyms::ISO_Prev_Group => NamedKey::GroupPrevious,
    // keysyms::ISO_Prev_Group_Lock => NamedKey::GroupPreviousLock,
    keysyms::ISO_First_Group => NamedKey::GroupFirst,
    // keysyms::ISO_First_Group_Lock => NamedKey::GroupFirstLock,
    keysyms::ISO_Last_Group => NamedKey::GroupLast,
    // keysyms::ISO_Last_Group_Lock => NamedKey::GroupLastLock,
    keysyms::ISO_Left_Tab => NamedKey::Tab,
    // keysyms::ISO_Move_Line_Up => NamedKey::IsoMoveLineUp,
    // keysyms::ISO_Move_Line_Down => NamedKey::IsoMoveLineDown,
    // keysyms::ISO_Partial_Line_Up => NamedKey::IsoPartialLineUp,
    // keysyms::ISO_Partial_Line_Down => NamedKey::IsoPartialLineDown,
    // keysyms::ISO_Partial_Space_Left => NamedKey::IsoPartialSpaceLeft,
    // keysyms::ISO_Partial_Space_Right => NamedKey::IsoPartialSpaceRight,
    // keysyms::ISO_Set_Margin_Left => NamedKey::IsoSetMarginLeft,
    // keysyms::ISO_Set_Margin_Right => NamedKey::IsoSetMarginRight,
    // keysyms::ISO_Release_Margin_Left => NamedKey::IsoReleaseMarginLeft,
    // keysyms::ISO_Release_Margin_Right => NamedKey::IsoReleaseMarginRight,
    // keysyms::ISO_Release_Both_Margins => NamedKey::IsoReleaseBothMargins,
    // keysyms::ISO_Fast_Cursor_Left => NamedKey::IsoFastPointerLeft,
    // keysyms::ISO_Fast_Cursor_Right => NamedKey::IsoFastCursorRight,
    // keysyms::ISO_Fast_Cursor_Up => NamedKey::IsoFastCursorUp,
    // keysyms::ISO_Fast_Cursor_Down => NamedKey::IsoFastCursorDown,
    // keysyms::ISO_Continuous_Underline => NamedKey::IsoContinuousUnderline,
    // keysyms::ISO_Discontinuous_Underline => NamedKey::IsoDiscontinuousUnderline,
    // keysyms::ISO_Emphasize => NamedKey::IsoEmphasize,
    // keysyms::ISO_Center_Object => NamedKey::IsoCenterObject,
    keysyms::ISO_Enter => NamedKey::Enter,

    // dead_grave..dead_currency

    // dead_lowline..dead_longsolidusoverlay

    // dead_a..dead_capital_schwa

    // dead_greek

    // First_Virtual_Screen..Terminate_Server

    // AccessX_Enable..AudibleBell_Enable

    // Pointer_Left..Pointer_Drag5

    // Pointer_EnableKeys..Pointer_DfltBtnPrev

    // ch..C_H

    // 3270 terminal keys
    // keysyms::3270_Duplicate => NamedKey::Duplicate,
    // keysyms::3270_FieldMark => NamedKey::FieldMark,
    // keysyms::3270_Right2 => NamedKey::Right2,
    // keysyms::3270_Left2 => NamedKey::Left2,
    // keysyms::3270_BackTab => NamedKey::BackTab,
    keysyms::_3270_EraseEOF => NamedKey::EraseEof,
    // keysyms::3270_EraseInput => NamedKey::EraseInput,
    // keysyms::3270_Reset => NamedKey::Reset,
    // keysyms::3270_Quit => NamedKey::Quit,
    // keysyms::3270_PA1 => NamedKey::Pa1,
    // keysyms::3270_PA2 => NamedKey::Pa2,
    // keysyms::3270_PA3 => NamedKey::Pa3,
    // keysyms::3270_Test => NamedKey::Test,
    keysyms::_3270_Attn => NamedKey::Attn,
    // keysyms::3270_CursorBlink => NamedKey::CursorBlink,
    // keysyms::3270_AltCursor => NamedKey::AltCursor,
    // keysyms::3270_KeyClick => NamedKey::KeyClick,
    // keysyms::3270_Jump => NamedKey::Jump,
    // keysyms::3270_Ident => NamedKey::Ident,
    // keysyms::3270_Rule => NamedKey::Rule,
    // keysyms::3270_Copy => NamedKey::Copy,
    keysyms::_3270_Play => NamedKey::Play,
    // keysyms::3270_Setup => NamedKey::Setup,
    // keysyms::3270_Record => NamedKey::Record,
    // keysyms::3270_ChangeScreen => NamedKey::ChangeScreen,
    // keysyms::3270_DeleteWord => NamedKey::DeleteWord,
    keysyms::_3270_ExSelect => NamedKey::ExSel,
    keysyms::_3270_CursorSelect => NamedKey::CrSel,
    keysyms::_3270_PrintScreen => NamedKey::PrintScreen,
    keysyms::_3270_Enter => NamedKey::Enter,

    keysyms::space => return LogicalKey::Character(" ".into()),
    // exclam..Sinh_kunddaliya

    // XFree86
    // keysyms::XF86_ModeLock => NamedKey::ModeLock,

    // XFree86 - Backlight controls
    keysyms::XF86_MonBrightnessUp => NamedKey::BrightnessUp,
    keysyms::XF86_MonBrightnessDown => NamedKey::BrightnessDown,
    // keysyms::XF86_KbdLightOnOff => NamedKey::LightOnOff,
    // keysyms::XF86_KbdBrightnessUp => NamedKey::KeyboardBrightnessUp,
    // keysyms::XF86_KbdBrightnessDown => NamedKey::KeyboardBrightnessDown,

    // XFree86 - "Internet"
    keysyms::XF86_Standby => NamedKey::Standby,
    keysyms::XF86_AudioLowerVolume => NamedKey::AudioVolumeDown,
    keysyms::XF86_AudioRaiseVolume => NamedKey::AudioVolumeUp,
    keysyms::XF86_AudioPlay => NamedKey::MediaPlay,
    keysyms::XF86_AudioStop => NamedKey::MediaStop,
    keysyms::XF86_AudioPrev => NamedKey::MediaTrackPrevious,
    keysyms::XF86_AudioNext => NamedKey::MediaTrackNext,
    keysyms::XF86_HomePage => NamedKey::BrowserHome,
    keysyms::XF86_Mail => NamedKey::LaunchMail,
    // keysyms::XF86_Start => NamedKey::Start,
    keysyms::XF86_Search => NamedKey::BrowserSearch,
    keysyms::XF86_AudioRecord => NamedKey::MediaRecord,

    // XFree86 - PDA
    keysyms::XF86_Calculator => NamedKey::LaunchApplication2,
    // keysyms::XF86_Memo => NamedKey::Memo,
    // keysyms::XF86_ToDoList => NamedKey::ToDoList,
    keysyms::XF86_Calendar => NamedKey::LaunchCalendar,
    keysyms::XF86_PowerDown => NamedKey::Power,
    // keysyms::XF86_ContrastAdjust => NamedKey::AdjustContrast,
    // keysyms::XF86_RockerUp => NamedKey::RockerUp,
    // keysyms::XF86_RockerDown => NamedKey::RockerDown,
    // keysyms::XF86_RockerEnter => NamedKey::RockerEnter,

    // XFree86 - More "Internet"
    keysyms::XF86_Back => NamedKey::BrowserBack,
    keysyms::XF86_Forward => NamedKey::BrowserForward,
    // keysyms::XF86_Stop => NamedKey::Stop,
    keysyms::XF86_Refresh => NamedKey::BrowserRefresh,
    keysyms::XF86_PowerOff => NamedKey::Power,
    keysyms::XF86_WakeUp => NamedKey::WakeUp,
    keysyms::XF86_Eject => NamedKey::Eject,
    keysyms::XF86_ScreenSaver => NamedKey::LaunchScreenSaver,
    keysyms::XF86_WWW => NamedKey::LaunchWebBrowser,
    keysyms::XF86_Sleep => NamedKey::Standby,
    keysyms::XF86_Favorites => NamedKey::BrowserFavorites,
    keysyms::XF86_AudioPause => NamedKey::MediaPause,
    // keysyms::XF86_AudioMedia => NamedKey::AudioMedia,
    keysyms::XF86_MyComputer => NamedKey::LaunchApplication1,
    // keysyms::XF86_VendorHome => NamedKey::VendorHome,
    // keysyms::XF86_LightBulb => NamedKey::LightBulb,
    // keysyms::XF86_Shop => NamedKey::BrowserShop,
    // keysyms::XF86_History => NamedKey::BrowserHistory,
    // keysyms::XF86_OpenURL => NamedKey::OpenUrl,
    // keysyms::XF86_AddFavorite => NamedKey::AddFavorite,
    // keysyms::XF86_HotLinks => NamedKey::HotLinks,
    // keysyms::XF86_BrightnessAdjust => NamedKey::BrightnessAdjust,
    // keysyms::XF86_Finance => NamedKey::BrowserFinance,
    // keysyms::XF86_Community => NamedKey::BrowserCommunity,
    keysyms::XF86_AudioRewind => NamedKey::MediaRewind,
    // keysyms::XF86_BackForward => Key::???,
    // XF86_Launch0..XF86_LaunchF

    // XF86_ApplicationLeft..XF86_CD
    keysyms::XF86_Calculater => NamedKey::LaunchApplication2, // Nice typo, libxkbcommon :)
    // XF86_Clear
    keysyms::XF86_Close => NamedKey::Close,
    keysyms::XF86_Copy => NamedKey::Copy,
    keysyms::XF86_Cut => NamedKey::Cut,
    // XF86_Display..XF86_Documents
    keysyms::XF86_Excel => NamedKey::LaunchSpreadsheet,
    // XF86_Explorer..XF86iTouch
    keysyms::XF86_LogOff => NamedKey::LogOff,
    // XF86_Market..XF86_MenuPB
    keysyms::XF86_MySites => NamedKey::BrowserFavorites,
    keysyms::XF86_New => NamedKey::New,
    // XF86_News..XF86_OfficeHome
    keysyms::XF86_Open => NamedKey::Open,
    // XF86_Option
    keysyms::XF86_Paste => NamedKey::Paste,
    keysyms::XF86_Phone => NamedKey::LaunchPhone,
    // XF86_Q
    keysyms::XF86_Reply => NamedKey::MailReply,
    keysyms::XF86_Reload => NamedKey::BrowserRefresh,
    // XF86_RotateWindows..XF86_RotationKB
    keysyms::XF86_Save => NamedKey::Save,
    // XF86_ScrollUp..XF86_ScrollClick
    keysyms::XF86_Send => NamedKey::MailSend,
    keysyms::XF86_Spell => NamedKey::SpellCheck,
    keysyms::XF86_SplitScreen => NamedKey::SplitScreenToggle,
    // XF86_Support..XF86_User2KB
    keysyms::XF86_Video => NamedKey::LaunchMediaPlayer,
    // XF86_WheelButton
    keysyms::XF86_Word => NamedKey::LaunchWordProcessor,
    // XF86_Xfer
    keysyms::XF86_ZoomIn => NamedKey::ZoomIn,
    keysyms::XF86_ZoomOut => NamedKey::ZoomOut,

    // XF86_Away..XF86_Messenger
    keysyms::XF86_WebCam => NamedKey::LaunchWebCam,
    keysyms::XF86_MailForward => NamedKey::MailForward,
    // XF86_Pictures
    keysyms::XF86_Music => NamedKey::LaunchMusicPlayer,

    // XF86_Battery..XF86_UWB
    keysyms::XF86_AudioForward => NamedKey::MediaFastForward,
    // XF86_AudioRepeat
    keysyms::XF86_AudioRandomPlay => NamedKey::RandomToggle,
    keysyms::XF86_Subtitle => NamedKey::Subtitle,
    keysyms::XF86_AudioCycleTrack => NamedKey::MediaAudioTrack,
    // XF86_CycleAngle..XF86_Blue
    keysyms::XF86_Suspend => NamedKey::Standby,
    keysyms::XF86_Hibernate => NamedKey::Hibernate,
    // XF86_TouchpadToggle..XF86_TouchpadOff
    keysyms::XF86_AudioMute => NamedKey::AudioVolumeMute,

    // XF86_Switch_VT_1..XF86_Switch_VT_12

    // XF86_Ungrab..XF86_ClearGrab
    keysyms::XF86_Next_VMode => NamedKey::VideoModeNext,
    // keysyms::XF86_Prev_VMode => NamedKey::VideoModePrevious,
    // XF86_LogWindowTree..XF86_LogGrabInfo

    // SunFA_Grave..SunFA_Cedilla

    // keysyms::SunF36 => NamedKey::F36 | NamedKey::F11,
    // keysyms::SunF37 => NamedKey::F37 | NamedKey::F12,

    // keysyms::SunSys_Req => NamedKey::PrintScreen,
    // The next couple of xkb (until SunStop) are already handled.
    // SunPrint_Screen..SunPageDown

    // SunUndo..SunFront
    keysyms::SUN_Copy => NamedKey::Copy,
    keysyms::SUN_Open => NamedKey::Open,
    keysyms::SUN_Paste => NamedKey::Paste,
    keysyms::SUN_Cut => NamedKey::Cut,

    // SunPowerSwitch
    keysyms::SUN_AudioLowerVolume => NamedKey::AudioVolumeDown,
    keysyms::SUN_AudioMute => NamedKey::AudioVolumeMute,
    keysyms::SUN_AudioRaiseVolume => NamedKey::AudioVolumeUp,
    // SUN_VideoDegauss
    keysyms::SUN_VideoLowerBrightness => NamedKey::BrightnessDown,
    keysyms::SUN_VideoRaiseBrightness => NamedKey::BrightnessUp,
    // SunPowerSwitchShift
    0 => return LogicalKey::Unidentified(NativeKey::Unidentified),
    _ => return LogicalKey::Unidentified(NativeKey::Key(keysym)),
  })
}

pub fn keysym_location(keysym: u32) -> Location {
  use xkbcommon_dl::keysyms;
  match keysym {
    keysyms::Shift_L
    | keysyms::Control_L
    | keysyms::Meta_L
    | keysyms::Alt_L
    | keysyms::Super_L
    | keysyms::Hyper_L => Location::Left,
    keysyms::Shift_R
    | keysyms::Control_R
    | keysyms::Meta_R
    | keysyms::Alt_R
    | keysyms::Super_R
    | keysyms::Hyper_R => Location::Right,
    keysyms::KP_0
    | keysyms::KP_1
    | keysyms::KP_2
    | keysyms::KP_3
    | keysyms::KP_4
    | keysyms::KP_5
    | keysyms::KP_6
    | keysyms::KP_7
    | keysyms::KP_8
    | keysyms::KP_9
    | keysyms::KP_Space
    | keysyms::KP_Tab
    | keysyms::KP_Enter
    | keysyms::KP_F1
    | keysyms::KP_F2
    | keysyms::KP_F3
    | keysyms::KP_F4
    | keysyms::KP_Home
    | keysyms::KP_Left
    | keysyms::KP_Up
    | keysyms::KP_Right
    | keysyms::KP_Down
    | keysyms::KP_Page_Up
    | keysyms::KP_Page_Down
    | keysyms::KP_End
    | keysyms::KP_Begin
    | keysyms::KP_Insert
    | keysyms::KP_Delete
    | keysyms::KP_Equal
    | keysyms::KP_Multiply
    | keysyms::KP_Add
    | keysyms::KP_Separator
    | keysyms::KP_Subtract
    | keysyms::KP_Decimal
    | keysyms::KP_Divide => Location::Numpad,
    _ => Location::Standard,
  }
}

#[derive(Debug)]
pub struct XkbKeymap {
  keymap: ThreadPtr<xkb_keymap>,
  _mods_indices: ModsIndices,
  pub _core_keyboard_id: i32,
}

impl XkbKeymap {
  pub fn from_x11_keymap(
    context: &XkbContext,
    xcb: *mut xcb_connection_t,
    core_keyboard_id: i32,
  ) -> Option<Self> {
    let keymap = unsafe {
      (XKBXH.xkb_x11_keymap_new_from_device)(
        context.as_ptr(),
        xcb,
        core_keyboard_id,
        xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
      )
    };
    let keymap = ThreadPtr::new(keymap)?;
    Some(Self::new_inner(keymap, core_keyboard_id))
  }

  fn new_inner(keymap: ThreadPtr<xkb_keymap>, _core_keyboard_id: i32) -> Self {
    let mods_indices = ModsIndices {
      shift: mod_index_for_name(keymap.clone(), xkb::XKB_MOD_NAME_SHIFT),
      caps: mod_index_for_name(keymap.clone(), xkb::XKB_MOD_NAME_CAPS),
      ctrl: mod_index_for_name(keymap.clone(), xkb::XKB_MOD_NAME_CTRL),
      alt: mod_index_for_name(keymap.clone(), xkb::XKB_MOD_NAME_ALT),
      num: mod_index_for_name(keymap.clone(), xkb::XKB_MOD_NAME_NUM),
      mod3: mod_index_for_name(keymap.clone(), b"Mod3\0"),
      logo: mod_index_for_name(keymap.clone(), xkb::XKB_MOD_NAME_LOGO),
      mod5: mod_index_for_name(keymap.clone(), b"Mod5\0"),
    };

    Self { keymap, _mods_indices: mods_indices, _core_keyboard_id }
  }

  pub fn mods_indices(&self) -> ModsIndices {
    self._mods_indices
  }

  pub fn first_keysym_by_level(
    &mut self,
    layout: xkb_layout_index_t,
    keycode: xkb_keycode_t,
  ) -> xkb_keysym_t {
    unsafe {
      let mut keysyms = std::ptr::null();
      let count = (XKBH.xkb_keymap_key_get_syms_by_level)(
        self.keymap.as_ptr(),
        keycode,
        layout,
        // NOTE: The level should be zero to ignore modifiers.
        0,
        &mut keysyms,
      );

      if count == 1 { *keysyms } else { 0 }
    }
  }

  /// Check whether the given key repeats.
  pub fn key_repeats(&mut self, keycode: xkb_keycode_t) -> bool {
    unsafe { (XKBH.xkb_keymap_key_repeats)(self.keymap.as_ptr(), keycode) == 1 }
  }
}

impl Drop for XkbKeymap {
  fn drop(&mut self) {
    unsafe {
      (XKBH.xkb_keymap_unref)(self.keymap.as_ptr());
    };
  }
}

impl Deref for XkbKeymap {
  type Target = ThreadPtr<xkb_keymap>;

  fn deref(&self) -> &Self::Target {
    &self.keymap
  }
}

/// Modifier index in the keymap.
#[derive(Default, Debug, Clone, Copy)]
pub struct ModsIndices {
  pub shift: Option<xkb_mod_index_t>,
  pub caps: Option<xkb_mod_index_t>,
  pub ctrl: Option<xkb_mod_index_t>,
  pub alt: Option<xkb_mod_index_t>,
  pub num: Option<xkb_mod_index_t>,
  pub mod3: Option<xkb_mod_index_t>,
  pub logo: Option<xkb_mod_index_t>,
  pub mod5: Option<xkb_mod_index_t>,
}

fn mod_index_for_name(keymap: ThreadPtr<xkb_keymap>, name: &[u8]) -> Option<xkb_mod_index_t> {
  unsafe {
    let mod_index = (XKBH.xkb_keymap_mod_get_index)(keymap.as_ptr(), name.as_ptr() as *const c_char);
    if mod_index == XKB_MOD_INVALID { None } else { Some(mod_index) }
  }
}
