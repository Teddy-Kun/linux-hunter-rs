use crate::err::Error;

#[derive(Debug)]
pub struct Pattern<'a> {
	bytes: &'a str,
	name: &'a str,
}

#[derive(Debug)]
pub struct MemoryPattern {
	pub bytes: Vec<u8>,
	pub matches: Vec<OffLen>,
	pub name: String,
	pub mem_location: isize,
}

impl MemoryPattern {
	pub fn pattern() -> Self {
		MemoryPattern {
			bytes: vec![],
			matches: vec![],
			name: "".to_string(),
			mem_location: 0,
		}
	}

	pub fn new(p: &Pattern) -> Result<MemoryPattern, Error> {
		let mut tgt_offset: usize = 0;
		let mut reset_chunk = true;

		let mut bytes = Vec::new();
		let mut matches = Vec::new();

		for b in p.bytes.split_whitespace() {
			if b == "??" {
				tgt_offset += 1;
				continue;
			}

			match u8::from_str_radix(b, 16) {
				Ok(v) => {
					bytes.push(v);
				}
				Err(e) => {
					return Err(Error::new(&e.to_string()));
				}
			}

			if reset_chunk {
				reset_chunk = false;
				matches.push(OffLen {
					src_offset: bytes.len() - 1,
					tgt_offset,
					len: 0,
				});
				tgt_offset += 1;
			}
		}

		Ok(MemoryPattern {
			bytes,
			matches,
			name: p.name.to_string(),
			mem_location: -1,
		})
	}
}

#[derive(Debug)]
pub struct OffLen {
	pub src_offset: usize,
	pub tgt_offset: usize,
	pub len: usize,
}

pub const PLAYER_NAME: Pattern = Pattern {
	bytes: "48 8B 0D ?? ?? ?? ?? 48 8D 54 24 38 C6 44 24 20 00 E8 ?? ?? ?? ?? 48 8B 5C 24 70 48 8B 7C 24 60 48 83 C4 68 C3",
	name: "PlayerName",
};

pub const CURRENT_PLAYER_NAME: Pattern = Pattern {
	bytes: "48 8B 0D ?? ?? ?? ?? 48 8D 55 ?? 45 31 C9 41 89 C0 E8",
	name: "CurrentPlayerName",
};

pub const PLAYER_DAMAGE: Pattern = Pattern {
	bytes: "48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B D8 48 85 C0 75 04 33 C9",
	name: "PlayerDamage",
};

pub const MONSTER: Pattern = Pattern {
	bytes: "48 8B 0D ?? ?? ?? ?? B2 01 E8 ?? ?? ?? ?? C6 83 ?? ?? ?? ?? ?? 48 8B 0D",
	name: "Monster",
};

pub const PLAYER_BUFF: Pattern = Pattern {
	bytes: "48 8B 05 ?? ?? ?? ?? 41 8B 94 00 ?? ?? ?? ?? 89 57",
	name: "PlayerBuff",
};

pub const LOBBY_STATUS: Pattern = Pattern {
	bytes: "48 8B 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E ?? F3 0F 10 86 ?? ?? ?? ?? F3 0F 58 86 ?? ?? ?? ?? F3 0F 11 86 ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 4E",
	name: "LobbyStatus",
};

pub const EMETTA: Pattern = Pattern {
	bytes: "45 6D 65 74 74 61",
	name: "Emetta",
};

pub const PLAYER_NAME_LINUX: Pattern = Pattern {
	bytes:
		"48 8B 0D ?? ?? ?? ?? 48 8D 54 24 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 48 8B 5C 24 60 48 83 C4 50 5F C3",
	name: "PlayerNameLinux",
};
