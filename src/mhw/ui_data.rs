#[derive(Debug)]
pub enum Crown {
	None,
	SmallGold,
	Silver,
	Gold,
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
	pub crown: Crown,
}

impl MonsterInfo {
	pub fn get_monster_info() -> Self {
		// TODO: get from data
		MonsterInfo {
			name: "<N/A>".to_string(),
			hp: 0,
			max_hp: 0,
			crown: Crown::None,
		}
	}

	pub fn get_hp_percentage(&self) -> f64 {
		((self.hp as f64) / (self.max_hp as f64)) * 100.0
	}
}

pub struct UiInfo {
	pub player_info: [Option<PlayerInfo>; 4],
	pub monster_info: [Option<MonsterInfo>; 3],
	pub session_id: String,
	pub host_name: String,
}

impl UiInfo {
	pub fn get_num_players(&self) -> usize {
		self.player_info.iter().filter(|p| p.is_some()).count()
	}

	pub fn get_num_monsters(&self) -> usize {
		self.monster_info.iter().filter(|m| m.is_some()).count()
	}
}
