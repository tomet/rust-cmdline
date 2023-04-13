return {
	name = "rust-cmdline",
	targets = {
		run = {
			cmd = "cargo run -- --ver",
			parser = "rust",
		},
		test = {
			cmd = "cargo test --lib",
			parser = "rust",
		}
	}
}
 