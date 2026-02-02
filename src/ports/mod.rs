// Port traits - define interfaces between domain and adapters

pub mod log;
pub mod output;
pub mod storage;
pub mod sync;

pub use log::LogPort;
pub use output::OutputPort;
pub use storage::StoragePort;
pub use sync::SyncPort;
