use std::collections::HashMap;
use minifb::Key;


#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq )]



pub enum Action  {
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
    LockEnemies  //locks all current Enemies
}

impl Action{
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

}