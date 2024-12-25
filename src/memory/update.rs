use super::{
	pattern::{PatternGetter, PatternType},
	region::MemoryRegion,
};
use crate::{
	memory::pattern::get_search_index,
	mhw::offsets::{
		MONSTER_HEALTH_COMPONENT, MONSTER_ID, MONSTER_MODEL_ID_LENGTH, MONSTER_START_OF_STRUCT,
	},
};
use std::collections::HashMap;

pub fn get_monster_data(raw_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
	println!("raw_data: {:02X?}", &raw_data[..24]);

	let banbaro_id = b"em1";
	println!("Banbaro ID: {:02X?}", banbaro_id);

	println!("{:?}", get_search_index(banbaro_id, &raw_data));

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

		if region.from_vec {
			continue;
		}

		let relevant_data = &region.data[p.offset.unwrap()..];

		// TODO: data is to short??

		match p.pattern_type {
			PatternType::Monsters => {
				println!("len: {}", region.data.len());

				println!(
					"should be here: {:02X?}",
					&relevant_data[4034814..27 + 4034814]
				);

				let _ = get_monster_data(relevant_data);
			}
			_ => {
				println!("TODO: {:?}", p.pattern_type)
			}
		}
	}
}
