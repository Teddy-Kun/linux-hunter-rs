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
pub struct FullData {
	pub players: [Option<PlayerInfo>; 4],
	pub monsters: [Option<MonsterInfo>; 3],
	pub session_id: String,
	pub host_name: String,
}

impl FullData {
	pub fn get_total_damage(&self) -> usize {
		let mut total: usize = 0;
		for player in self.players.iter().flatten() {
			total += player.damage;
		}

		total
	}

	pub fn get_num_players(&self) -> usize {
		self.players.iter().filter(|p| p.is_some()).count()
	}

	pub fn get_num_monsters(&self) -> usize {
		self.monsters.iter().filter(|m| m.is_some()).count()
	}
}
