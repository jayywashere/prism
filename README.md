# Prism

> a Rust-based LLVM IR toolchain for turning `.ll` files into native executables (+ object files (`.o`), and bitcode files (`.bc`)) because apparently making an interpreter was too tuff.. ~~and slow (unless it has some kinda JIT or AOT or whatever)~~... (more suffering, indeed..)

## About

`Prism` is a `Rust`-based toolchain built around `LLVM`, designed to compile LLVM `Intermediate Representation` (IR) into native executables!!!

it currently accepts existing LLVM IR files rather than source code written in a programming language, because this is supposed to be a friendlier LLVM, not.. a fully-fledged programming language, too... congratulations to me because that means no `lexer`, `parser` and more here!!

i made this to practice working with:

- LLVM and LLVM IR
- compiler toolchain design
- intermediate representations
- native code generation
- optimization pipelines
- in-process LLVM APIs through Inkwell
- Rust library design and LLVM abstractions (the main point)

the main idea is for Prism to provide a convenient, reusable LLVM interface and compilation pipeline for projects that wanna work with LLVM... (literally what i meant by a friendlier abstraction layer)

## Features

- LLVM IR input (`.ll` files)
- LLVM bitcode generation (`.bc` files)
- configurable optimization levels (`-O0`, `-O1`, `-O2`, `-O3`, `-Os`, `-Oz`)
- native object-file generation through LLVM's in-process APIs
- native executable linking through a configurable external linker
- Rust API built around `Inkwell` and LLVM
- programmatic LLVM module construction through Prism's builder API

## How It Works

Prism takes an existing LLVM IR file and passes it through a compilation pipeline.

the general flow looks like this:

```text
LLVM IR (.ll)
     |
     v
Inkwell / LLVM API
     |
     + ------- > Parse LLVM IR
     |
     + ------- > Optimize (when needed)
     |
     + ------- > Generate native object code
     |
     v
Object File (.o)
     |
     v
Configurable external linker
 (like: gcc / clang / cc)
     |
     v
Native Executable
```

### Optimization

Prism supports the following optimization levels:

| Flag  | Description                                |
| ----- | ------------------------------------------ |
| `-O0` | no explicit LLVM IR optimization pipeline  |
| `-O1` | basic optimization                         |
| `-O2` | more extensive optimization                |
| `-O3` | aggressive optimization                    |
| `-Os` | optimization with a focus on code size     |
| `-Oz` | more aggressive optimization for code size |

optimization levels configure LLVM's optimization pipeline and native code-generation settings where applicable.

for example, LLVM can fold constant expressions, simplify instructions, eliminate unnecessary work, and optimize function calls. a sufficiently simple program might end up returning a constant instead of performing all its original operations. JUST.. REMEMBER THE WORD `OPTIMIZE`..

but truthfully.. O2, Os, and Oz are all dumbed down to the same level..

## Example

suppose you have an LLVM IR file named `input.ll`:

```llvm
define i32 @main() {
entry:
    ret i32 42
}
```

you can compile it into a native executable using:

```bash
prism input.ll -o program
```

to enable more aggressive optimization:

```bash
prism input.ll -o program -O3
```

to specify a linker too:

```bash
prism input.ll -o program -O3 --linker /usr/bin/gcc
```

then execute the resulting program:

```bash
./program
echo $? # for result
```

the exit status will be `42`.

yes, this program does absolutely nothing except return a number. we're starting small here!!

## Command-Line Usage

the general syntax is:

```text
prism <input.ll> -o <output> [optimization flag]? [--linker <path>]?
```

available optimization flags:

- -O0
- -O1
- -O2
- -O3
- -Os
- -Oz

example:

```bash
prism program.ll -o program -O2
```

the output path specifies the name of the generated executable.

Prism also generates intermediate compilation artifacts, including LLVM bitcode and an object file, during compilation.

do take note that if you don't pass an optimization flag, it uses `-O0` as default!!

## Requirements

you'll need:

- Rust and Cargo
- LLVM 22 and the LLVM libraries required by Prism's Inkwell configuration
- a compatible system linker, such as `gcc`, `clang`, or `cc`

make sure the LLVM is installed and configured so Prism can build and link against it, and ensure your chosen system linker is available

## Installation and Building

clone the repository:

```bash
git clone https://github.com/jayywashere/prism
cd prism
```

build the project:

```bash
cargo build --release
```

you can also run Prism directly through Cargo:

```bash
cargo run --release -- input.ll -o program -O2
```

if you've installed the `prism` launcher, you can invoke it directly:

```bash
prism input.ll -o program -O2
```

(you do notice there's also a `.bat` launcher.. for `Windows` users!! i just haven't tested it so.. i'll leave it up to you ^^)

## Current Scope

so, Prism currently focuses on compiling existing LLVM IR into native executables.

LLVM IR can be written manually or generated by another tool. Prism also provides a Rust API for constructing LLVM modules programmatically, so projects can generate IR through Prism rather than writing `.ll` files themselves

for example, another project can generate LLVM IR, then pass it through Prism's compilation pipeline to produce native code. that's the point of the reusable abstraction layer!!

## Related Projects

Prism is intended to fit into a larger ecosystem of programming-language projects, too

- **ARC** — a reusable stack-based bytecode virtual machine.
- **TASM** — an assembly-like language intended to target ARC and Prism.
- **Spark** — a future language project that may use Prism as part of its compilation pipeline. (and `ARC`)

each project has its own responsibilities, and Prism focuses on the LLVM side of things

## Why I Made This

tbh, i wanted to understand how compiler toolchains actually work instead of treating compilation as some mysterious process that magically turns code into executables. (sigh, the curious me)

i also wanted to experiment with LLVM and build reusable Rust abstractions around it, rather than just invoking random commands manually every time... the `Inkwell` methods give me a headache lowkey

naturally, this turned into another project where one feature leads to five more ideas and suddenly i'm thinking about optimization pipelines HAHA

goodbai

## License

See [LICENSE](LICENSE).
