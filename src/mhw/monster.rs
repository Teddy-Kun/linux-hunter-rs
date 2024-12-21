use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug)]
pub enum CrownType {
	Undefined,
	Standard,
	Alternate,
	Savage,
	Rajang,
}

#[derive(Debug)]
pub struct CrownData {
	pub crown_type: CrownType,
	pub small: f64,
	pub large: f64,
	pub very_large: f64,
}

const UNDEFINED_CROWN_DATA: CrownData = CrownData {
	crown_type: CrownType::Undefined,
	small: 1.0,
	large: 1.0,
	very_large: 1.0,
};

const STANDARD_CROWN_DATA: CrownData = CrownData {
	crown_type: CrownType::Standard,
	small: 0.90,
	large: 1.15,
	very_large: 1.23,
};

const ALTERNATE_CROWN_DATA: CrownData = CrownData {
	crown_type: CrownType::Alternate,
	small: 0.90,
	large: 1.10,
	very_large: 1.20,
};

const SAVAGE_CROWN_DATA: CrownData = CrownData {
	crown_type: CrownType::Savage,
	small: 0.99,
	large: 1.14,
	very_large: 1.20,
};

const RAJANG_CROWN_DATA: CrownData = CrownData {
	crown_type: CrownType::Rajang,
	small: 0.90,
	large: 1.11,
	very_large: 1.28,
};

#[derive(Debug)]
pub struct MonsterData<'a> {
	pub str_id: &'a str,
	pub id: u32,
	pub base_size: f64,
	pub crown_data: CrownData,
	pub name: &'a str,
}

const MONSTERS: [MonsterData; 71] = [
	MonsterData {
		str_id: "em001_00",
		id: 9,
		base_size: 1754.37,
		crown_data: STANDARD_CROWN_DATA,
		name: "Rathian",
	},
	MonsterData {
		str_id: "em001_01",
		id: 10,
		base_size: 1754.37,
		crown_data: STANDARD_CROWN_DATA,
		name: "Pink Rathian",
	},
	MonsterData {
		str_id: "em001_02",
		id: 88,
		base_size: 1754.37,
		crown_data: STANDARD_CROWN_DATA,
		name: "Gold Rathian",
	},
	MonsterData {
		str_id: "em002_00",
		id: 1,
		base_size: 1704.22,
		crown_data: STANDARD_CROWN_DATA,
		name: "Rathalos",
	},
	MonsterData {
		str_id: "em002_01",
		id: 11,
		base_size: 1704.22,
		crown_data: STANDARD_CROWN_DATA,
		name: "Azure Rathalos",
	},
	MonsterData {
		str_id: "em002_02",
		id: 89,
		base_size: 1704.22,
		crown_data: STANDARD_CROWN_DATA,
		name: "Silver Rathalos",
	},
	MonsterData {
		str_id: "em007_00",
		id: 12,
		base_size: 2096.25,
		crown_data: STANDARD_CROWN_DATA,
		name: "Diablos",
	},
	MonsterData {
		str_id: "em007_01",
		id: 13,
		base_size: 2096.25,
		crown_data: STANDARD_CROWN_DATA,
		name: "Black Diablos",
	},
	MonsterData {
		str_id: "em011_00",
		id: 14,
		base_size: 536.26,
		crown_data: STANDARD_CROWN_DATA,
		name: "Kirin",
	},
	MonsterData {
		str_id: "em018_00",
		id: 90,
		base_size: 1389.01,
		crown_data: STANDARD_CROWN_DATA,
		name: "Yian Garuga",
	},
	MonsterData {
		str_id: "em018_05",
		id: 99,
		base_size: 1389.01,
		crown_data: STANDARD_CROWN_DATA,
		name: "Scarred Yian Garuga",
	},
	MonsterData {
		str_id: "em023_00",
		id: 91,
		base_size: 829.11,
		crown_data: RAJANG_CROWN_DATA,
		name: "Rajang",
	},
	MonsterData {
		str_id: "em023_05",
		id: 92,
		base_size: 829.11,
		crown_data: RAJANG_CROWN_DATA,
		name: "Furious Rajang",
	},
	MonsterData {
		str_id: "em024_00",
		id: 16,
		base_size: 1913.13,
		crown_data: STANDARD_CROWN_DATA,
		name: "Kushala Daora",
	},
	MonsterData {
		str_id: "em026_00",
		id: 17,
		base_size: 1828.69,
		crown_data: STANDARD_CROWN_DATA,
		name: "Lunastra",
	},
	MonsterData {
		str_id: "em027_00",
		id: 18,
		base_size: 1790.15,
		crown_data: STANDARD_CROWN_DATA,
		name: "Teostra",
	},
	MonsterData {
		str_id: "em032_00",
		id: 61,
		base_size: 1943.20,
		crown_data: STANDARD_CROWN_DATA,
		name: "Tigrex",
	},
	MonsterData {
		str_id: "em032_01",
		id: 93,
		base_size: 1943.20,
		crown_data: STANDARD_CROWN_DATA,
		name: "Brute Tigrex",
	},
	MonsterData {
		str_id: "em036_00",
		id: 19,
		base_size: 1797.24,
		crown_data: STANDARD_CROWN_DATA,
		name: "Lavasioth",
	},
	MonsterData {
		str_id: "em037_00",
		id: 62,
		base_size: 1914.74,
		crown_data: STANDARD_CROWN_DATA,
		name: "Nargacuga",
	},
	MonsterData {
		str_id: "em042_00",
		id: 63,
		base_size: 2098.30,
		crown_data: STANDARD_CROWN_DATA,
		name: "Barioth",
	},
	MonsterData {
		str_id: "em043_00",
		id: 20,
		base_size: 2063.82,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Deviljho",
	},
	MonsterData {
		str_id: "em043_05",
		id: 64,
		base_size: 2063.82,
		crown_data: SAVAGE_CROWN_DATA,
		name: "Savage Deviljho",
	},
	MonsterData {
		str_id: "em044_00",
		id: 21,
		base_size: 1383.07,
		crown_data: STANDARD_CROWN_DATA,
		name: "Barroth",
	},
	MonsterData {
		str_id: "em045_00",
		id: 22,
		base_size: 2058.63,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Uragaan",
	},
	MonsterData {
		str_id: "em057_00",
		id: 94,
		base_size: 1743.49,
		crown_data: STANDARD_CROWN_DATA,
		name: "Zinogre",
	},
	MonsterData {
		str_id: "em063_00",
		id: 65,
		base_size: 1630.55,
		crown_data: STANDARD_CROWN_DATA,
		name: "Brachydios",
	},
	MonsterData {
		str_id: "em063_05",
		id: 96,
		base_size: 2282.77,
		crown_data: STANDARD_CROWN_DATA,
		name: "Raging Brachydios",
	},
	MonsterData {
		str_id: "em057_01",
		id: 95,
		base_size: 1743.49,
		crown_data: STANDARD_CROWN_DATA,
		name: "Stygian Zinogre",
	},
	MonsterData {
		str_id: "em080_00",
		id: 66,
		base_size: 2461.50,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Glavenus",
	},
	MonsterData {
		str_id: "em080_01",
		id: 67,
		base_size: 2372.44,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Acidic Glavenus",
	},
	MonsterData {
		str_id: "em100_00",
		id: 0,
		base_size: 1646.46,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Anjanath",
	},
	MonsterData {
		str_id: "em100_01",
		id: 68,
		base_size: 1646.46,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Fulgur Anjanath",
	},
	MonsterData {
		str_id: "em101_00",
		id: 7,
		base_size: 1109.66,
		crown_data: STANDARD_CROWN_DATA,
		name: "Great Jagras",
	},
	MonsterData {
		str_id: "em102_00",
		id: 24,
		base_size: 1102.45,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Pukei Pukei",
	},
	MonsterData {
		str_id: "em102_01",
		id: 69,
		base_size: 1102.45,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Coral Pukei Pukei",
	},
	MonsterData {
		str_id: "em103_00",
		id: 25,
		base_size: 1848.12,
		crown_data: STANDARD_CROWN_DATA,
		name: "Nergigante",
	},
	MonsterData {
		str_id: "em103_05",
		id: 70,
		base_size: 1848.12,
		crown_data: STANDARD_CROWN_DATA,
		name: "Ruiner Nergigante",
	},
	MonsterData {
		str_id: "em104_00",
		id: 97,
		base_size: 4799.78,
		crown_data: STANDARD_CROWN_DATA,
		name: "Safi Jiiva",
	},
	MonsterData {
		str_id: "em105_00",
		id: 26,
		base_size: 4509.10,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Xeno Jiiva",
	},
	MonsterData {
		str_id: "em106_00",
		id: 4,
		base_size: 25764.59,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Zorah Magdaros",
	},
	MonsterData {
		str_id: "em107_00",
		id: 27,
		base_size: 901.24,
		crown_data: STANDARD_CROWN_DATA,
		name: "Kulu Ya Ku",
	},
	MonsterData {
		str_id: "em108_00",
		id: 29,
		base_size: 1508.71,
		crown_data: STANDARD_CROWN_DATA,
		name: "Jyuratodus",
	},
	MonsterData {
		str_id: "em109_00",
		id: 30,
		base_size: 1300.52,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Tobi Kadachi",
	},
	MonsterData {
		str_id: "em109_01",
		id: 71,
		base_size: 1300.52,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Viper Tobi Kadachi",
	},
	MonsterData {
		str_id: "em110_00",
		id: 31,
		base_size: 1143.36,
		crown_data: STANDARD_CROWN_DATA,
		name: "Paolumu",
	},
	MonsterData {
		str_id: "em110_01",
		id: 72,
		base_size: 1143.36,
		crown_data: STANDARD_CROWN_DATA,
		name: "Nightshade Paolumu",
	},
	MonsterData {
		str_id: "em111_00",
		id: 32,
		base_size: 1699.75,
		crown_data: STANDARD_CROWN_DATA,
		name: "Legiana",
	},
	MonsterData {
		str_id: "em111_05",
		id: 73,
		base_size: 1831.69,
		crown_data: STANDARD_CROWN_DATA,
		name: "Shrieking Legiana",
	},
	MonsterData {
		str_id: "em112_00",
		id: 33,
		base_size: 1053.15,
		crown_data: STANDARD_CROWN_DATA,
		name: "Great Girros",
	},
	MonsterData {
		str_id: "em113_00",
		id: 34,
		base_size: 1388.75,
		crown_data: STANDARD_CROWN_DATA,
		name: "Odogaron",
	},
	MonsterData {
		str_id: "em113_01",
		id: 74,
		base_size: 1388.75,
		crown_data: STANDARD_CROWN_DATA,
		name: "Ebony Odogaron",
	},
	MonsterData {
		str_id: "em114_00",
		id: 35,
		base_size: 1803.47,
		crown_data: ALTERNATE_CROWN_DATA,
		name: "Radobaan",
	},
	MonsterData {
		str_id: "em115_00",
		id: 36,
		base_size: 2095.40,
		crown_data: STANDARD_CROWN_DATA,
		name: "Vaal Hazak",
	},
	MonsterData {
		str_id: "em115_05",
		id: 75,
		base_size: 2095.40,
		crown_data: STANDARD_CROWN_DATA,
		name: "Blackveil Vaal Hazak",
	},
	MonsterData {
		str_id: "em116_00",
		id: 37,
		base_size: 1111.11,
		crown_data: STANDARD_CROWN_DATA,
		name: "Dodogama",
	},
	MonsterData {
		str_id: "em117_00",
		id: 38,
		base_size: 4573.25,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Kulve Taroth",
	},
	MonsterData {
		str_id: "em118_00",
		id: 39,
		base_size: 1928.38,
		crown_data: STANDARD_CROWN_DATA,
		name: "Bazelgeuse",
	},
	MonsterData {
		str_id: "em118_05",
		id: 76,
		base_size: 1928.38,
		crown_data: STANDARD_CROWN_DATA,
		name: "Seething Bazelgeuse",
	},
	MonsterData {
		str_id: "em120_00",
		id: 28,
		base_size: 894.04,
		crown_data: STANDARD_CROWN_DATA,
		name: "Tzitzi Ya Ku",
	},
	MonsterData {
		str_id: "em121_00",
		id: 15,
		base_size: 3423.65,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Behemoth",
	},
	MonsterData {
		str_id: "em122_00",
		id: 77,
		base_size: 1661.99,
		crown_data: STANDARD_CROWN_DATA,
		name: "Beotodus",
	},
	MonsterData {
		str_id: "em123_00",
		id: 78,
		base_size: 2404.84,
		crown_data: STANDARD_CROWN_DATA,
		name: "Banbaro",
	},
	MonsterData {
		str_id: "em124_00",
		id: 79,
		base_size: 2596.05,
		crown_data: STANDARD_CROWN_DATA,
		name: "Velkhana",
	},
	MonsterData {
		str_id: "em125_00",
		id: 80,
		base_size: 2048.25,
		crown_data: STANDARD_CROWN_DATA,
		name: "Namielle",
	},
	MonsterData {
		str_id: "em126_00",
		id: 81,
		base_size: 2910.91,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Shara Ishvalda",
	},
	MonsterData {
		str_id: "em127_00",
		id: 23,
		base_size: 549.70,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Leshen",
	},
	MonsterData {
		str_id: "em127_01",
		id: 51,
		base_size: 633.81,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Ancient Leshen",
	},
	MonsterData {
		str_id: "em050_00",
		id: 87,
		base_size: 2969.63,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Alatreon",
	},
	MonsterData {
		str_id: "em042_05",
		id: 100,
		base_size: 2098.30,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Frostfang Barioth",
	},
	MonsterData {
		str_id: "em013_00",
		id: 101,
		base_size: 4137.17,
		crown_data: UNDEFINED_CROWN_DATA,
		name: "Fatalis",
	},
];

lazy_static! {
	pub static ref MONSTER_MAP: HashMap<u32, &'static MonsterData<'static>> = {
		let mut map = HashMap::new();
		for monster in MONSTERS.iter() {
			map.insert(monster.id, monster);
		}
		map
	};
}
