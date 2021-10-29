#[derive(Default)]
pub struct Opts {
    pub debug_scope: bool,
    pub debug_stack: bool,
    pub filename: String,
}

impl Opts {
    pub fn from_args() -> Opts {
        let args: Vec<String> = std::env::args().collect();
        let opts = Opts {
            debug_scope: args.contains(&"--scope".to_string()),
            debug_stack: args.contains(&"--stack".to_string()),
            filename: args.iter().last().unwrap().clone(),
        };
        let set_opts_count = opts.as_vec().iter().filter(|&&x| x).count();
        if args.len() != set_opts_count + 2 {
            eprintln!("Usage: {} [--scope|--stack] <source file>", args[0]);
            std::process::exit(1);
        }
        opts
    }

    fn as_vec(&self) -> Vec<bool> {
        vec![self.debug_scope, self.debug_stack]
    }
}
