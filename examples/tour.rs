fn main() -> Result<(), argwerk::Error> {
    let args = argwerk::parse! {
        /// A command touring the capabilities of argwerk.
        "tour [-h]" {
            help: bool,
            file: Option<String>,
            input: Option<String>,
            limit: usize = 10,
            positional: Option<(String, Option<String>)>,
            rest: Vec<String>,
        }
        /// Prints the help.
        ///
        /// This includes:
        ///    * All the available switches.
        ///    * All the available positional arguments.
        ///    * Whatever else the developer decided to put in here! We even support wrapping comments which are overly long.
        ["-h" | "--help"] => {
            help = true;
        }
        /// Limit the number of things by <n> (default: 10).
        ["--limit" | "-l", n] => {
            limit = str::parse(&n)?;
        }
        /// Write to the file specified by <path>.
        ["--file", path] if !file.is_some() => {
            file = Some(path);
        }
        /// Read from the specified input.
        ["--input", #[option] path] => {
            input = path;
        }
        /// Takes argument at <foo> and <bar>.
        ///
        ///    * This is an indented message. The first alphanumeric character determines the indentation to use.
        [foo, #[option] bar, #[rest] args] if positional.is_none() => {
            positional = Some((foo, bar));
            rest = args;
        }
    }?;

    if args.help {
        println!("{}", args.help().format().width(100));
    }

    dbg!(args);
    Ok(())
}
