use nom::{
	bytes::complete::{tag, take, take_until},
	sequence::tuple,
};

use crate::err::Error;

// 48 8B 0D ?? ?? ?? ?? 48 8D 54 24 38 C6 44 24 20 00 E8 ?? ?? ?? ?? 48 8B 5C 24 70 48 8B 7C 24 60 48 83 C4 68 C3
pub fn find_player_name(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let sliced = match take_until::<_, _, nom::error::Error<&[u8]>>(&initial_bytes[..])(input) {
		Ok((res, _)) => res,
		Err(_) => return Err(Error::new("pattern not found").into()),
	};

	match tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0xE8]),
		take(4usize),
		tag(&[0x48, 0x8B, 0xD8, 0x48, 0x85, 0xC0, 0x75, 0x04, 0x33, 0xC9]),
	))(sliced)
	{
		Ok((_, res)) => Ok([res.0, res.1, res.2, res.3, res.4].concat()),
		Err(_) => Err(Error::new("pattern not found").into()),
	}
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 55 ?? 45 31 C9 41 89 C0 E8
pub fn find_current_player_name(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let sliced = match take_until::<_, _, nom::error::Error<&[u8]>>(&initial_bytes[..])(input) {
		Ok((res, _)) => res,
		Err(_) => return Err(Error::new("pattern not found").into()),
	};

	match tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0x48, 0x8D, 0x55]),
		take(1usize),
		tag(&[0x45, 0x31, 0xC9, 0x41, 0x89, 0xC0, 0xE8]),
	))(sliced)
	{
		Ok((_, res)) => Ok([res.0, res.1, res.2, res.3, res.4].concat()),
		Err(_) => Err(Error::new("pattern not found").into()),
	}
}

// 48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B D8 48 85 C0 75 04 33 C9
pub fn find_player_damage(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let sliced = match take_until::<_, _, nom::error::Error<&[u8]>>(&initial_bytes[..])(input) {
		Ok((res, _)) => res,
		Err(_) => return Err(Error::new("pattern not found").into()),
	};

	match tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0xE8]),
		take(4usize),
		tag(&[0x48, 0x8B, 0xD8, 0x48, 0x85, 0xC0, 0x75, 0x04, 0x33, 0xC9]),
	))(sliced)
	{
		Ok((_, res)) => Ok([res.0, res.1, res.2, res.3, res.4].concat()),
		Err(_) => Err(Error::new("pattern not found").into()),
	}
}

// 48 8B 0D ?? ?? ?? ?? B2 01 E8 ?? ?? ?? ?? C6 83 ?? ?? ?? ?? ?? 48 8B 0D
pub fn find_monster(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let initial_bytes = [0x48, 0x8B, 0x0D];

	let sliced = match take_until::<_, _, nom::error::Error<&[u8]>>(&initial_bytes[..])(input) {
		Ok((res, _)) => res,
		Err(_) => return Err(Error::new("pattern not found").into()),
	};

	match tuple((
		tag::<_, _, nom::error::Error<&[u8]>>(initial_bytes),
		take(4usize),
		tag(&[0xB2, 0x01, 0xE8]),
		take(4usize),
		tag(&[0xC6, 0x83]),
		take(5usize),
		tag(&[0x48, 0x8B, 0x0D]),
	))(sliced)
	{
		Ok((_, res)) => Ok([res.0, res.1, res.2, res.3, res.4].concat()),
		Err(_) => Err(Error::new("pattern not found").into()),
	}
}

// 48 8B 05 ?? ?? ?? ?? 41 8B 94 00 ?? ?? ?? ?? 89 57
pub fn find_player_buff(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	todo!("find_player_buff");
}

// 48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E ?? F3 0F 10 86 ?? ?? ?? ?? F3 0F 58 86 ?? ?? ?? ?? F3 0F 11 86 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E
pub fn find_lobby_status(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	todo!("find_lobby_status");
}

// 45 6D 65 74 74 61
pub fn find_emetta(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	todo!("find_emetta");
}

// 48 8B 0D ?? ?? ?? ?? 48 8D 54 24 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 48 8B 5C 24 60 48 83 C4 50 5F C3
pub fn find_player_name_linux(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	todo!("find_player_name_linux");
}
