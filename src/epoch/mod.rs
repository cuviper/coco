//! Epoch-based memory management.
// TODO: Explain how EBR is used, with examples
// TODO: Explain pinning
// TODO: Explain pointers
// TODO: Explain garbage collection

mod atomic;
mod garbage;
mod thread;

pub use self::atomic::{Atomic, Ptr};
pub use self::garbage::Garbage;
pub use self::thread::{Pin, defer_free, flush, is_pinned, pin};

// TODO: unit tests
// TODO: sanitization
