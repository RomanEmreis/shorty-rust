use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU64};
use crate::token::MIN_VALUE;

pub(crate) struct Counter {
    count: Arc<AtomicU64>,
}

impl Default for Counter {
    fn default() -> Self {
        Self { 
            count: Arc::new(AtomicU64::new(MIN_VALUE))
        }
    }
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        Self { count: self.count.clone() }
    }
}

impl Counter {
    pub(crate) fn increment(&mut self) -> u64 {
        self.count.fetch_add(1, Ordering::SeqCst)
    }
}