use memchr::memchr3;
use nom::{
	bytes::streaming::{tag, take, take_until},
	sequence::tuple,
};

use crate::err::Error;

#[derive(Debug)]
pub struct PatternGetter {
	pub result: Option<Vec<u8>>,
	pub debug_name: String,
	find_func: fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>,
	pub index: usize, // for saving the index of the region of the found pattern
}

impl PatternGetter {
	pub fn new(
		debug_name: &str,
		find_func: fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>,
	) -> Self {
		PatternGetter {
			result: None,
			debug_name: debug_name.to_string(),
			find_func,
			index: 0,
		}
	}

	pub fn search(&mut self, input: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
		match (self.find_func)(input) {
			Ok(res) => self.result = Some(res),
			Err(e) => {
				self.result = None;
				return Err(e);
			}
		};
		Ok(())
	}
}

fn get_search_index(
	first_three_bytes: &[u8; 3],
	input: &[u8],
) -> Result<usize, Box<dyn std::error::Error>> {
	match memchr3(
		first_three_bytes[0],
		first_three_bytes[1],
		first_three_bytes[2],
		input,
	) {
		Some(pos) => Ok(pos),
		None => Err(Error::new("pattern not found").into()),
	}
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 54 24 38 C6 44 24 20 00 E8 ?? ?? ?? ?? 48 8B 5C 24 70 48 8B 7C 24 60 48 83 C4 68 C3
pub fn find_player_name(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let mut sliced;
	let pos = get_search_index(&initial_bytes, input)?;
	sliced = &input[pos..];

	let mut pattern = tuple((
		take::<_, _, nom::error::Error<&[u8]>>(initial_bytes.len()),
		take(4usize),
		tag(&[
			0x48, 0x8D, 0x54, 0x24, 0x38, 0xC6, 0x44, 0x24, 0x20, 0x00, 0xE8,
		]),
		take(4usize),
		tag(&[
			0x48, 0x8B, 0x5C, 0x24, 0x70, 0x48, 0x8B, 0x7C, 0x24, 0x60, 0x48, 0x83, 0xC4, 0x68,
			0xC3,
		]),
	));

	// since nom fails to find the pattern, if it matches the first ones partially, but the rest does not match, we need to try again, after removing the first bytes
	while sliced.len() > 37 {
		match pattern(sliced) {
			Ok((_, res)) => return Ok([res.0, res.1, res.2, res.3, res.4].concat()),
			Err(_) => {
				// we can safely skip the same amount of bytes that we search for, since we failed to find it
				sliced = &sliced[37..];
				let pos = get_search_index(&initial_bytes, sliced)?;
				sliced = &sliced[pos..];
			}
		}
	}

	Err(Error::new("pattern not found").into())
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 55 ?? 45 31 C9 41 89 C0 E8
pub fn find_current_player_name(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let mut sliced;
	let pos = get_search_index(&initial_bytes, input)?;
	sliced = &input[pos..];

	let mut pattern = tuple((
		take::<_, _, nom::error::Error<&[u8]>>(initial_bytes.len()),
		take(4usize),
		tag(&[0x48, 0x8D, 0x55]),
		take(1usize),
		tag(&[0x45, 0x31, 0xC9, 0x41, 0x89, 0xC0, 0xE8]),
	));

	// since nom fails to find the pattern, if it matches the first ones partially, but the rest does not match, we need to try again, after removing the first bytes
	while sliced.len() > 37 {
		match pattern(sliced) {
			Ok((_, res)) => return Ok([res.0, res.1, res.2, res.3, res.4].concat()),
			Err(_) => {
				// we can safely skip the same amount of bytes that we search for, since we failed to find it
				sliced = &sliced[37..];
				let pos = get_search_index(&initial_bytes, sliced)?;
				sliced = &sliced[pos..];
			}
		}
	}

	Err(Error::new("pattern not found").into())
}

// 48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B D8 48 85 C0 75 04 33 C9
pub fn find_player_damage(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let mut sliced;
	let pos = get_search_index(&initial_bytes, input)?;
	sliced = &input[pos..];

	let mut pattern = tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0xE8]),
		take(4usize),
		tag(&[0x48, 0x8B, 0xD8, 0x48, 0x85, 0xC0, 0x75, 0x04, 0x33, 0xC9]),
	));

	// since nom fails to find the pattern, if it matches the first ones partially, but the rest does not match, we need to try again, after removing the first bytes
	while sliced.len() > 37 {
		match pattern(sliced) {
			Ok((_, res)) => return Ok([res.0, res.1, res.2, res.3, res.4].concat()),
			Err(_) => {
				// we can safely skip the same amount of bytes that we search for, since we failed to find it
				sliced = &sliced[37..];
				let pos = get_search_index(&initial_bytes, sliced)?;
				sliced = &sliced[pos..];
			}
		}
	}

	Err(Error::new("pattern not found").into())
}

// 48 8B 0D ?? ?? ?? ?? B2 01 E8 ?? ?? ?? ?? C6 83 ?? ?? ?? ?? ?? 48 8B 0D
pub fn find_monster(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let mut sliced;
	let pos = get_search_index(&initial_bytes, input)?;
	sliced = &input[pos..];

	let mut pattern = tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0xB2, 0x01, 0xE8]),
		take(4usize),
		tag(&[0xC6, 0x83]),
		take(5usize),
		tag(&[0x48, 0x8B, 0x0D]),
	));

	// since nom fails to find the pattern, if it matches the first ones partially, but the rest does not match, we need to try again, after removing the first bytes
	while sliced.len() > 37 {
		match pattern(sliced) {
			Ok((_, res)) => return Ok([res.0, res.1, res.2, res.3, res.4, res.5, res.6].concat()),
			Err(_) => {
				// we can safely skip the same amount of bytes that we search for, since we failed to find it
				sliced = &sliced[37..];
				let pos = get_search_index(&initial_bytes, sliced)?;
				sliced = &sliced[pos..];
			}
		}
	}

	Err(Error::new("pattern not found").into())
}

// 48 8B 05 ?? ?? ?? ?? 41 8B 94 00 ?? ?? ?? ?? 89 57
pub fn find_player_buff(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x05];

	let mut sliced;
	let pos = get_search_index(&initial_bytes, input)?;
	sliced = &input[pos..];

	let mut pattern = tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0x41, 0x8B, 0x94, 0x00]),
		take(4usize),
		tag(&[0x89, 0x57]),
	));

	// since nom fails to find the pattern, if it matches the first ones partially, but the rest does not match, we need to try again, after removing the first bytes
	while sliced.len() > 37 {
		match pattern(sliced) {
			Ok((_, res)) => return Ok([res.0, res.1, res.2, res.3, res.4].concat()),
			Err(_) => {
				// we can safely skip the same amount of bytes that we search for, since we failed to find it
				sliced = &sliced[37..];
				let pos = get_search_index(&initial_bytes, sliced)?;
				sliced = &sliced[pos..];
			}
		}
	}

	Err(Error::new("pattern not found").into())
}

// 48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E ?? F3 0F 10 86 ?? ?? ?? ?? F3 0F 58 86 ?? ?? ?? ?? F3 0F 11 86 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E
pub fn find_lobby_status(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let mut sliced;
	let pos = get_search_index(&initial_bytes, input)?;
	sliced = &input[pos..];

	let mut pattern = tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0xE8]),
		take(4usize),
		tag(&[0x48, 0x8B, 0x4E]),
		take(1usize),
		tag(&[0xF3, 0x0F, 0x10, 0x86]),
		take(4usize),
		tag(&[0xF3, 0x0F, 0x58, 0x86]),
		take(4usize),
		tag(&[0xF3, 0x0F, 0x11, 0x86]),
		take(4usize),
		tag(&[0xE8]),
		take(4usize),
		tag(&[0x48, 0x8B, 0x4E]),
	));

	// since nom fails to find the pattern, if it matches the first ones partially, but the rest does not match, we need to try again, after removing the first bytes
	while sliced.len() > 37 {
		match pattern(sliced) {
			Ok((_, res)) => {
				return Ok([
					res.0, res.1, res.2, res.3, res.4, res.5, res.6, res.7, res.8, res.9, res.10,
					res.11, res.12, res.13, res.14,
				]
				.concat())
			}
			Err(_) => {
				// we can safely skip the same amount of bytes that we search for, since we failed to find it
				sliced = &sliced[37..];
				let pos = get_search_index(&initial_bytes, sliced)?;
				sliced = &sliced[pos..];
			}
		}
	}

	Err(Error::new("pattern not found").into())
}

// 45 6D 65 74 74 61
pub fn find_emetta(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	// TODO: potentially modify this to be faster, after fixing it in the first place
	let initial_bytes = [0x45, 0x6D, 0x65, 0x74, 0x74, 0x61];

	let sliced = match take_until::<_, _, nom::error::Error<&[u8]>>(&initial_bytes[..])(input) {
		Ok((res, _)) => res,
		Err(_) => return Err(Error::new("pattern not found").into()),
	};

	match tuple((tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),))(sliced) {
		Ok((_, res)) => Ok(res.0.to_vec()),
		Err(_) => Err(Error::new("pattern not found").into()),
	}
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 54 24 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 48 8B 5C 24 60 48 83 C4 50 5F C3
pub fn find_player_name_linux(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let mut sliced;
	let pos = get_search_index(&initial_bytes, input)?;
	sliced = &input[pos..];

	let mut pattern = tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0x48, 0x8D, 0x54, 0x24]),
		take(14usize),
		tag(&[
			0x48, 0x8B, 0x5C, 0x24, 0x60, 0x48, 0x83, 0xC4, 0x50, 0x5F, 0xC3,
		]),
	));

	// since nom fails to find the pattern, if it matches the first ones partially, but the rest does not match, we need to try again, after removing the first bytes
	while sliced.len() > 37 {
		match pattern(sliced) {
			Ok((_, res)) => return Ok([res.0, res.1, res.2, res.3, res.4].concat()),
			Err(_) => {
				// we can safely skip the same amount of bytes that we search for, since we failed to find it
				sliced = &sliced[37..];
				let pos = get_search_index(&initial_bytes, sliced)?;
				sliced = &sliced[pos..];
			}
		}
	}

	Err(Error::new("pattern not found").into())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_find_player_name() {
		let data = vec![
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x48, 0x8b, 0x0d, 0x0a, 0x0c, 0xba, 0x04, 0x48, 0x8d, 0x54, 0x24, 0x38, 0xc6, 0x44,
			0x24, 0x20, 0x00, 0xe8, 0x23, 0xfb, 0x4d, 0x01, 0x48, 0x8b, 0x5c, 0x24, 0x70, 0x48,
			0x8b, 0x7c, 0x24, 0x60, 0x48, 0x83, 0xc4, 0x68, 0xc3, 0x48, 0x63, 0x87, 0x58, 0x02,
			0x00, 0x00, 0x4c, 0x8d, 0x0d, 0xd6, 0xab, 0xac, 0x00, 0x4c, 0x8b, 0x47, 0x08, 0x8b,
			0x94, 0x87, 0x28, 0x02, 0x00, 0x00, 0xe8, 0x66,
		];

		if let Err(e) = find_player_name(&data) {
			panic!("error: {}", e);
		}
	}

	#[test]
	fn test_find_current_player_name() {
		let data = vec![
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x55, 0x00, 0x45, 0x31,
			0xC9, 0x41, 0x89, 0xC0, 0xE8, 0x00,
		];

		if let Err(e) = find_current_player_name(&data) {
			panic!("error: {}", e);
		}
	}

	#[test]
	fn test_find_player_damage() {
		let data = vec![
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48,
			0x8B, 0xD8, 0x48, 0x85, 0xC0, 0x75, 0x04, 0x33, 0xC9, 0x00,
		];

		if let Err(e) = find_player_damage(&data) {
			panic!("error: {}", e);
		}
	}

	#[test]
	fn test_find_monster() {
		let data = vec![
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0xB2, 0x01, 0xE8, 0x00, 0x00, 0x00,
			0x00, 0xC6, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x0D, 0x00,
		];

		if let Err(e) = find_monster(&data) {
			panic!("error: {}", e);
		}
	}

	#[test]
	fn test_find_player_buff() {
		let data = vec![
			0x00, 0x48, 0x8B, 0x05, 0x00, 0x00, 0x00, 0x00, 0x41, 0x8B, 0x94, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x89, 0x57, 0x00,
		];

		if let Err(e) = find_player_buff(&data) {
			panic!("error: {}", e);
		}
	}

	#[test]
	fn test_find_lobby_status() {
		let data = vec![
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x48,
			0x8B, 0x4E, 0x00, 0xF3, 0x0F, 0x10, 0x86, 0x00, 0x00, 0x00, 0x00, 0xF3, 0x0F, 0x58,
			0x86, 0x00, 0x00, 0x00, 0x00, 0xF3, 0x0F, 0x11, 0x86, 0x00, 0x00, 0x00, 0x00, 0xE8,
			0x00, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x4E, 0x00,
		];

		if let Err(e) = find_lobby_status(&data) {
			panic!("error: {}", e);
		}
	}

	#[test]
	fn test_find_emetta() {
		let data = vec![0x00, 0x45, 0x6D, 0x65, 0x74, 0x74, 0x61, 0x00];

		if let Err(e) = find_emetta(&data) {
			panic!("error: {}", e);
		}
	}

	#[test]
	fn test_find_player_name_linux() {
		let data = vec![
			0x00, 0x48, 0x8B, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x54, 0x24, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8B,
			0x5C, 0x24, 0x60, 0x48, 0x83, 0xC4, 0x50, 0x5F, 0xC3, 0x00,
		];

		if let Err(e) = find_player_name_linux(&data) {
			panic!("error: {}", e);
		}
	}
}
