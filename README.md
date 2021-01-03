# argwerk

[![Documentation](https://docs.rs/argwerk/badge.svg)](https://docs.rs/argwerk)
[![Crates](https://img.shields.io/crates/v/argwerk.svg)](https://crates.io/crates/argwerk)
[![Actions Status](https://github.com/udoprog/argwerk/workflows/Rust/badge.svg)](https://github.com/udoprog/argwerk/actions)

Helper utility for parsing simple commandline arguments.

This is **not** intended to be a complete commandline parser library.
Instead this can be used as an alternative quick-and-dirty approach that can
be cheaply incorporated into a tool.

For a more complete commandline parsing library, use [clap].

## Examples

> This is available as a runnable example:
> ```sh
> cargo run --example basic
> ```

```rust
let args = argwerk::argwerk! {
    /// A simple test command.
    ///
    /// This is nice!
    "testcommand [-h]" {
        help: bool,
        file: Option<String>,
        limit: usize = 42,
    }
    /// Print this help.
    "-h" | "--help" => {
        help = true;
        print_help();
        Ok(())
    }
    /// Limit the number of things by <n>.
    "--limit" | "-l", n => {
        limit = str::parse(&n)?;
        Ok(())
    }
    /// Write to the file specified by <path>.
    "--file", path if !file.is_some() => {
        file = Some(path);
        Ok(())
    }
}?;

if args.help {
    return Ok(());
}

Ok(())
```

[clap]: https://docs.rs/clap

License: MIT/Apache-2.0
