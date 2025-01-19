use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Crown {
	SmallGold,
	Silver,
	Gold,
}

impl Display for Crown {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Crown::SmallGold => write!(f, "Small Gold"),
			Crown::Silver => write!(f, "Silver"),
			Crown::Gold => write!(f, "Gold"),
		}
	}
}

#[derive(Debug)]
pub struct PlayerInfo {
	pub name: String,
	pub damage: usize,
	pub left_session: bool,
}

impl PlayerInfo {
	pub fn get_player_info() -> Self {
		todo!("get player info from bytes");
	}
}

#[derive(Debug)]
pub struct MonsterInfo {
	pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub crown: Option<Crown>,
}

impl MonsterInfo {
	pub fn get_monster_info() -> Self {
		todo!("get player info from bytes");
	}
}

#[derive(Debug, Default)]
pub struct SessionInfo {
	pub session_id: String,
	pub hostname: String,
	pub is_mission: bool,
	pub is_expedition: bool,
}

#[derive(Debug, Default)]
pub struct GameData {
	pub session: SessionInfo,
	pub monsters: Box<[MonsterInfo]>,
	pub players: Box<[PlayerInfo]>,
}

impl GameData {
	pub fn new(session: SessionInfo) -> Self {
		Self {
			session,
			monsters: Box::new([]),
			players: Box::new([]),
		}
	}

	pub fn get_total_damage(&self) -> usize {
		let mut total: usize = 0;
		for player in self.players.iter() {
			total += player.damage;
		}

		total
	}
}
