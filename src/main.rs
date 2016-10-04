#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rustc_serialize;

#[macro_use]
mod field;
mod steps;
mod cmdui;
mod field_js;
use field::*;
use std::env;
use std::io;
use std::io::{Result, Read};
use std::fs::File;
use steps::*;

fn rand_in<A>(v : &Vec<A>) -> &A {
	return &v[v.len() / 2]
}

fn help() {
	let lines = vec![
		"-h               - this message",
		"-ip <file>       - file with game map",
		"-is <data>       - source string data",
		"-if <(js|map|t)> - input format. Default : t",
		"-of <(js|map|t)> - data format in file. Default : t",
		"--- MODES ---",
		"-af <(w|b)>      - active figures for color",
		"-sf <cell>       - steps for figure in cell. Sample: '-sf d6'",
		"-m  <(w|b)>      - make AI move for color",
		"-rp              - just show figures"
	];
	for line in lines {
		println!("{}", line);
	}
}

fn read_file(s : &str) -> Result<String> {
	let mut file = try!(File::open(s));
	let mut res = String::new();
	try!(file.read_to_string(&mut res));
	return Ok(res);
}

fn main() {
	let args : Vec<String> = env::args().collect();
	if args.len() < 1 {
		println!("need args");
		return;
	}
	let args = cmdui::read_args(&args);
	// fill field
	let field = match args.input {
		None => Field::new(),
		Some(data) => {
			let data =
				if args.use_file {
					match read_file(&*data) {
						Err(e) => {
							println!("{}\"error\": \"can't read file\"{}",'{','}');
							return;
						},
						Ok(val) => val
					}
				} else
					{ data };
			let f = match args.input_format {
						Some(f) => f,
						None => cmdui::DataFormat::Text
					};
			match f {
				cmdui::DataFormat::Text => panic!(),
				cmdui::DataFormat::Json =>
					match field_js::json2field(&data) {
						Ok(f) => f,
						Err(e) => {
							println!("{}\"error\": \"{}\"{}", '{', e, '}');
							return;
						}
					},
				_ => {
					println!("input must be text or json");
					return;
				}
			}
		}
	};
	// make result
	match args.cmd {
		cmdui::Cmd::Help =>
			help(),
		cmdui::Cmd::JustShow => {
			match args.out_format {
				None => field.print(),
				Some(cmdui::DataFormat::Graphic) => field.print(),
				Some(cmdui::DataFormat::Json) => println!("{}", field_js::field2json(&field)),
				_ => panic!()
			}
		},
		cmdui::Cmd::MovesFor(cell) => {
			let mut steps = vec![];
			let mut found = None;
			let mut kind = 0;
			let mut is_white = false;
			for i in 0 .. field.white.len() {
				if field.white[i].cell == cell {
					found = Some(i);
					kind = field.white[i].kind;
					is_white = true;
					break;
				}
			}
			for i in  0 .. field.black.len() {
				if field.black[i].cell == cell {
					found = Some(i);
					kind = field.black[i].kind;
					break;
				}
			}
			match found {
				None => {
					println!("cell is empty");
					return;
				},
				Some(i) => {		
					add_steps(&field, is_white, &mut steps);
					// ADD PAWN AT END CHECK HERE
					let ci = found.unwrap();
					for step in steps {
						if step.fig == ci {
							if kind == PAWN && (is_white && step.cell == 7) || (!is_white && step.cell == 0) {
								println!("[{}, {}, true]", step.cell, step.prey);
							}
							println!("[{}, {}, []]", step.cell, step.prey);
						}
					}
				}
			}
		},
		cmdui::Cmd::ActiveFig(clr) => panic!(),
		cmdui::Cmd::MakeMove(clr) => panic!(),
		cmdui::Cmd::Error(e) =>
			println!("{}\"error\": \"{}\"{}", '{', e, '}')
	}
}
