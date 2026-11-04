use minifb::Key;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]

pub enum Action {
    Forwards,
    Backwards,
    Sprint,
    Crouch,
    Interact,
    MoveCrosshairDown,
    TurnRight,
    RocketJump,
    FireBullet,
    EnterGodmode,
    ExitToMainMenu,
    Left,
    Right,
    Jump,
    MoveCrosshairUp,
    TurnLeft,
    FineAiming,
    FireArrow,
    LockEnemies, //locks all current Enemies
}

impl Action {
    pub const ALL: [Action; 19] = [
        Action::Forwards,
        Action::Backwards,
        Action::Crouch,
        Action::EnterGodmode,
        Action::ExitToMainMenu,
        Action::FineAiming,
        Action::FireArrow,
        Action::FireBullet,
        Action::Interact,
        Action::Jump,
        Action::Left,
        Action::LockEnemies,
        Action::MoveCrosshairDown,
        Action::MoveCrosshairUp,
        Action::Right,
        Action::RocketJump,
        Action::Sprint,
        Action::TurnLeft,
        Action::TurnRight,
    ];
}

pub const ALL_KEYS: [Key; 107] = [
    Key::Key0,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
    Key::Key9,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::Up,
    Key::Apostrophe,
    Key::Backquote,
    Key::Backslash,
    Key::Comma,
    Key::Equal,
    Key::LeftBracket,
    Key::Minus,
    Key::Period,
    Key::RightBracket,
    Key::Semicolon,
    Key::Slash,
    Key::Backspace,
    Key::Delete,
    Key::End,
    Key::Enter,
    Key::Escape,
    Key::Home,
    Key::Insert,
    Key::Menu,
    Key::PageDown,
    Key::PageUp,
    Key::Pause,
    Key::Space,
    Key::Tab,
    Key::NumLock,
    Key::CapsLock,
    Key::ScrollLock,
    Key::LeftShift,
    Key::RightShift,
    Key::LeftCtrl,
    Key::RightCtrl,
    Key::NumPad0,
    Key::NumPad1,
    Key::NumPad2,
    Key::NumPad3,
    Key::NumPad4,
    Key::NumPad5,
    Key::NumPad6,
    Key::NumPad7,
    Key::NumPad8,
    Key::NumPad9,
    Key::NumPadDot,
    Key::NumPadSlash,
    Key::NumPadAsterisk,
    Key::NumPadMinus,
    Key::NumPadPlus,
    Key::NumPadEnter,
    Key::LeftAlt,
    Key::RightAlt,
    Key::LeftSuper,
    Key::RightSuper,
    Key::Unknown,
];

pub struct KeyBindings {
    pub map: HashMap<Action, Key>,
}

impl KeyBindings {
    pub fn default_bindings() -> Self {
        let mut map = HashMap::new();

        map.insert(Action::Forwards, Key::W);
        map.insert(Action::Backwards, Key::S);
        map.insert(Action::Left, Key::A);
        map.insert(Action::Right, Key::D);
        map.insert(Action::Sprint, Key::LeftShift);
        map.insert(Action::Jump, Key::Space);
        map.insert(Action::Crouch, Key::C);
        map.insert(Action::RocketJump, Key::R);
        map.insert(Action::Interact, Key::F);
        map.insert(Action::ExitToMainMenu, Key::Escape);
        map.insert(Action::MoveCrosshairUp, Key::Up);
        map.insert(Action::MoveCrosshairDown, Key::Down);
        map.insert(Action::TurnLeft, Key::Left);
        map.insert(Action::TurnRight, Key::Right);
        map.insert(Action::FineAiming, Key::E);
        map.insert(Action::FireBullet, Key::RightCtrl);
        map.insert(Action::FireArrow, Key::RightShift);
        map.insert(Action::EnterGodmode, Key::G);
        map.insert(Action::LockEnemies, Key::L);

        KeyBindings { map }
    }

    pub fn key(&self, action: Action) -> Key {
        self.map[&action]
    }

    pub fn load() -> Self {
        let mut bindings = KeyBindings::default_bindings();

        if let Ok(content) = fs::read_to_string("keybindings.txt") {
            for line in content.lines() {
                if let Some((action_text, key_text)) = line.split_once('=') {
                    for action in Action::ALL {
                        if format!("{:?}", action) == action_text {
                            for key in ALL_KEYS {
                                if format!("{:?}", key) == key_text {
                                    bindings.map.insert(action, key);
                                }
                            }
                        }
                    }
                }
            }
        }

        bindings
    }

    pub fn save(&self) {
        let mut text = String::new();
        for action in Action::ALL {
            text.push_str(&format!("{:?}={:?}\n", action, self.key(action)));
        }

        let _ = fs::write("keybindings.txt", &text);
    }
}
