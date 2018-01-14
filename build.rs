fn main() {
	use std::path::Path;
	use std::collections::HashMap;
	use std::fs;
	use std::io::Read;
	use std::borrow::Cow;

	let paths = fs::read_dir("./dictionaries").unwrap();
	let mut file_data = HashMap::<String, String>::new();

    for path in paths {
    	let path = path.unwrap().path();
    	let file_name = String::from(path.file_stem().unwrap().to_string_lossy());
    	let mut read_str = String::new();
    	let mut file = fs::File::open(path.canonicalize().unwrap()).unwrap();
    	file.read_to_string(&mut read_str).expect(&format!("no valid unicode for: {:?}", file_name));
    	file_data.insert(file_name, read_str);
    }
}