use seahorse::{error::FlagError, App, Command, Context, Flag, FlagType};
use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let app = App::new("Better Wallpaper Reoraganizer")
	.author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
	.usage("bwro [arg]")
	.version(env!("CARGO_PKG_VERSION"))
	.command(path_command());

	app.run(args);
}

fn path_action(c: &Context) {
	/* get path value */
	match c.string_flag("path") {
		Ok(p) => {
			print!("f");
		}
		Err(e) => match e {
			FlagError::Undefined => panic!("path undefined..."),
			FlagError::ArgumentError => panic!("argument error..."),
			FlagError::NotFound => panic!("not found flag..."),
			FlagError::ValueTypeError => panic!("value type mismatch..."),
			FlagError::TypeError => panic!("flag type mismatch..,")
		},
	}
}

fn path_command() -> Command {
	Command::new("path")
		.description("Path to folder")
		.alias("p")
		.usage("bwro -p [path]")
		.action(path_action)
}