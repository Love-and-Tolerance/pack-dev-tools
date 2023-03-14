use pdtlib::optimize_images::optimize_images;
use pdtlib::pdtstdin;
use std::env;

fn main() {
	let dir = env::args().collect::<Vec<String>>()[1].to_string();
	let stdin = pdtstdin::get_stdin().unwrap_or(["test".to_string()].to_vec());

	for thing in stdin {
		println!("{thing}");
	}
	//optimize_images(dir);
}

// arg options
// -l --level (0 to 6) default 6
// -s --strip (none, safe, all) default safe.
// -f --fix (true or false) default true
// -i --interlace (true or false) default true
// everything after the arguments should be a list of files/folders
