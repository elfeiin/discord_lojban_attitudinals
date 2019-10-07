use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
	
	let vert: [&str; 7] = [
		"u",
		"i",
		".u'",
		".i'",
		".a'",
		".e'",
		".o'",
	];
	
	let horz: [&str; 5] = [
		"a",
		"e",
		"i",
		"o",
		"u",
	];
	
	let mut fiel = File::open("input.html")?;
	let mut text = String::new();
	fiel.read_to_string(&mut text)?;
	
	let parts: Vec<&str> = text.split("|||").collect();
	
	if parts.len() == 3 {
	
		let mut first = parts[0].to_string();
		let mut mid = parts[1].to_string();
		let last = parts[2];
		
		for v in vert.iter() {
			for h in horz.iter() {
				let id = format!("{}{}", v, h).replace(".", "_").replace("'", "h");
				first = put(&first, id.clone(), v, h);
				mid = format!("{}\n			'{}',", mid, id);
			}
		}
		
		
		first = put(&first, String::from("cuhi"), "cu'i", "");
		mid = format!("{}\n			'cuhi',", mid);
		first = put(&first, String::from("nai"), "nai", "");
		mid = format!("{}\n			'nai'\n		", mid);
		
		let together = format!("{}{}{}", first, mid, last);
		
		create(String::from("output.html"), together)?;
		
	}
	
	Ok(())
	
}

fn create(name: String, contents: String) -> std::io::Result<()> {

	let mut file = File::create(name)?;
	file.write_all(contents.as_bytes())?;
	
	Ok(())
	
}

fn put(text: &String, id: String, v: &str, h: &str) -> String {
	format!("{}\n		<div id='{}' class='outer'><div class='mid'><font class='inner'>{}{}</font></div></div>\n<br>", text, id, v, h)
}