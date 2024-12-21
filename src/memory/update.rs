use super::{
	pattern::{PatternGetter, PatternType},
	region::MemoryRegion,
};
use crate::mhw::offsets::{
	MONSTER_HEALTH_COMPONENT, MONSTER_ID, MONSTER_MODEL_ID_LENGTH, MONSTER_START_OF_STRUCT,
};
use std::collections::HashMap;

pub fn get_monster_data(raw_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
	let start = MONSTER_START_OF_STRUCT + MONSTER_HEALTH_COMPONENT;

	let relevant_data = &raw_data[start..];

	let _h_comp_addr = 0;

	let monster_id_offset = MONSTER_ID + 0x0c;
	let str_id = &relevant_data[monster_id_offset..monster_id_offset + MONSTER_MODEL_ID_LENGTH];
	let id = &relevant_data[MONSTER_ID..MONSTER_ID + 4];
	let id: u32 = id.iter().fold(0, |acc, x| acc << 8 | *x as u32);

	println!("str_id: {}", String::from_utf8_lossy(str_id));
	println!("id: {}", id);

	Ok(())
}

pub fn update_all(patterns: &[PatternGetter], mem_regions: &HashMap<usize, MemoryRegion>) {
	for p in patterns {
		let region = mem_regions.get(&p.index.unwrap()).unwrap();
		let relevant_data = &region.data[p.offset.unwrap() - 24..];

		match p.pattern_type {
			PatternType::Monsters => {
				println!("index: {}", p.index.unwrap());
				println!("offset: 0x{:02X}", p.offset.unwrap());
				println!("relevant_data: {:02X?}", &relevant_data[..50]);

				let _ = get_monster_data(relevant_data);
			}
			_ => {
				println!("TODO: {:?}", p.pattern_type)
			}
		}
	}
}
