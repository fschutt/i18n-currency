use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};

pub struct LanguageDictionary {
	/// "BYN" => { ""BYN", "Weissrussischer Rubel", ... }
	pub currencies: HashMap<String, TranslatedCurrency>,
}

pub struct TranslatedCurrency {
	/// "BYN"
	pub shortcode: String,
	/// "Weissrussischer Rubel"
	pub country: String,
	/// "$", "¥", etc.
	pub currency_symbol: Option<String>,
	/// Alternative currency symbol,
	pub currency_symbol_variant: Option<String>,
	/// "Weissrussische Rubel"
	pub plural: Option<String>,
	/// Ordering, $DOLLAR or DOLLAR$, etc.
	pub currency_unit_pattern: Option<String>,
}

fn main() {
	let paths = fs::read_dir("./dictionaries").unwrap();
	let mut dictionaries = HashMap::<(String, Option<String>), LanguageDictionary>::new();

    for path in paths {
    	let path = path.unwrap().path();
    	let file_name = String::from(path.file_stem().unwrap().to_string_lossy());
    	let mut file_items_iterator = file_name.split("_");
    	let first_item = file_items_iterator.next().unwrap();
    	let second_item = file_items_iterator.next();
    	let second_item = if let Some(a) = second_item { Some(a.to_string()) } else { None };
    	let mut read_str = String::new();
    	let mut file = fs::File::open(path.canonicalize().unwrap()).unwrap();
    	file.read_to_string(&mut read_str).expect(&format!("no valid unicode for input file: {:?}", file_name));
    	if let Some(dict) = parse_makefile(&read_str, &file_name) {
    		dictionaries.insert((first_item.to_string(), second_item), dict);
    	}
    }

    // dictionaries now contains: <("en", Some("CX")), "file contents">

    let mut lib_rs = fs::File::create("src/lib.rs").unwrap();

    let mut lib_rs_contents = String::from(include_str!("prefix.rs"));
    lib_rs_contents.push_str("pub enum Language {\r\n");

    for (language, file_contents) in &dictionaries {
    	let second_string = match language.1 {
    		Some(ref lang) => format!("_{}", lang.to_uppercase()),
    		None => String::new(),
    	};
    	lib_rs_contents.push_str(&format!("    {}{},\n", language.0.to_uppercase(), second_string));
    }
	lib_rs_contents.push_str("}\n\n");

	lib_rs_contents.push_str("/// Translates the currency into the target language.\n");
	lib_rs_contents.push_str("/// If there is no translation, returns `None`.\n");
	lib_rs_contents.push_str("pub fn translate_currency(currency: Currency, language: Language) -> Option<&'static str> {\n");
	lib_rs_contents.push_str("    use self::Language::*;\n");
	lib_rs_contents.push_str("    match language {\n");
	for (language, file_contents) in &dictionaries {
		let second_string = match language.1 {
			Some(ref lang) => format!("_{}", lang.to_uppercase()),
			None => String::new(),
		};
		lib_rs_contents.push_str(&format!("        {}{} => None,\n", language.0.to_uppercase(), second_string));
	}
	lib_rs_contents.push_str("    }\n");
	lib_rs_contents.push_str("}\n");

    lib_rs.write_all(&lib_rs_contents.as_bytes()).unwrap();
}

fn parse_makefile(input: &str, file_name_debug: &str) -> Option<LanguageDictionary> {
	println!("parsing file: {:?}", file_name_debug);
	let mut currencies = HashMap::new();

	// stores the indents, like "af", "Currency", "one"
	let mut indent_stack = Vec::<String>::new();
	let mut cur_indent_lv = 0;

	let mut cur_read_buf = vec![String::new()];
	let mut item_enum = 0;

	for line in input.lines() {
		let line = line.trim();

		if line.starts_with("//") ||
		   line.starts_with("\u{feff}//") {
		   	continue;
		}

		for ch in line.chars() {
			match ch {
				'"' => {
					continue;
				},
				'{' => {
					// push stack
					cur_indent_lv += 1;
					indent_stack.push(cur_read_buf[0].clone());
					cur_read_buf = vec![String::new()];
				},
				'}' => {
					// pop stack, decide action
					item_enum = 0;
					cur_indent_lv -= 1;
				},
				',' => {
					item_enum += 1;
					cur_read_buf.push(String::new());
				},
				_ => {
					cur_read_buf[item_enum].push(ch);
				}
			}
		}
	}

	if currencies.is_empty() {
		None
	} else {
		Some(LanguageDictionary {
			currencies: currencies,
		})
	}
}

/*if let Some(indent) = indent_stack.get(cur_indent_lv - 1) {
	match indent.as_ref() {
		"Currencies" => {

		},
		"Currencies%narrow" => {

		},
		"Currencies%variant" => {

		},
		"CurrencyPlurals" => {

		},
		"CurrencyUnitPatterns" => {

		},
		_ => { }
	}
}*/