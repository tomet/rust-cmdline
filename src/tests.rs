use super::CmdLine;

struct Opts {
	verbose: bool,
	file: Option<String>,
	cmd: Option<String>,
	args: Vec<String>,
	result: Result<(), String>
}

#[test]
fn args_and_long_opts() {
	let opts = parse_cmdline("cmdline --verbose cmd --file=myfile.txt arg1 arg2");
	assert_eq!(opts.result, Ok(()));
	assert_eq!(opts.file, Some("myfile.txt".to_string()));
	assert_eq!(opts.cmd, Some("cmd".to_string()));
	assert_eq!(opts.args.len(), 2);
	assert_eq!(&opts.args[0], "arg1");
	assert_eq!(&opts.args[1], "arg2");
	assert_eq!(opts.verbose, true);
}

#[test]
fn only_short_opts() {
	let opts = parse_cmdline("cmdline -v -f myfile.txt");
	assert_eq!(opts.result, Ok(()));
	assert_eq!(opts.file, Some("myfile.txt".to_string()));
	assert_eq!(opts.args.len(), 0);
	assert_eq!(opts.verbose, true);
}

#[test]
fn only_args_left() {
	let opts = parse_cmdline("cmdline --verbose -- cmd --file=file.txt");
	assert_eq!(opts.result, Ok(()));
	assert_eq!(opts.file, None);
	assert_eq!(opts.cmd, Some("cmd".to_string()));
	assert_eq!(opts.args.len(), 1);
	assert_eq!(&opts.args[0], "--file=file.txt");
	assert_eq!(opts.verbose, true);
}

#[test]
fn unexpected_opt_val() {
	let opts = parse_cmdline("/usr/bin/cmdline --verbose=file.txt");
	assert_eq!(opts.result, Err("Option erlaubt kein Argument: --verbose".to_string()));
}

#[test]
fn empty_opt_name() {
	let opts = parse_cmdline("/usr/bin/cmdline --=file.txt");
	assert_eq!(opts.result, Err("Option ohne Name ist nicht erlaubt!".to_string()));
}

#[test]
fn unknown_opt() {
	let opts = parse_cmdline("cmdline --verbose --unknown");
	assert_eq!(opts.result, Err("Unbekannte Option: --unknown".to_string()));
}

#[test]
fn missing_opt_val() {
	let opts = parse_cmdline("cmdline --file --verbose");
	assert_eq!(opts.result, Err("Fehlendes Argument für Option: --file".to_string()));
}

fn parse_cmdline(s: &str) -> Opts {
	let mut opts = Opts {
		verbose: false,
		file: None,
		cmd: None,
		args: vec![],
		result: Ok(())
	};
	
	let mut cmdline = CmdLine::from_str(s);
	
	assert_eq!(cmdline.program(), "cmdline");
	
	cmdline.set_exit_on_error(false);
	
	while cmdline.next() {
		if cmdline.is_opt("verbose", "v") {
			opts.verbose = true;
		} else if let Some(arg) = cmdline.is_arg_n(0) {
			opts.cmd = Some(arg);
		} else if let Some(arg) = cmdline.is_arg() {
			opts.args.push(arg);
		} else if let Some(file) = cmdline.is_opt_with_val("file", "f") {
			opts.file = Some(file);
		}
	}
	
	opts.result = cmdline.result();
	
	opts
}

