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
- external compiler tools
- Rust library design and LLVM abstractions (the main point)

the main idea is for Prism to provide a convenient, reusable LLVM interface and compilation pipeline for projects that wanna work with LLVM... (literally what i meant by a friendlier abstraction layer)

## Features

- LLVM IR input (`.ll` files)
- LLVM bitcode generation (`.bc` files)
- support for configurable optimization levels (`-O0`, `-O1`, `-O2`, `-O3`, `-Os`, `-Oz`)
- native object-file generation through `llc`
- native executable linking through `gcc`, `clang`, or `cc`
- automatic discovery of LLVM tools available through `PATH`
- Rust API built around `Inkwell` and LLVM

## How It Works

Prism takes an existing LLVM IR file and passes it through a compilation pipeline.

the general flow looks like this:

```text
LLVM IR (.ll)
     |
     v
  llvm-as
     |
     v
LLVM Bitcode (.bc)
     |
     v
opt (optional)
     |
     v
Optimized Bitcode
     |
     v
    llc
     |
     v
Object File (.o)
     |
     v
gcc / clang / cc
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

optimization levels configure both LLVM's IR optimization pipeline and the code-generation optimization level where applicable.

for example, LLVM can fold constant expressions, simplify instructions, eliminate unnecessary work, and optimize function calls. a sufficiently simple program might end up returning a constant instead of performing all its original operations. JUST.. REMEMBER THE WORD `OPTIMIZE`..

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
prism <input.ll> -o <output> [optimization flag]?
```

available optimization flags:

- -O0
- -O1
- -O2
- -O3
- Os
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
- LLVM 22
- LLVM tools: `llvm-as`, `llc`, and `opt` for optimized builds
- a compatible system linker, such as `gcc`, `clang`, or `cc`

make sure the LLVM tools are available through your `PATH`

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

so, Prism currently focuses on compiling existing LLVM IR into native executables

the LLVM IR is expected to be generated beforehand, whether manually or by another tool!! although, this library can already generate the LLVM! like.. in [ARC](https://github.com/jayywashere/arc)!! where you just have to emit bytecode and whatever then call that one specific method where it bakes everything, and tada!!

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
