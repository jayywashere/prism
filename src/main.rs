use std::{env, path::PathBuf, process, str::FromStr};

use prism::compiler::{self, Linker, OptimizationLevel};

fn main() {
    let mut args = env::args().skip(1);

    let Some(input) = args.next() else {
        usage();
    };

    let input = PathBuf::from(input);

    let mut output = None;
    let mut linker_path = None;
    let mut optimization = OptimizationLevel::default();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" => {
                output = Some(
                    args.next()
                        .map(PathBuf::from)
                        .unwrap_or_else(|| fail("Missing output path after '-o'.")),
                );
            }

            "--linker" => {
                linker_path = Some(
                    args.next()
                        .map(PathBuf::from)
                        .unwrap_or_else(|| fail("Missing linker path after '--linker'.")),
                );
            }

            "-h" | "--help" => usage(),

            _ if arg.starts_with("-O") => {
                optimization =
                    OptimizationLevel::from_str(&arg).unwrap_or_else(|err| fail(&err.to_string()));
            }

            _ => fail(&format!("Unknown argument: '{arg}'.")),
        }
    }

    let Some(output) = output else {
        fail("Missing required output argument '-o'.");
    };

    let linker = match linker_path {
        Some(path) => Linker::new(path),
        None => Linker::discover().unwrap_or_else(|err| fail(&err.to_string())),
    };

    if let Err(err) = compiler::compile(&linker, &input, &output, optimization) {
        fail(&err.to_string());
    }
}

fn usage() -> ! {
    eprintln!(
        "Usage: prism <input.ll> -o <output> 
         [--linker <path>] [-O0|-O1|-O2|-O3|-Os|-Oz]"
    );
    process::exit(0);
}

fn fail(message: &str) -> ! {
    eprintln!("[PrismError] {message}");
    process::exit(1);
}
