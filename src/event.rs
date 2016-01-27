// #[cfg(target_os="windows")] pub use self::win32::KeyCode;
use winapi::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    KeyboardInput(KeyCode, KeyboardEvent),
    MouseInput(MouseEvent, i16, i16),
    Closed,
    Resized(u16, u16),
    Moved(i16, i16),
    Focused(bool)
}

#[derive(Debug, PartialEq, Eq)]
pub enum MouseEvent {
    Move,
    /// Left mouse button pressed
    LeftButtonDown,
    /// Left mouse button released
    LeftButtonUp,
    /// Right mouse button pressed
    RightButtonDown,
    /// Right mouse button released
    RightButtonUp,
    /// Middle mouse button pressed
    MiddleButtonDown,
    /// Middle mouse button released
    MiddleButtonUp
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyboardEvent {
    /// Key pressed
    Pressed,
    /// Key held
    Repeated,
    /// Key released
    Released
}

// #[cfg(target_os="windows")]
// mod win32 {
macro_rules! keycode {
    ($($key:ident => $name:ident),+) => (
        #[derive(Debug, PartialEq, Eq)]
        #[allow(missing_docs)]
        /// Friendly names for Windows VK_* constants
        pub enum KeyCode {
            /// Unknown key, contained value is the virtual key code
            //Unknown(i32),
            Unknown,
            $($name),+
        }

        impl KeyCode {
            /// Convert from Windows virtual keycode.
            pub fn from_virtual_key(virtual_key: i32) -> KeyCode {
                match virtual_key {
                    $($key => KeyCode::$name),+,
                    _ => KeyCode::Unknown
                }
            }
        }
    )
}

keycode! {
    VK_LSHIFT => LeftShift,
    VK_RSHIFT => RightShift,
    VK_LCONTROL => LeftControl,
    VK_RCONTROL => RightControl,
    VK_LMENU => LeftAlt,
    VK_RMENU => RightAlt,
    VK_OEM_PLUS => Plus,
    VK_OEM_COMMA => Comma,
    VK_OEM_MINUS => Minus,
    VK_OEM_PERIOD => Period,
    VK_OEM_1 => SemiColon,
    VK_OEM_2 => Slash,
    VK_OEM_3 => Tilde,
    VK_OEM_4 => LeftBracket,
    VK_OEM_5 => BackSlash,
    VK_OEM_6 => RightBracket,
    VK_OEM_7 => Quote,
    VK_BACK => Backspace,
    VK_TAB => Tab,
    VK_RETURN => Return,
    VK_PAUSE => Pause,
    VK_ESCAPE => Escape,
    VK_SPACE => Space,
    VK_PRIOR => PageUp,
    VK_NEXT => PageDown,
    VK_END => End,
    VK_HOME => Home,
    VK_LEFT => Left,
    VK_UP => Up,
    VK_RIGHT => Right,
    VK_DOWN => Down,
    VK_SNAPSHOT => PrintScreen,
    VK_INSERT => Insert,
    VK_DELETE => Delete,
    VK_CAPITAL => CapsLock,
    VK_NUMLOCK => NumLock,
    VK_SCROLL => ScrollLock,
    VK_0 => Key0,
    VK_1 => Key1,
    VK_2 => Key2,
    VK_3 => Key3,
    VK_4 => Key4,
    VK_5 => Key5,
    VK_6 => Key6,
    VK_7 => Key7,
    VK_8 => Key8,
    VK_9 => Key9,
    VK_A => A,
    VK_B => B,
    VK_C => C,
    VK_D => D,
    VK_E => E,
    VK_F => F,
    VK_G => G,
    VK_H => H,
    VK_I => I,
    VK_J => J,
    VK_K => K,
    VK_L => L,
    VK_M => M,
    VK_N => N,
    VK_O => O,
    VK_P => P,
    VK_Q => Q,
    VK_R => R,
    VK_S => S,
    VK_T => T,
    VK_U => U,
    VK_V => V,
    VK_W => W,
    VK_X => X,
    VK_Y => Y,
    VK_Z => Z,
    VK_NUMPAD0 => Numpad0,
    VK_NUMPAD1 => Numpad1,
    VK_NUMPAD2 => Numpad2,
    VK_NUMPAD3 => Numpad3,
    VK_NUMPAD4 => Numpad4,
    VK_NUMPAD5 => Numpad5,
    VK_NUMPAD6 => Numpad6,
    VK_NUMPAD7 => Numpad7,
    VK_NUMPAD8 => Numpad8,
    VK_NUMPAD9 => Numpad9,
    VK_MULTIPLY => NumpadMultiply,
    VK_ADD => NumpadPlus,
    VK_SUBTRACT => NumpadMinus,
    VK_DECIMAL => NumpadPeriod,
    VK_DIVIDE => NumpadDivide,
    VK_F1 => F1,
    VK_F2 => F2,
    VK_F3 => F3,
    VK_F4 => F4,
    VK_F5 => F5,
    VK_F6 => F6,
    VK_F7 => F7,
    VK_F8 => F8,
    VK_F9 => F9,
    VK_F10 => F10,
    VK_F11 => F11,
    VK_F12 => F12,
    VK_F13 => F13,
    VK_F14 => F14,
    VK_F15 => F15,
    VK_F16 => F16,
    VK_F17 => F17,
    VK_F18 => F18,
    VK_F19 => F19,
    VK_F20 => F20,
    VK_F21 => F21,
    VK_F22 => F22,
    VK_F23 => F23,
    VK_F24 => F24
}

// Define alphanumeric virtual keys for the keycode macro above
const VK_0: i32 = 0x30;
const VK_1: i32 = 0x31;
const VK_2: i32 = 0x32;
const VK_3: i32 = 0x33;
const VK_4: i32 = 0x34;
const VK_5: i32 = 0x35;
const VK_6: i32 = 0x36;
const VK_7: i32 = 0x37;
const VK_8: i32 = 0x38;
const VK_9: i32 = 0x39;
const VK_A: i32 = 0x41;
const VK_B: i32 = 0x42;
const VK_C: i32 = 0x43;
const VK_D: i32 = 0x44;
const VK_E: i32 = 0x45;
const VK_F: i32 = 0x46;
const VK_G: i32 = 0x47;
const VK_H: i32 = 0x48;
const VK_I: i32 = 0x49;
const VK_J: i32 = 0x4A;
const VK_K: i32 = 0x4B;
const VK_L: i32 = 0x4C;
const VK_M: i32 = 0x4D;
const VK_N: i32 = 0x4E;
const VK_O: i32 = 0x4F;
const VK_P: i32 = 0x50;
const VK_Q: i32 = 0x51;
const VK_R: i32 = 0x52;
const VK_S: i32 = 0x53;
const VK_T: i32 = 0x54;
const VK_U: i32 = 0x55;
const VK_V: i32 = 0x56;
const VK_W: i32 = 0x57;
const VK_X: i32 = 0x58;
const VK_Y: i32 = 0x59;
const VK_Z: i32 = 0x5A;


//}