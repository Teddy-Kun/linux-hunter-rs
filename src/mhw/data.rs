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

#[derive(Debug)]
pub struct MonsterInfo {
	pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub crown: Option<Crown>,
}

impl MonsterInfo {
	pub fn get_monster_info() -> Self {
		// TODO: get from data
		MonsterInfo {
			name: "<N/A>".to_string(),
			hp: 0,
			max_hp: 0,
			crown: None,
		}
	}

	pub fn get_hp_percentage(&self) -> f64 {
		((self.hp as f64) / (self.max_hp as f64)) * 100.0
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
		for p in &self.players {
			if let Some(player) = p {
				total += player.damage;
			}
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
