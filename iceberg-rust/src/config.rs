//! Process-wide runtime configuration for the iceberg-rust crate.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Default number of concurrent manifest object-store operations (reads during scan planning,
/// deletes during maintenance).
pub const DEFAULT_CONCURRENT_MANIFEST_OPS: usize = 64;

static CONCURRENT_MANIFEST_OPS: AtomicUsize = AtomicUsize::new(DEFAULT_CONCURRENT_MANIFEST_OPS);

/// Set the maximum number of concurrent manifest object-store operations.
///
/// This bounds both in-flight object-store requests and peak manifest-decode memory. It applies
/// to manifest reads during scan planning and to object-store fan-out in maintenance operations
/// (e.g. dropping a table's files). The value is clamped to at least 1 and applies process-wide
/// to all subsequent operations.
pub fn set_concurrent_manifest_ops(limit: usize) {
    CONCURRENT_MANIFEST_OPS.store(limit.max(1), Ordering::Relaxed);
}

/// Current maximum number of concurrent manifest object-store operations.
pub fn concurrent_manifest_ops() -> usize {
    CONCURRENT_MANIFEST_OPS.load(Ordering::Relaxed)
}
