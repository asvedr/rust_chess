use std::str::{Chars,FromStr};

#[derive(Clone)]
pub struct Fig {
	pub kind : u8,
	pub cell : i8
}

pub struct Field {
	pub white : Vec<Fig>,
	pub black : Vec<Fig>
}

macro_rules! fig_crd {
	($fig:expr, $x:expr, $y:expr) => {{
		$y = $fig.cell / 8;
		$x = $fig.cell % 8;
	}};
	($fig:expr) => {
		Cell{y : $fig.cell / 8, x : $fig.cell % 8}
	};
}

#[macro_export]
macro_rules! x { ($c:expr) => {$c % 8}; }
#[macro_export]
macro_rules! y { ($c:expr) => {$c / 8}; }
#[macro_export]
macro_rules! cell { ($x:expr, $y:expr) => {(($y as i8) * 8) + ($x as i8)}; }
#[macro_export]
macro_rules! is_black { ($x:expr, $y:expr) => {$x % 2 == $y % 2};
                        ($c:expr) => {($c / 8) % 2 == ($c % 8) % 2}; }

pub fn cell2str(c : i8) -> String {
	let letters = vec!["A","B","C","D","E","F","G","H"];
	return format!("{}{}", letters[x!(c) as usize], y!(c) + 1);
}

pub fn str2cell(c : &str) -> Result<i8,()> {
	let s : Vec<char> = c.chars().collect();
	if s.len() != 2
		{ return Err(()) }
	let xc = s[0];
	let yc = s[1];
	let letters_s = vec!['a','b','c','d','e','f','g','h'];
	let letters_g = vec!['A','B','C','D','E','F','G','H'];
	for x in 0 .. 8 {
		if letters_s[x] == xc || letters_g[x] == xc {
			match usize::from_str(&*format!("{}",yc)) {
				Ok(y) => return Ok(cell!(x,y)),
				_ => return Err(())
			}
		}
	}
	return Err(())
}

const BLACK : &'static [&'static str] = &["k","q","b","h","t","p"];
const WHITE : &'static [&'static str] = &["K","Q","B","H","T","P"];

pub const KING   : u8 = 0;
pub const QWEEN  : u8 = 1;
pub const BISHOP : u8 = 2;
pub const HORSE  : u8 = 3;
pub const TOWER  : u8 = 4;
pub const PAWN   : u8 = 5;

fn s_to_kind(s : &str) -> u8 {
	for i in 0 .. 6 {
		if BLACK[i] == s {
			return i as u8
		}
	}
	panic!()
}

impl Fig {
	pub fn x(&self) -> i8 {
		self.cell / 8
	}
	pub fn y(&self) -> i8 {
		self.cell / 8
	}
	pub fn crd(&self, x : &mut i8 , y : &mut i8) {
		fig_crd!(self, *x, *y);
	}
}

impl Field {
	pub fn new() -> Field {
		let mut white = vec![];
		let mut black = vec![];
		// PAWNS
		let p = s_to_kind("p");
		for x in 0 .. 8 {
			white.push(Fig{kind : p, cell : cell!(x, 1)});
			black.push(Fig{kind : p, cell : cell!(x, 6)});
		}
		// MAIN FIGS
		let figs = vec!["t","h","b","q","k","b","h","t"];
		for x in 0 .. 8 {
			white.push(Fig{kind : s_to_kind(figs[x]), cell : cell!(x,0)});
			black.push(Fig{kind : s_to_kind(figs[x]), cell : cell!(x,7)});
		}
		Field {
			white : white,
			black : black
		}
	}
	//pub fn read_text()
	pub fn step_black(&self, cell_in : i8, cell_out : i8) -> Field {
		let mut black = self.black.clone();
		let mut white = vec![];
		for fig in black.iter_mut() {
			if fig.cell == cell_in {
				fig.cell = cell_out;
				break;
			}
		}
		for fig in self.white.iter() {
			if fig.cell != cell_out {
				white.push(fig.clone());
			}
		}
		Field{black : black, white : white}
	}
	pub fn step_white(&self, cell_in : i8, cell_out : i8) -> Field {
		let mut white = self.white.clone();
		let mut black = vec![];
		for fig in white.iter_mut() {
			if fig.cell == cell_in {
				fig.cell = cell_out;
				break;
			}
		}
		for fig in self.black.iter() {
			if fig.cell != cell_out {
				black.push(fig.clone());
			}
		}
		Field{white : white, black : black}
	}
	pub fn print(&self) {
		let mut line = String::new();
		for y in 0 .. 8 {
			for x in 0 .. 8 {
				let cell = cell!(x,y);
				let mut found = false;
				for c in self.white.iter() {
					if c.cell == cell {
						line.push_str(WHITE[c.kind as usize]);
						found = true;
					}
				}
				if !found {
					for c in self.black.iter() {
						if c.cell == cell {
							line.push_str(BLACK[c.kind as usize]);
							found = true;
						}
					}
				}
				if !found {
					if is_black!(x,y) {
						line.push('.');
					} else {
						line.push('_');
					}
				}
			}
			println!("{}",line);
			line.clear();
		}
	}
}
