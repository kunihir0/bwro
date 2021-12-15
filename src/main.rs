use seahorse::{error::FlagError, App, Command, Context, Flag, FlagType};
use std::{env, path::Path, panic, fs, io};


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
			get_files(p);
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
	Command::new("config")
		.description("configure program")
		.alias("c")
		.usage("bwro config [path...]")
		.action(path_action)
		.flag(
			Flag::new("path", FlagType::String)
				.description("path to directory")
				.alias("p")
		)
}

fn get_files(d: String) -> io::Result<()> {
	if Path::new(&d).exists() {
		/* populate folders into array same with files */
		for p in fs::read_dir(&d)? {
			let path = p?.path();
			let p_str = path.to_str();
			set_files(p_str.unwrap().to_string());
		}

	} else {
		panic!("doesn't exist");
	}
	Ok(())
}


fn set_files(file: String) {
	let mut files = Vec::new();
	files.push(file);
}
