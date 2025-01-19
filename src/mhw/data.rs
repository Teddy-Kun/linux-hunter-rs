use std::fmt::Display;

use super::monster::{MonsterData, MONSTER_MAP};

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
	pub name: Box<str>,
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
	pub id: u32,
	pub name: Box<str>,
	pub hp: u32,
	pub max_hp: u32,
	pub size: f64,
	pub crown: Option<Crown>,
}

impl MonsterInfo {
	pub fn new(
		id: u32,
		hp: u32,
		max_hp: u32,
		size: f64,
	) -> Result<Self, Box<dyn std::error::Error>> {
		let monster_data = match MONSTER_MAP.get(&id) {
			None => {
				return Err(format!("unknown monster id: {}", id).into());
			}
			Some(data) => *data,
		};
		let crown = Self::calc_crown(size, monster_data);
		Ok(Self {
			id,
			name: Box::from(MONSTER_MAP.get(&id).unwrap().name),
			hp,
			max_hp,
			size,
			crown,
		})
	}

	fn calc_crown(size: f64, monster_data: &MonsterData) -> Option<Crown> {
		let small_size = monster_data.base_size * monster_data.crown_data.small;
		if size < small_size {
			return Some(Crown::SmallGold);
		}

		let gold_size = monster_data.base_size * monster_data.crown_data.very_large;
		if size >= gold_size {
			return Some(Crown::Gold);
		}

		let silver_size = monster_data.base_size * monster_data.crown_data.large;
		if size >= silver_size {
			return Some(Crown::Silver);
		}

		None
	}
}

#[derive(Debug)]
pub struct SessionInfo {
	pub session_id: Box<str>,
	pub hostname: Box<str>,
	pub is_mission: bool,
	pub is_expedition: bool,
}

impl Default for SessionInfo {
	fn default() -> Self {
		Self {
			session_id: Box::from(""),
			hostname: Box::from(""),
			is_mission: true, // set to true by default since this will make us scan for additional patterns in case this was not found
			is_expedition: false,
		}
	}
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
