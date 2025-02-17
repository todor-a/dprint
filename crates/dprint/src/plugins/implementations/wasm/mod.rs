mod compile;
mod import_object;
mod instance;
mod load_instance;
mod plugin;
mod setup_wasm_plugin;

pub use compile::*;
pub use import_object::*;
use instance::*;
pub use load_instance::WasmModuleCreator;
use load_instance::*;
pub use plugin::*;
pub use setup_wasm_plugin::*;
