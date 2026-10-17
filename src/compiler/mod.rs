pub mod ir;
pub use ir::load_ir;

pub mod linker;
pub use linker::Linker;

pub mod optimization;
pub use optimization::OptimizationLevel;

pub mod output;
pub use output::write_ir;

pub mod pipeline;
pub use pipeline::compile;

pub mod target;
pub use target::CodeGenerator;

pub mod tools;
pub use tools::Toolchain;
