use super::region::MemoryRegion;
use std::fmt::Display;

#[derive(Debug)]
pub enum PatternType {
	PlayerName,
	CurrentPlayerName,
	PlayerDamage,
	Monsters,
	PlayerBuff,
	LobbyStatus,
	Emetta,
	PlayerNameLinux,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryLocation {
	pub start: usize,  // start of the memory region where the pattern was found
	pub offset: usize, // offset of where it was found, relative to the start of the region
	pub address: usize,
}

impl MemoryLocation {
	pub fn new(start: usize, offset: usize) -> Self {
		Self {
			start,
			offset,
			address: start + offset,
		}
	}
}

impl Display for MemoryLocation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"start: 0x{:02X}; offset: {}; addr: {}",
			self.start, self.offset, self.address,
		)
	}
}

#[derive(Debug)]
pub struct PatternGetter {
	pub mem_location: Option<MemoryLocation>,
	pub pattern_type: PatternType,
	find_func: fn(&[u8]) -> Option<usize>,
}

impl PatternGetter {
	pub fn new(pattern_type: PatternType, find_func: fn(&[u8]) -> Option<usize>) -> Self {
		PatternGetter {
			pattern_type,
			find_func,
			mem_location: None,
		}
	}

	pub fn search(&mut self, mem_region: &MemoryRegion) -> anyhow::Result<()> {
		let data = match &mem_region.data {
			Some(data) => data,
			None => return Err(anyhow::anyhow!("Memory region has no data")),
		};

		match (self.find_func)(data) {
			Some(res) => {
				let loc = MemoryLocation::new(mem_region.get_begin(), res);
				self.mem_location = Some(loc);
			}
			None => {
				self.mem_location = None;
				return Err(anyhow::anyhow!("not found"));
			}
		};

		Ok(())
	}
}

pub fn get_search_index(first_bytes: &[u8], input: &[u8]) -> Option<usize> {
	let mut inp = input;

	let mut i = memchr::memchr(first_bytes[0], input)?;
	let mut index_in_og = i;
	let fb_len = first_bytes.len();

	while i < input.len() - 2 {
		if i >= inp.len() || fb_len + 8 > inp.len() {
			return None;
		}

		let bytes_to_compare = &inp[i..fb_len + i];

		if *bytes_to_compare == *first_bytes {
			return Some(index_in_og);
		}

		inp = &inp[i + 1..];

		i = memchr::memchr(first_bytes[0], inp)?;

		index_in_og += i;
	}

	None
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 54 24 38 C6 44 24 20 00 E8 ?? ?? ?? ?? 48 8B 5C 24 70 48 8B 7C 24 60 48 83 C4 68 C3
pub fn find_player_name(input: &[u8]) -> Option<usize> {
	const INITIAL_BYTES: [u8; 3] = [0x48, 0x8B, 0x0D];
	const TOTAL_LEN: usize = 37;

	let mut sliced;
	let mut pos = get_search_index(&INITIAL_BYTES, input)?;
	sliced = &input[pos..];
	dbg!(&sliced[..TOTAL_LEN]);

	const SKIP_1: usize = 4;

	const POS_SECTION_2: usize = INITIAL_BYTES.len() + SKIP_1;
	const SECTION_2: [u8; 11] = [
		0x48, 0x8D, 0x54, 0x24, 0x38, 0xC6, 0x44, 0x24, 0x20, 0x00, 0xE8,
	];

	const SKIP_2: usize = 4;

	const POS_SECTION_3: usize = POS_SECTION_2 + SECTION_2.len() + SKIP_2;
	const SECTION_3: [u8; 15] = [
		0x48, 0x8B, 0x5C, 0x24, 0x70, 0x48, 0x8B, 0x7C, 0x24, 0x60, 0x48, 0x83, 0xC4, 0x68, 0xC3,
	];

	// checks if the start of the sliced array is the same as the search pattern
	let condition = |input: &[u8]| -> bool {
		input[POS_SECTION_2..POS_SECTION_2 + SECTION_2.len()] == SECTION_2
			&& input[POS_SECTION_3..POS_SECTION_3 + SECTION_3.len()] == SECTION_3
	};

	let mut total_pos = 0;

	while sliced.len() > TOTAL_LEN {
		total_pos += pos;

		if condition(sliced) {
			return Some(total_pos);
		} else {
			sliced = &sliced[1..];
			pos = get_search_index(&INITIAL_BYTES, sliced)?;
			sliced = &sliced[pos..];
			total_pos += 1;
		}
	}
	None
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 55 ?? 45 31 C9 41 89 C0 E8
pub fn find_current_player_name(input: &[u8]) -> Option<usize> {
	const INITIAL_BYTES: [u8; 3] = [0x48, 0x8B, 0x0D];
	const TOTAL_LEN: usize = 18;

	let mut sliced;
	let mut pos: usize = get_search_index(&INITIAL_BYTES, input)?;
	sliced = &input[pos..];

	const SKIP_1: usize = 4;

	const POS_SECTION_2: usize = INITIAL_BYTES.len() + SKIP_1;
	const SECTION_2: [u8; 3] = [0x48, 0x8D, 0x55];

	const SKIP_2: usize = 1;

	const POS_SECTION_3: usize = POS_SECTION_2 + SECTION_2.len() + SKIP_2;
	const SECTION_3: [u8; 7] = [0x45, 0x31, 0xC9, 0x41, 0x89, 0xC0, 0xE8];

	let condition = |input: &[u8]| -> bool {
		input[POS_SECTION_2..POS_SECTION_2 + SECTION_2.len()] == SECTION_2
			&& input[POS_SECTION_3..POS_SECTION_3 + SECTION_3.len()] == SECTION_3
	};

	let mut total_pos = 0;

	while sliced.len() > TOTAL_LEN {
		total_pos += pos;

		if condition(sliced) {
			return Some(total_pos);
		} else {
			sliced = &sliced[1..];
			pos = get_search_index(&INITIAL_BYTES, sliced)?;
			sliced = &sliced[pos..];
			total_pos += 1;
		}
	}

	None
}

// 48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B D8 48 85 C0 75 04 33 C9
pub fn find_player_damage(input: &[u8]) -> Option<usize> {
	const INITIAL_BYTES: [u8; 3] = [0x48, 0x8B, 0x0D];
	const TOTAL_LEN: usize = 22;

	let mut sliced;
	let mut pos = get_search_index(&INITIAL_BYTES, input)?;
	sliced = &input[pos..];

	const SKIP_1: usize = 4;

	const POS_SECTION_2: usize = INITIAL_BYTES.len() + SKIP_1;
	const SECTION_2: u8 = 0xE8;

	const SKIP_2: usize = 4;

	const POS_SECTION_3: usize = POS_SECTION_2 + 1 + SKIP_2;
	const SECTION_3: [u8; 10] = [0x48, 0x8B, 0xD8, 0x48, 0x85, 0xC0, 0x75, 0x04, 0x33, 0xC9];

	let condition = |input: &[u8]| -> bool {
		input[POS_SECTION_2] == SECTION_2
			&& input[POS_SECTION_3..POS_SECTION_3 + SECTION_3.len()] == SECTION_3
	};

	let mut total_pos = 0;

	while sliced.len() > TOTAL_LEN {
		total_pos += pos;

		if condition(sliced) {
			return Some(total_pos);
		} else {
			sliced = &sliced[1..];
			pos = get_search_index(&INITIAL_BYTES, sliced)?;
			sliced = &sliced[pos..];
			total_pos += 1;
		}
	}

	None
}

// 48 8B 0D ?? ?? ?? ?? B2 01 E8 ?? ?? ?? ?? C6 83 ?? ?? ?? ?? ?? 48 8B 0D
pub fn find_monster(input: &[u8]) -> Option<usize> {
	const INITIAL_BYTES: [u8; 3] = [0x48, 0x8B, 0x0D];
	const TOTAL_LEN: usize = 24;

	let mut sliced;
	let mut pos = get_search_index(&INITIAL_BYTES, input)?;
	sliced = &input[pos..];

	const SKIP_1: usize = 4;

	const POS_SECTION_2: usize = INITIAL_BYTES.len() + SKIP_1;
	const SECTION_2: [u8; 3] = [0xB2, 0x01, 0xE8];

	const SKIP_2: usize = 4;

	const POS_SECTION_3: usize = POS_SECTION_2 + SECTION_2.len() + SKIP_2;
	const SECTION_3: [u8; 2] = [0xC6, 0x83];

	const SKIP_3: usize = 5;

	const POS_SECTION_4: usize = POS_SECTION_3 + SECTION_3.len() + SKIP_3;
	const SECTION_4: [u8; 3] = [0x48, 0x8B, 0x0D];

	let condition = |input: &[u8]| -> bool {
		input[POS_SECTION_2..POS_SECTION_2 + SECTION_2.len()] == SECTION_2
			&& input[POS_SECTION_3..POS_SECTION_3 + SECTION_3.len()] == SECTION_3
			&& input[POS_SECTION_4..POS_SECTION_4 + SECTION_4.len()] == SECTION_4
	};

	let mut total_pos = 0;

	while sliced.len() > TOTAL_LEN {
		total_pos += pos;

		if condition(sliced) {
			return Some(total_pos);
		} else {
			sliced = &sliced[1..];
			pos = get_search_index(&INITIAL_BYTES, sliced)?;
			sliced = &sliced[pos..];
			total_pos += 1;
		}
	}

	None
}

// 48 8B 05 ?? ?? ?? ?? 41 8B 94 00 ?? ?? ?? ?? 89 57
pub fn find_player_buff(input: &[u8]) -> Option<usize> {
	const INITIAL_BYTES: [u8; 3] = [0x48, 0x8B, 0x05];
	const TOTAL_LEN: usize = 17;

	let mut sliced;
	let mut pos = get_search_index(&INITIAL_BYTES, input)?;
	sliced = &input[pos..];

	const SKIP_1: usize = 4;

	const POS_SECTION_2: usize = INITIAL_BYTES.len() + SKIP_1;
	const SECTION_2: [u8; 4] = [0x41, 0x8B, 0x94, 0x00];

	const SKIP_2: usize = 4;

	const POS_SECTION_3: usize = POS_SECTION_2 + SECTION_2.len() + SKIP_2;
	const SECTION_3: [u8; 2] = [0x89, 0x57];

	let condition = |input: &[u8]| -> bool {
		input[POS_SECTION_2..POS_SECTION_2 + SECTION_2.len()] == SECTION_2
			&& input[POS_SECTION_3..POS_SECTION_3 + SECTION_3.len()] == SECTION_3
	};

	let mut total_pos = 0;

	while sliced.len() > TOTAL_LEN {
		total_pos += pos;

		if condition(sliced) {
			return Some(total_pos);
		} else {
			sliced = &sliced[1..];
			pos = get_search_index(&INITIAL_BYTES, sliced)?;
			sliced = &sliced[pos..];
			total_pos += 1;
		}
	}

	None
}

// 48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E ?? F3 0F 10 86 ?? ?? ?? ?? F3 0F 58 86 ?? ?? ?? ?? F3 0F 11 86 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E
pub fn find_lobby_status(input: &[u8]) -> Option<usize> {
	const INITIAL_BYTES: [u8; 3] = [0x48, 0x8B, 0x0D];
	const TOTAL_LEN: usize = 39;

	let mut sliced;
	let mut pos = get_search_index(&INITIAL_BYTES, input)?;
	sliced = &input[pos..];

	const SKIP_1: usize = 4;

	const POS_SECTION_2: usize = INITIAL_BYTES.len() + SKIP_1;
	const SECTION_2: u8 = 0xE8;

	const SKIP_2: usize = 4;

	const POS_SECTION_3: usize = POS_SECTION_2 + 1 + SKIP_2;
	const SECTION_3: [u8; 3] = [0x48, 0x8B, 0x4E];

	const SKIP_3: usize = 1;

	const POS_SECTION_4: usize = POS_SECTION_3 + SECTION_3.len() + SKIP_3;
	const SECTION_4: [u8; 4] = [0xF3, 0x0F, 0x10, 0x86];

	const SKIP_4: usize = 4;

	const POS_SECTION_5: usize = POS_SECTION_4 + SECTION_4.len() + SKIP_4;
	const SECTION_5: [u8; 4] = [0xF3, 0x0F, 0x58, 0x86];

	const SKIP_5: usize = 4;

	const POS_SECTION_6: usize = POS_SECTION_5 + SECTION_5.len() + SKIP_5;
	const SECTION_6: [u8; 4] = [0xF3, 0x0F, 0x11, 0x86];

	const SKIP_6: usize = 4;

	const POS_SECTION_7: usize = POS_SECTION_6 + SECTION_6.len() + SKIP_6;
	const SECTION_7: u8 = 0xE8;

	const SKIP_7: usize = 4;

	const POS_SECTION_8: usize = POS_SECTION_7 + 1 + SKIP_7;
	const SECTION_8: [u8; 3] = [0x48, 0x8B, 0x4E];

	let condition = |input: &[u8]| -> bool {
		input[POS_SECTION_2] == SECTION_2
			&& input[POS_SECTION_3..POS_SECTION_3 + SECTION_3.len()] == SECTION_3
			&& input[POS_SECTION_4..POS_SECTION_4 + SECTION_4.len()] == SECTION_4
			&& input[POS_SECTION_5..POS_SECTION_5 + SECTION_5.len()] == SECTION_5
			&& input[POS_SECTION_6..POS_SECTION_6 + SECTION_6.len()] == SECTION_6
			&& input[POS_SECTION_7] == SECTION_7
			&& input[POS_SECTION_8..POS_SECTION_8 + SECTION_8.len()] == SECTION_8
	};

	let mut total_pos = 0;

	while sliced.len() > TOTAL_LEN {
		total_pos += pos;

		if condition(sliced) {
			return Some(total_pos);
		} else {
			sliced = &sliced[1..];
			pos = get_search_index(&INITIAL_BYTES, sliced)?;
			sliced = &sliced[pos..];
			total_pos += 1;
		}
	}

	None
}

// 45 6D 65 74 74 61
pub fn find_emetta(input: &[u8]) -> Option<usize> {
	let bytes = [0x45, 0x6D, 0x65, 0x74, 0x74, 0x61];
	get_search_index(&bytes, input)
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 54 24 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 48 8B 5C 24 60 48 83 C4 50 5F C3
pub fn find_player_name_linux(input: &[u8]) -> Option<usize> {
	const INITIAL_BYTES: [u8; 3] = [0x48, 0x8B, 0x0D];
	const TOTAL_LEN: usize = 36;

	let mut sliced;
	let mut pos = get_search_index(&INITIAL_BYTES, input)?;
	sliced = &input[pos..];

	const SKIP_1: usize = 4;

	const POS_SECTION_2: usize = INITIAL_BYTES.len() + SKIP_1;
	const SECTION_2: [u8; 4] = [0x48, 0x8D, 0x54, 0x24];

	const SKIP_2: usize = 14;

	const POS_SECTION_3: usize = POS_SECTION_2 + SECTION_2.len() + SKIP_2;
	const SECTION_3: [u8; 11] = [
		0x48, 0x8B, 0x5C, 0x24, 0x60, 0x48, 0x83, 0xC4, 0x50, 0x5F, 0xC3,
	];

	let condition = |input: &[u8]| -> bool {
		input[POS_SECTION_2..POS_SECTION_2 + SECTION_2.len()] == SECTION_2
			&& input[POS_SECTION_3..POS_SECTION_3 + SECTION_3.len()] == SECTION_3
	};

	let mut total_pos = 0;

	while sliced.len() > TOTAL_LEN {
		total_pos += pos;

		if condition(sliced) {
			return Some(total_pos);
		} else {
			sliced = &sliced[1..];
			pos = get_search_index(&INITIAL_BYTES, sliced)?;
			sliced = &sliced[pos..];
			total_pos += 1;
		}
	}

	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_find_player_name() {
		let data = [
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x48, 0x8b, 0x0d, 0x0a, 0x0c, 0xba, 0x04, 0x48, 0x8d, 0x54, 0x24, 0x38, 0xc6, 0x44,
			0x24, 0x20, 0x00, 0xe8, 0x23, 0xfb, 0x4d, 0x01, 0x48, 0x8b, 0x5c, 0x24, 0x70, 0x48,
			0x8b, 0x7c, 0x24, 0x60, 0x48, 0x83, 0xc4, 0x68, 0xc3, 0x48, 0x63, 0x87, 0x58, 0x02,
			0x00, 0x00, 0x4c, 0x8d, 0x0d, 0xd6, 0xab, 0xac, 0x00, 0x4c, 0x8b, 0x47, 0x08, 0x8b,
			0x94, 0x87, 0x28, 0x02, 0x00, 0x00, 0xe8, 0x66,
		];

		match find_player_name(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 3], [0x48, 0x8B, 0x0D]);
			}
		}
	}

	#[test]
	fn test_find_current_player_name() {
		let data = [
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x55, 0x00, 0x45, 0x31,
			0xC9, 0x41, 0x89, 0xC0, 0xE8, 0x00,
		];

		match find_current_player_name(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 3], [0x48, 0x8B, 0x0D]);
			}
		}
	}

	#[test]
	fn test_find_player_damage() {
		let data = [
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48,
			0x8B, 0xD8, 0x48, 0x85, 0xC0, 0x75, 0x04, 0x33, 0xC9, 0x00,
		];

		match find_player_damage(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 3], [0x48, 0x8B, 0x0D]);
			}
		}
	}

	#[test]
	fn test_find_monster() {
		let data = [
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0xB2, 0x01, 0xE8, 0x00, 0x00, 0x00,
			0x00, 0xC6, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x0D, 0x00,
		];

		match find_monster(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 3], [0x48, 0x8B, 0x0D]);
			}
		}
	}

	#[test]
	fn test_find_player_buff() {
		let data = [
			0x00, 0x48, 0x8B, 0x05, 0x00, 0x00, 0x00, 0x00, 0x41, 0x8B, 0x94, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x89, 0x57, 0x00,
		];

		match find_player_buff(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 3], [0x48, 0x8B, 0x05]);
			}
		}
	}

	#[test]
	fn test_find_lobby_status() {
		let data = [
			0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x00, 0x48, 0x00,
			0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B,
			0x4E, 0x00, 0xF3, 0x0F, 0x10, 0x86, 0x00, 0x00, 0x00, 0x00, 0xF3, 0x0F, 0x58, 0x86,
			0x00, 0x00, 0x00, 0x00, 0xF3, 0x0F, 0x11, 0x86, 0x00, 0x00, 0x00, 0x00, 0xE8, 0x00,
			0x00, 0x00, 0x00, 0x48, 0x8B, 0x4E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		];

		match find_lobby_status(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 3], [0x48, 0x8B, 0x0D]);
			}
		}
	}

	#[test]
	fn test_find_emetta() {
		let data = vec![
			0x00, 0x00, 0x00, 0x00, 0x45, 0x6D, 0x65, 0x74, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00,
		];

		match find_emetta(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 6], [0x45, 0x6D, 0x65, 0x74, 0x74, 0x61]);
			}
		}
	}

	#[test]
	fn test_find_player_name_linux() {
		let data = [
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x54, 0x24, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B,
			0x5C, 0x24, 0x60, 0x48, 0x83, 0xC4, 0x50, 0x5F, 0xC3, 0x00,
		];

		match find_player_name_linux(&data) {
			None => panic!("pattern not found"),
			Some(pos) => {
				assert_eq!(data[pos..pos + 3], [0x48, 0x8B, 0x0D]);
			}
		}
	}
}
