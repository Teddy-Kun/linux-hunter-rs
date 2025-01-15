pub const PREVIOUS_MONSTER: usize = 0x10;
pub const NEXT_MONSTER: usize = 0x18;
pub const MONSTER_START_OF_STRUCT: usize = 0x40;
pub const MONSTER_HEALTH_COMPONENT: usize = 0x7670;
pub const MONSTER_ID: usize = 0x12280;
pub const MONSTER_SIZE_SCALE: usize = 0x188;
pub const MONSTER_SCALE_MODIFIER: usize = 0x7730;

pub const MONSTER_MODEL_ID_LENGTH: usize = 32;
pub const MONSTER_MODEL_ID_OFFSET: usize = 0x179;

pub const MONSTER_HEALTH_COMPONENT_MAX: usize = 0x60;
pub const MONSTER_HEALTH_COMPONENT_CURRENT: usize = 0x64;

pub const PLAYER_NAME_LENGTH: usize = 32;
pub const FIRST_PLAYER_NAME: usize = 0x53305;

pub const SESSION_ID: usize = FIRST_PLAYER_NAME + 0xf43;
pub const SESSION_HOST_NAME: usize = SESSION_ID + 0x3f;
pub const ID_LENGTH: usize = 12;

pub const EXPEDITION_STATUS_OFFSET: usize = 0x38;
pub const MISSION_STATUS_OFFSET: usize = 0x54;
