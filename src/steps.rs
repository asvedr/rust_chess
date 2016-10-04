use field::*;
use std::fmt;

/*
	STEP PREY CODES:
		0> prey is enemy. Value is an index in enemy vec.
		-1 no prey. Empty step.
*/
pub struct Step {
	//pub cell1 : i8,
	pub fig   : usize, // index of figure
	pub cell  : i8, // cell to put
	pub prey  : isize // enemy fig index or '-1'
}

impl fmt::Debug for Step {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<F:{} C:({},{}) P:{}>", self.fig, x!(self.cell), y!(self.cell), self.prey)
	}
}

type Cache = [isize;64];
/*
	0 - cell is free
	<0 - cell is friend
	>0 - cell is index in enemy list
*/

static mut cache : [isize;64] = [0;64];

struct Cell {
	x : i8,
	y : i8
}

macro_rules! c { ($x:expr, $y:expr) => (Cell{x:$x, y:$y}); }

macro_rules! linear_move {
	($f:expr, $cell:expr, $out:expr, $($way:expr),+ ) => {{
		let src_x = x!($cell);
		let src_y = y!($cell);
		$(
			let mut x = src_x;
			let mut y = src_y;
			let inc = $way;
			let px = inc.x;
			let py = inc.y;
			loop {
				x += px;
				y += py;
				if !(x >= 0 && x < 8 && y >= 0 && y < 8) {
					break;
				}
				let c = cell!(x,y);
				let f = cache[c as usize];
				if f == 0 {
					$out.push(Step{fig : $f, cell : c, prey : -1});
				} else if f > 0 {
					$out.push(Step{fig : $f, cell : c, prey : f});
					break;
				} else {
					break;
				}
			}
		)+
	}};
}

macro_rules! king_horse_move {
	($f:expr, $cell:expr, $out:expr, $($way:expr),+) => {{
		let x = x!($cell);
		let y = y!($cell);
		$({
			let inc = $way;
			let x = x + inc.x;
			let y = y + inc.y;
			if x >= 0 && x < 8 && y >= 0 && y < 8 {
				let c = cell!(x, y);
				let f = cache[c as usize];
				if f == 0 {
					$out.push(Step{fig : $f, cell : c, prey : -1});
				} else if f > 0 {
					$out.push(Step{fig : $f, cell : c, prey : f});
				}
			}
		})+
	}};
}

macro_rules! put {
	($f:expr, $cell:expr, $x:expr, $y:expr, $out:expr, $py:expr) => {{
		let c = cell!($x, $y + $py);
		if cache[c as usize] == 0 {
			$out.push(Step{fig : $f, cell : c, prey : -1});
		}
	}};
}
macro_rules! eat {
	($f:expr, $cell:expr, $x:expr, $y:expr, $out:expr, $px:expr, $py:expr) => {{
		let x = $x + $px;
		if x >= 0 && x < 8 {
			let c = cell!(x, $y + $py);
			let f = cache[c as usize];
			if f > 0 {
				$out.push(Step{fig : $f, cell : c, prey : f});
			}
		}
	}};
}
/* WARNING!!!
	- THERE IS NO BOUND CHECK FOR 'Y' ONLY 'X'
	- THERE IS NO CHECK TO TRANSFORM PAWN TO OTHER FIG
*/
macro_rules! pawn_move {
	($f:expr, $cell:expr, $out:expr, $init_y:expr, $py:expr) => {{
		let x = x!($cell);
		let y = y!($cell);
		if $init_y == y {
			put!($f, $cell,x,y,$out,$py * 2);
		} else {
			put!($f, $cell,x,y,$out,$py);
		}
		eat!($f, $cell,x,y,$out,1,$py);
		eat!($f, $cell,x,y,$out,-1,$py);
	}};
}

macro_rules! king_move {
	($f:expr, $c:expr, $o:expr) => {
		king_horse_move!($f, $c, $o,
			c!(1,-1), c!(1,0), c!(1,1),c!(0,-1), c!(0,0), c!(0,1), c!(-1,-1), c!(-1,0), c!(-1,1));
	};
}

macro_rules! horse_move {
	($f:expr, $c:expr, $o:expr) => {
		king_horse_move!($f, $c, $o,
			c!(2,1),c!(2,-1),c!(-2,1),c!(-2,-1),c!(1,2),c!(-1,2),c!(1,-2),c!(-1,-2));
	};
}

pub fn add_steps(field : &Field, use_white : bool, out : &mut Vec<Step>) {
	let friends : &Vec<Fig>;
	let enemies : &Vec<Fig>;
	let init_py : i8;
	let ppy     : i8;
	if use_white {
		friends = &field.white;
		enemies = &field.black;
		init_py = 1;
		ppy = 1;
	} else {
		friends = &field.black;
		enemies = &field.white;
		init_py = 6;
		ppy = -1;
	}
	// CACHING
	unsafe {
		for i in 0 .. 64 {
			cache[i] = 0;
		}
		for i in 0 .. friends.len() {
			cache[friends[i].cell as usize] = -1;
		}
		for i in 0 .. enemies.len() {
			cache[enemies[i].cell as usize] = i as isize;
		}
		// CALCULATING
		for i in 0 .. friends.len() {
			let fig = &friends[i];
			match fig.kind {
				PAWN   => pawn_move!(i, fig.cell, out, init_py, ppy),
				BISHOP => linear_move!(i, fig.cell, out, c!(1,1), c!(1,-1), c!(-1,1), c!(-1,-1)),
				TOWER  => linear_move!(i, fig.cell, out, c!(0,1), c!(0,-1), c!(1,0), c!(-1,0)),
				HORSE  => horse_move!(i, fig.cell, out),
				QWEEN  => linear_move!(i, fig.cell, out, c!(0,1), c!(0,-1), c!(1,0), c!(-1,0), c!(1,1), c!(1,-1), c!(-1,1), c!(-1,-1)),
				KING   => king_move!(i, fig.cell, out),
				_ => ()
			}
		}
	}
}
