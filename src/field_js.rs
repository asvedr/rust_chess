use field::*;
use std::collections::BTreeMap;
use rustc_serialize::json::*;

pub fn field2json(fld : &Field) -> Json {
	let mut white = vec![];
	let mut black = vec![];
	macro_rules! addf {($fig:expr, $dest:expr) => {
		let kind = Json::String(kind_to_s($fig.kind));
		let cell = Json::String(cell2str($fig.cell));
		$dest.push(Json::Array(vec![kind, cell]));
	};}
	for fig in fld.white.iter() {
		addf!(fig, white);
	}
	for fig in fld.black.iter() {
		addf!(fig, black);
	}
	let mut obj = BTreeMap::new();
	obj.insert("white".to_string(), Json::Array(white));
	obj.insert("black".to_string(), Json::Array(black));
	return Json::Object(obj);
}

pub fn json2field(source : &str) -> Result<Field, String> {
	macro_rules! errf {($a:expr) => {return Err($a.to_string())}; }
	macro_rules! tryv {($a:expr) => {match $a {Ok(a) => a, Err(a) => errf!(a)}};
					   ($a:expr, $v:expr) => {match $a {Ok(a) => a, Err(_) => errf!($v)}}; }
	macro_rules! geto {($a:expr, $v:expr) => {match $a {Some(a) => a, None => errf!($v)}}; }
	let js = tryv!(Json::from_str(source));
	let cont  = geto!(js.as_object(), "incorrect json");
	let white = geto!(cont.get("white"), "white not found");
	let black = geto!(cont.get("black"), "black not found");

	let mut field = Field{white : vec![], black : vec![]};

	macro_rules! put_fig {($fig:expr, $dest:expr) => {
		let fig = geto!($fig.as_array(), "figure isn't array");
		if fig.len() != 2
			{ errf!("figure len isn't 2") }
		let kind = tryv!(s_to_kind_safe(geto!(fig[0].as_string(), "figure kind isn't string")), "incorrect kind");
		let cell = tryv!(str2cell(geto!(fig[1].as_string(), "figure cell isn't string")), "incorrect cell");
		$dest.push(Fig{kind : kind, cell : cell});
	};}

	let white = geto!(white.as_array(), "white isn't array");
	let black = geto!(black.as_array(), "black isn't array");

	for fig in white.iter() {
		put_fig!(fig, field.white);
	}
	for fig in black.iter() {
		put_fig!(fig, field.black);
	}

	return Ok(field);
}
