use std::{env, path::PathBuf, process, str::FromStr};

use prism::compiler::{OptimizationLevel, Toolchain, pipeline};

fn main() {
    let mut args = env::args().skip(1);

    let Some(input) = args.next() else {
        eprintln!("[PrismError] Usage: prism <input.ll> -o <output>");
        process::exit(1);
    };

    let input = PathBuf::from(input);

    let mut output = None;
    let mut optimization = OptimizationLevel::default();
    
    while let Some(arg) = args.next() {
        if arg == "-o" {
            let Some(path) = args.next() else {
                eprintln!("[PrismError] Missing output path after '-o'.");
                process::exit(1);
            };
    
            output = Some(PathBuf::from(path));
        } else if arg.starts_with("-O") {
            optimization = match OptimizationLevel::from_str(&arg) {
                Ok(level) => level,
                Err(err) => {
                    eprintln!("[PrismError] {err}");
                    process::exit(1);
                }
            };
        } else {
            eprintln!("[PrismError] Unknown argument: '{arg}'.");
            process::exit(1);
        }
    }
    
    let Some(output) = output else {
        eprintln!("[PrismError] Usage: prism <input.ll> -o <output> [-O0|-O1|-O2|-O3|-Os|-Oz]?");
        process::exit(1);
    };

    let tools = match Toolchain::discover() {
        Ok(tools) => tools,
        Err(err) => {
            eprintln!("[PrismError] {err}");
            process::exit(1);
        }
    };

    if let Err(err) = pipeline::compile(&tools, &input, &output, optimization) {
        eprintln!("[PrismError] {err}");
        process::exit(1);
    }
}
