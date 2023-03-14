use atty::Stream;
use std::io;

pub fn get_stdin() -> Option<Vec<String>> {
	if atty::isnt(Stream::Stdin) {
		Some(
			io::stdin()
				.lines()
				.flat_map(|i| {
					i.unwrap()
						.split(' ')
						.map(|i| i.to_owned())
						.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>(),
		)
	} else {
		None
	}
}
