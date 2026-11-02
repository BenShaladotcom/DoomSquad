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

pub struct KeyBindings {
    pub map: HashMap<Action, Key>,
}