// Port traits - define interfaces between domain and adapters

pub mod storage;
pub mod sync;
pub mod output;
pub mod log;

pub use storage::StoragePort;
pub use sync::SyncPort;
pub use output::OutputPort;
pub use log::LogPort;
