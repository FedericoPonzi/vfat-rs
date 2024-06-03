use crate::TimeManagerTrait;
use alloc::sync::Arc;

/// A no-op implementation of TimeManagerTrait. It will use 0 as the current timestamp.
/// Useful if you don't care about timestamps or don't have a timer implementation yet.
#[derive(Clone, Debug, Default)]
pub struct TimeManagerNoop {}
impl TimeManagerNoop {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self {})
    }
}
impl TimeManagerTrait for TimeManagerNoop {
    fn get_current_timestamp(&self) -> u64 {
        0
    }
}

/// A Chronos base implementation of TimeManagerTrait. Needs std feature to work.
#[cfg(feature = "std")]
#[derive(Clone, Debug)]
pub struct TimeManagerChronos {}
#[cfg(feature = "std")]
impl TimeManagerChronos {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
#[cfg(feature = "std")]
impl TimeManagerTrait for TimeManagerChronos {
    fn get_current_timestamp(&self) -> u64 {
        use chrono::Utc;
        let now = Utc::now();
        let seconds_since_epoch: i64 = now.timestamp();
        // I guess it's an i64 because of underflow for dates before 1970
        seconds_since_epoch as u64
    }
}
