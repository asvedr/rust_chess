use std::str::*;
use field::*;

#[derive(Debug)]
pub enum Color {
	White,
	Black
}

#[derive(Debug)]
pub enum Cmd {
	// check steps for fig
	MovesFor(i8),
	// make ai move for color
	MakeMove(Color),
	// check available figs for color
	ActiveFig(Color),
	// help window
	Help,
	JustShow,
	Error(String)
}

#[derive(Debug)]
pub enum DataFormat {
	Text,
	Json,
	Graphic
}

#[derive(Debug)]
pub struct Args {
	pub cmd          : Cmd,
	pub out_format   : Option<DataFormat>,
	pub use_file     : bool,
	pub input        : Option<String>,
	pub input_format : Option<DataFormat>
}
impl Args {
	fn new(c : Cmd, of : Option<DataFormat>, u : bool, i : Option<String>, _if : Option<DataFormat>) -> Args {
		Args{cmd : c, out_format : of, use_file : u, input : i, input_format : _if}
	}
	fn err(s : &str) -> Args {
		Args{cmd : Cmd::Error(format!("{}", s)), out_format : None, use_file : false, input : None, input_format : None}
	}
}

pub fn read_args(src : &Vec<String>) -> Args {
	match look_for(src, "h", false) {
		Either::Left(true) => {
			return Args::new(Cmd::Help, None, false, None, None)
		},
		_ => ()
	}

	// PARAMS

	let mut input = None;
	let mut is_file = false;
	match look_for(src, "ip", true) {
		Either::Right(path) => {
			input = Some(format!("{}",path));
			is_file = true;
		},
		_ => match look_for(src, "is", true) {
			Either::Right(data) =>
				input = Some(format!("{}",data)),
			_ => ()
		}
	}
	let i_format = 
		match look_for(src, "if", true) {
			Either::Right(fmt) =>
				match fmt {
					"js"  => Some(DataFormat::Json),
					"map" => Some(DataFormat::Graphic),
					"t"   => Some(DataFormat::Text),
					_     => return Args::err("incorrect format")
				},
			_ => None
		};
	let o_format =
		match look_for(src, "of", true) {
			Either::Right(fmt) =>
				match fmt {
					"js"  => Some(DataFormat::Json),
					"map" => Some(DataFormat::Graphic),
					"t"   => Some(DataFormat::Text),
					_     => return Args::err("incorrect format")
				},
			_ => None
		};

	// MODE
	
	let mut out = Args::new(Cmd::Help, o_format, is_file, input, i_format);

	match look_for(src, "af", true) {
		Either::Right(clr) => {
			let clr = match clr {
				"w" => Color::White,
				"b" => Color::Black,
				_   => return Args::err("incorrect color")
			};
			out.cmd = Cmd::ActiveFig(clr);
			return out;
		},
		_ =>
			match look_for(src, "sf", true) {
				Either::Right(cell) => {
					match str2cell(cell) {
						Ok(c) => {
							out.cmd = Cmd::MovesFor(c);
							return out;
						},
						_ => return Args::err("incorrect cell")
					}
				},
				_ =>
					match look_for(src, "m", true) {
						Either::Right(clr) => {
							let clr = match clr {
								"w" => Color::White,
								"b" => Color::Black,
								_   => return Args::err("incorrect color")
							};
							out.cmd = Cmd::MakeMove(clr);
							return out;
						},
						_ =>
							match look_for(src, "rp", false) {
								Either::Left(true) => {
									out.cmd = Cmd::JustShow;
									return out;
								},
								_ => ()
							}
					}
			}
	}

	return Args::err("action cmd not found")
}

enum Either<A,B> {
	Left(A),
	Right(B)
}

fn look_for<'a>(vec : &'a Vec<String>, key : &str, has_param : bool) -> Either<bool,&'a str> {
	let len = vec.len();
	for i in 0 .. len {
		let mut item = vec[i].chars();
		if item.next() == Some('-') {
			if item.eq(key.chars()) {
				if has_param {
					if i+1 < len {
						return Either::Right(&*vec[i+1])
					} else {
						return Either::Left(false)
					}
				}
				else {
					return Either::Left(true)
				}
			}
		}
	}
	return Either::Left(false)
}

