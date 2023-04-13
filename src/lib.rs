use std::path::Path;
use std::process;
use std::env;

#[cfg(test)]
mod tests;

pub struct CmdLine {
	program: String,
	help: String,
	rest: Vec<String>,
	arg: Arg,
	arg_idx: i32,
	only_args_left: bool,
	exit_on_error: bool,
	result: Result<(), String>
}

impl CmdLine {
	
	pub fn from_env_args() -> Self {
		Self::from_args(env::args().collect())
	}
	
	pub fn from_str(s: &str) -> Self {
		let args = s
			.split(" ")
			.map(|s| s.to_string() )
			.collect();
		
		Self::from_args(args)
	}
	
	pub fn from_args(args: Vec<String>) -> Self {
		let mut rest = args;
		
		// Argumente umdrehen, damit rest.pop() immer
		// das nächste Argument liefert!
		rest.reverse();
		
		let executable = rest
			.pop()
			.expect("Mindestens ein Argument (Programm-Pfad) erforderlich!")
			.trim()
			.to_string();
		
		let program = Path::new(&executable)
			.file_name()
			.expect("Ungültiges erstes Argument: Muß Pfad zum Executable sein!")
			.to_str()
			.expect("UTF8-Fehler")
			.to_string();
		
		Self {
			help: format!("Verwendung: {} OPTIONEN", program),
			program,
			rest,
			arg: Arg::None,
			arg_idx: -1,
			only_args_left: false,
			exit_on_error: true,
			result: Ok(()),
		}
	}
	
	pub fn next(&mut self) -> bool {
		if self.result.is_err() {
			self.maybe_exit_on_error();
			return false;
		}
		
		if self.arg.is_some() {
			if let Arg::Opt(name) | Arg::OptWithVal(name, _) = &self.arg {
				if name == "" {
					self.result = Err("Option ohne Name ist nicht erlaubt!".to_string());
				} else {
					self.result = Err(format!("Unbekannte Option: --{}", name));
				}
			} else {
				self.result = Err("Zu viele Argumente!".to_string());
			}
			self.maybe_exit_on_error();
			return false;
		}
		
		if self.rest.is_empty() {
			return false;
		}
		
		let arg = self.rest.pop().unwrap();
		
		if self.only_args_left {
			self.arg_idx += 1;
			self.arg = Arg::Arg(arg);
			return true;
		}
		
		match &arg[..] {
			"--" => {
				if let Some(arg) = self.rest.pop() {
					self.arg_idx += 1;
					self.arg = Arg::Arg(arg);
					self.only_args_left = true;
				} else {
					return false;
				}
			},
			"--help" => {
				self.print_help();
				process::exit(0);
			},
			_ => {
				self.arg = Self::parse_arg(&arg);
				if self.arg.is_arg() {
					self.arg_idx += 1;
				}
			}
		}
		
		true
	}
	
	fn parse_arg(s: &str) -> Arg {
		if s.starts_with('-') {
			let name = s.trim_start_matches('-');
			if let Some((name, value)) = name.split_once('=') {
				Arg::OptWithVal(name.to_string(), value.to_string())
			} else {
				Arg::Opt(name.to_string())
			}
		} else {
			Arg::Arg(s.to_string())
		}
	}
	
	pub fn result(&self) -> Result<(), String> {
		self.result.clone()
	}
	
	pub fn is_opt(&mut self, long: &str, short: &str) -> bool {
		match &self.arg {
			Arg::Opt(name) => {
				if name == long || name == short {
					self.arg = Arg::None;
					return true;
				}
				false
			},
			Arg::OptWithVal(name, _) => {
				if name == long || name == short {
					self.result = Err(format!("Option erlaubt kein Argument: --{}", name));
				}
				false
			},
			_ => false
		}
	}
	
	pub fn is_opt_with_val(&mut self, long: &str, short: &str) -> Option<String> {
		match &self.arg {
			Arg::OptWithVal(name, val) => {
				if name == long || name == short {
					let val = val.clone();
					self.arg = Arg::None;
					return Some(val);
				}
				None
			},
			Arg::Opt(name) => {
				if name == long || name == short {
					if let Some(next_arg) = self.rest.pop() {
						if let Arg::Arg(val) = Self::parse_arg(&next_arg) {
							self.arg = Arg::None;
							return Some(val);
						}
					}
					self.result = Err(format!("Fehlendes Argument für Option: --{}", name));
				}
				None
			},
			_ => None
		}
	}
	
	pub fn is_arg(&mut self) -> Option<String> {
		if let Arg::Arg(value) = &self.arg {
			let val = value.clone();
			self.arg = Arg::None;
			Some(val)
		} else {
			None
		}
	}
	
	pub fn is_arg_n(&mut self, idx: i32) -> Option<String> {
		if self.arg_idx != idx {
			return None;
		}
		self.is_arg()
	}
	
	pub fn arg_idx(&self) -> i32 {
		self.arg_idx
	}
	
	//--------------------------------------------------------------------------------
	// Help/Program-Name
	//--------------------------------------------------------------------------------
		
	pub fn program(&self) -> &str {
		&self.program
	}
	
	pub fn print_help(&self) {
		println!("{}", self.help)
	}
	
	pub fn set_help(&mut self, help: &str) {
		self.help = help.to_string();
	}
	
	pub fn help(&self) -> &str {
		&self.help
	}
	
	//--------------------------------------------------------------------------------
	// Fehlermeldungen
	//--------------------------------------------------------------------------------
		
	pub fn set_error(&mut self, msg: &str) {
		self.result = Err(msg.to_string());
	}
	
	pub fn syntax_error(&self, msg: &str) {
		eprintln!("{}: {}\n\nVerwenden Sie die Option --help für weitere Hilfe!", self.program, msg);
		process::exit(1);
	}
	
	pub fn runtime_error(&self, msg: &str) {
		eprintln!("{}: {}\n", self.program, msg);
		process::exit(1);
	}
	
	pub fn set_exit_on_error(&mut self, do_exit: bool) {
		self.exit_on_error = do_exit;
	}
	
	fn maybe_exit_on_error(&self) {
		if let Err(msg) = &self.result {
			if self.exit_on_error {
				self.syntax_error(&msg);
			}
		}
	}
}

//--------------------------------------------------------------------------------
// Arg-Enum
//--------------------------------------------------------------------------------

enum Arg {
	None,
	Arg(String),
	Opt(String),
	OptWithVal(String, String),
}

impl Arg {
	fn is_arg(&self) -> bool {
		match self {
			Arg::Arg(_) => true,
			_ => false
		}
	}
	
	fn is_some(&self) -> bool {
		match self {
			Arg::None => false,
			_ => true
		}
	}
}