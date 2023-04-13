use cmdline::CmdLine;

fn main() {
	let mut cmdline = CmdLine::from_env_args();
	
	println!("program: {}", cmdline.program());
	
	while cmdline.next() {
		if cmdline.is_opt("verbose", "v") {
			println!("--verbose");
		} else if let Some(cmd) = cmdline.is_arg_n(0) {
			println!("cmd: {}", cmd);
		} else if let Some(arg) = cmdline.is_arg() {
			println!("arg[{}] = {}", cmdline.arg_idx(), arg);
		} else if let Some(val) = cmdline.is_opt_with_val("file", "f") {
			println!("--file={}", val);
		}
	}
}
