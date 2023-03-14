use atty::Stream;
use std::io;

pub fn get_stdin() -> Option<Vec<String>> {
	if atty::isnt(Stream::Stdin) {
		Some(
			io::stdin()
				.lines()
				.into_iter()
				.map(|i| {
					i.unwrap()
						.split(" ")
						.into_iter()
						.map(|i| i.to_owned())
						.collect::<Vec<_>>()
				})
				.flatten()
				.collect::<Vec<_>>(),
		)
	} else {
		None
	}
}
